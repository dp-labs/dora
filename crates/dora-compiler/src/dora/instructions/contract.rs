use crate::{
    arith_constant,
    backend::IntCC,
    block_argument, check_runtime_error,
    conversion::{builder::OpBuilder, rewriter::Rewriter},
    create_var,
    dora::{
        conversion::ConversionPass,
        gas::{compute_eofcreate_create2_cost, compute_initcode_cost},
        memory,
    },
    ensure_non_staticcall,
    errors::Result,
    gas_or_fail, if_here, maybe_revert_here, operands, rewrite_ctx, u256_to_u64,
};
use dora_primitives::SpecId;
use dora_runtime::ExitStatusCode;
use dora_runtime::constants::CallType;
use dora_runtime::constants::{ExtCallType, gas_cost};
use dora_runtime::symbols;
use melior::{
    Context,
    dialect::{
        arith, func,
        llvm::{self, LoadStoreOptions},
        ods, scf,
    },
    ir::{
        Block, Region, Value,
        attribute::{FlatSymbolRefAttribute, IntegerAttribute, TypeAttribute},
        operation::OperationRef,
    },
};
use num_bigint::BigUint;
use std::mem::offset_of;

impl ConversionPass<'_> {
    pub(crate) fn eofcreate(
        context: &Context,
        op: &OperationRef<'_, '_>,
        limit_contract_code_size: usize,
    ) -> Result<()> {
        operands!(
            op,
            initcontainer_index,
            value,
            salt,
            input_offset,
            input_size
        );
        block_argument!(op, syscall_ctx, gas_counter_ptr);

        rewrite_ctx!(context, op, rewriter, NoDefer);
        ensure_non_staticcall!(op, rewriter, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location, NoDefer);

        let uint8 = rewriter.i8_ty();
        let uint64 = rewriter.i64_ty();
        let uint256 = rewriter.i256_ty();
        let ptr_type = rewriter.ptr_ty();

        // Compare the input size with the limit
        u256_to_u64!(op, rewriter, input_size);
        let size_is_not_zero =
            rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, input_size, 0)?)?;
        if_here!(op, rewriter, size_is_not_zero, {
            let revert_flag = rewriter.make(rewriter.icmp_imm(
                IntCC::UnsignedGreaterThan,
                input_size,
                limit_contract_code_size as i64,
            )?)?;
            maybe_revert_here!(
                op,
                rewriter,
                revert_flag,
                ExitStatusCode::CreateContractSizeLimit
            );

            // Deduct gas cost for possible memory expansion
            rewrite_ctx!(context, op, rewriter, NoDefer);
            u256_to_u64!(op, rewriter, input_offset);
            memory::resize_memory(
                context,
                op,
                &rewriter,
                syscall_ctx,
                gas_counter_ptr,
                input_offset,
                input_size,
            )?;
        });

        // Calculate the gas cost and check the gas limit
        rewrite_ctx!(context, op, rewriter, NoDefer);
        let gas = compute_eofcreate_create2_cost(&rewriter, input_size)?;
        gas_or_fail!(op, rewriter, gas, gas_counter_ptr);

        rewrite_ctx!(context, op, rewriter, NoDefer);
        let offset = rewriter.make(arith::trunci(input_offset, uint64, location))?;
        let value_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, value, location)?;
        let remaining_gas = rewriter.make(rewriter.load(gas_counter_ptr, uint64))?;
        let salt_ptr = memory::allocate_u256_and_assign_value(context, &rewriter, salt, location)?;

        let result_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::EOFCREATE),
            &[
                syscall_ctx.into(),
                initcontainer_index,
                input_size,
                offset,
                value_ptr,
                remaining_gas,
                salt_ptr,
            ],
            &[ptr_type],
            location,
        ))?;

        // Check the runtime halt error
        let error = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<*mut u8>, error),
            uint8,
        )?;
        check_runtime_error!(op, rewriter, error);

        // Check gas used
        rewrite_ctx!(context, op, rewriter, NoDefer);
        let gas = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<*mut u8>, gas_used),
            uint64,
        )?;
        gas_or_fail!(op, rewriter, gas, gas_counter_ptr);

        rewrite_ctx!(context, op, rewriter);
        // Deferred rewriter is need to be the op generation scope.
        rewriter.make(rewriter.load(value_ptr, uint256))?;

        Ok(())
    }

    pub(crate) fn returncontract(
        context: &Context,
        op: &OperationRef<'_, '_>,
        limit_contract_code_size: usize,
    ) -> Result<()> {
        operands!(op, deploy_container_index, aux_data_offset, aux_data_size);
        block_argument!(op, syscall_ctx, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, location, NoDefer);

        let uint8 = rewriter.i8_ty();
        let uint64 = rewriter.i64_ty();

        u256_to_u64!(op, rewriter, aux_data_size);
        let size_is_not_zero =
            rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, aux_data_size, 0)?)?;
        if_here!(op, rewriter, size_is_not_zero, {
            // Deduct gas cost for possible memory expansion
            u256_to_u64!(op, rewriter, aux_data_offset);
            memory::resize_memory(
                context,
                op,
                &rewriter,
                syscall_ctx,
                gas_counter_ptr,
                aux_data_offset,
                aux_data_size,
            )?;
        });

        let reason = rewriter.make(arith_constant!(
            rewriter,
            context,
            uint8,
            ExitStatusCode::Return.to_u8().into(),
            location
        ))?;

        rewrite_ctx!(context, op, rewriter, NoDefer);
        let offset = rewriter.make(arith::trunci(aux_data_offset, uint64, location))?;
        let max_code_size = rewriter.make(rewriter.index(limit_contract_code_size))?;
        let gas_counter = rewriter.make(rewriter.load(gas_counter_ptr, uint64))?;

        let result_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::RETURNCONTRACT),
            &[
                syscall_ctx.into(),
                deploy_container_index,
                aux_data_size,
                offset,
                max_code_size,
                gas_counter,
                reason,
            ],
            &[],
            location,
        ))?;
        let error = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<*mut u8>, error),
            uint8,
        )?;
        // Check the runtime halt error
        check_runtime_error!(op, rewriter, error);

        Ok(())
    }

    pub(crate) fn create(
        context: &Context,
        op: &OperationRef<'_, '_>,
        is_create2: bool,
        spec_id: SpecId,
        limit_contract_code_size: usize,
    ) -> Result<()> {
        operands!(op, value, offset, size);
        block_argument!(op, syscall_ctx, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, NoDefer);
        ensure_non_staticcall!(op, rewriter, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location, NoDefer);

        let uint8 = rewriter.i8_ty();
        let uint64 = rewriter.i64_ty();
        let uint256 = rewriter.i256_ty();
        let ptr_type = rewriter.ptr_ty();

        u256_to_u64!(op, rewriter, size);
        let size_is_not_zero = rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, size, 0)?)?;
        if_here!(op, rewriter, size_is_not_zero, {
            if spec_id.is_enabled_in(SpecId::SHANGHAI) {
                // Limit is set as double of max contract bytecode size.
                let max_initcode_size = limit_contract_code_size.saturating_mul(2);
                let revert_flag = rewriter.make(rewriter.icmp_imm(
                    IntCC::UnsignedGreaterThan,
                    size,
                    max_initcode_size as i64,
                )?)?;
                maybe_revert_here!(
                    op,
                    rewriter,
                    revert_flag,
                    ExitStatusCode::CreateInitCodeSizeLimit
                );
                let rewriter = Rewriter::new_with_op(context, *op);
                let gas = compute_initcode_cost(&rewriter, size)?;
                gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
            }
            // Deduct gas cost for possible memory expansion
            let rewriter = Rewriter::new_with_op(context, *op);
            u256_to_u64!(op, rewriter, offset);
            memory::resize_memory(
                context,
                op,
                &rewriter,
                syscall_ctx,
                gas_counter_ptr,
                offset,
                size,
            )?;
        });
        let rewriter = Rewriter::new_with_op(context, *op);
        let gas = if is_create2 {
            compute_eofcreate_create2_cost(&rewriter, size)
        } else {
            rewriter.make(rewriter.iconst_64(gas_cost::CREATE))
        }?;
        gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
        let rewriter = Rewriter::new_with_op(context, *op);
        let value_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, value, location)?;
        let remaining_gas = rewriter.make(rewriter.load(gas_counter_ptr, uint64))?;
        let offset = rewriter.make(arith::trunci(offset, uint64, location))?;
        let result_ptr = if is_create2 {
            let salt: Value<'_, '_> = op.operand(3)?;
            let salt_ptr =
                memory::allocate_u256_and_assign_value(context, &rewriter, salt, location)?;
            rewriter.make(func::call(
                context,
                FlatSymbolRefAttribute::new(context, symbols::CREATE2),
                &[
                    syscall_ctx.into(),
                    size,
                    offset,
                    value_ptr,
                    remaining_gas,
                    salt_ptr,
                ],
                &[ptr_type],
                location,
            ))?
        } else {
            rewriter.make(func::call(
                context,
                FlatSymbolRefAttribute::new(context, symbols::CREATE),
                &[syscall_ctx.into(), size, offset, value_ptr, remaining_gas],
                &[ptr_type],
                location,
            ))?
        };
        let error = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<()>, error),
            uint8,
        )?;
        // Check the runtime halt error
        check_runtime_error!(op, rewriter, error);
        rewrite_ctx!(context, op, rewriter, _location, NoDefer);
        let gas = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<()>, gas_used),
            uint64,
        )?;
        gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, _location);
        // Deferred rewriter is need to be the op generation scope.
        rewriter.make(rewriter.load(value_ptr, uint256))?;

        Ok(())
    }

    pub(crate) fn call(
        context: &Context,
        op: &OperationRef<'_, '_>,
        call_type: CallType,
    ) -> Result<()> {
        rewrite_ctx!(context, op, rewriter, location, NoDefer);

        let uint8 = rewriter.i8_ty();

        match call_type {
            CallType::Call | CallType::Callcode => {
                let value = op.operand(2)?;
                if call_type == CallType::Call {
                    block_argument!(op, syscall_ctx);
                    // Static call value is zero check
                    let ctx_is_static_u8 = rewriter.make(func::call(
                        context,
                        FlatSymbolRefAttribute::new(context, symbols::CTX_IS_STATIC),
                        &[syscall_ctx.into()],
                        &[uint8],
                        location,
                    ))?;
                    let ctx_is_static = rewriter.make(rewriter.icmp_imm(
                        IntCC::NotEqual,
                        ctx_is_static_u8,
                        0,
                    )?)?;
                    let zero = rewriter.make(rewriter.iconst_256_from_u64(0)?)?;
                    let value_is_not_zero =
                        rewriter.make(rewriter.icmp(IntCC::NotEqual, value, zero))?;
                    let revert_flag =
                        rewriter.make(arith::andi(ctx_is_static, value_is_not_zero, location))?;

                    maybe_revert_here!(
                        op,
                        rewriter,
                        revert_flag,
                        ExitStatusCode::CallNotAllowedInsideStatic
                    );
                }
                Self::intern_call(context, op, value, 3, call_type)?;
            }
            CallType::Staticcall | CallType::Delegatecall => {
                Self::intern_call(
                    context,
                    op,
                    rewriter.make(rewriter.iconst_256_from_u64(0)?)?,
                    2,
                    call_type,
                )?;
            }
        };
        Ok(())
    }

    fn intern_call(
        context: &Context,
        op: &OperationRef<'_, '_>,
        value: Value<'_, '_>,
        o_index: usize,
        call_type: CallType,
    ) -> Result<()> {
        operands!(op, gas, address);
        block_argument!(op, syscall_ctx, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, location, NoDefer);
        let (args_offset, args_size, ret_offset, ret_size) = (
            op.operand(o_index)?,
            op.operand(o_index + 1)?,
            op.operand(o_index + 2)?,
            op.operand(o_index + 3)?,
        );

        let uint8 = rewriter.i8_ty();
        let uint64 = rewriter.i64_ty();
        let uint256 = rewriter.i256_ty();
        let ptr_type = rewriter.ptr_ty();

        u256_to_u64!(op, rewriter, args_size);
        let size_is_not_zero =
            rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, args_size, 0)?)?;
        if_here!(op, rewriter, size_is_not_zero, {
            // Input memory resize
            u256_to_u64!(op, rewriter, args_offset);
            memory::resize_memory(
                context,
                op,
                &rewriter,
                syscall_ctx,
                gas_counter_ptr,
                args_offset,
                args_size,
            )?;
        });
        let rewriter = Rewriter::new_with_op(context, *op);
        u256_to_u64!(op, rewriter, ret_size);
        let size_is_not_zero = rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, ret_size, 0)?)?;
        if_here!(op, rewriter, size_is_not_zero, {
            // Output memery resize
            u256_to_u64!(op, rewriter, ret_offset);
            memory::resize_memory(
                context,
                op,
                &rewriter,
                syscall_ctx,
                gas_counter_ptr,
                ret_offset,
                ret_size,
            )?;
        });
        let args_offset = rewriter.make(arith::trunci(args_offset, uint64, location))?;
        let ret_offset = rewriter.make(arith::trunci(ret_offset, uint64, location))?;
        let rewriter = Rewriter::new_with_op(context, *op);
        let remaining_gas = rewriter.make(rewriter.load(gas_counter_ptr, uint64))?;
        let value_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, value, location)?;
        let gas_ptr = memory::allocate_u256_and_assign_value(context, &rewriter, gas, location)?;
        let address_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, address, location)?;
        let call_type_value = rewriter.make(arith_constant!(
            rewriter,
            context,
            uint8,
            call_type as u8 as i64,
            location
        ))?;
        let result_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::CALL),
            &[
                syscall_ctx.into(),
                gas_ptr,
                address_ptr,
                value_ptr,
                args_offset,
                args_size,
                ret_offset,
                ret_size,
                remaining_gas,
                call_type_value,
            ],
            &[ptr_type],
            location,
        ))?;
        let result = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<u64>, value),
            uint64,
        )?;
        let error = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<u64>, error),
            uint8,
        )?;
        // Check the runtime halt error
        check_runtime_error!(op, rewriter, error);
        rewrite_ctx!(context, op, rewriter, _location, NoDefer);
        let gas = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<u64>, gas_used),
            uint64,
        )?;
        gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, location);
        rewriter.create(arith::extui(result, uint256, location));

        Ok(())
    }

    pub(crate) fn creturn(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, offset, size);
        block_argument!(op, syscall_ctx, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, location, NoDefer);

        let uint8 = rewriter.i8_ty();
        let uint64 = rewriter.i64_ty();

        u256_to_u64!(op, rewriter, size);
        let size_is_not_zero = rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, size, 0)?)?;
        if_here!(op, rewriter, size_is_not_zero, {
            u256_to_u64!(op, rewriter, offset);
            memory::resize_memory(
                context,
                op,
                &rewriter,
                syscall_ctx,
                gas_counter_ptr,
                offset,
                size,
            )?;
        });
        let reason = rewriter.make(arith_constant!(
            rewriter,
            context,
            uint8,
            ExitStatusCode::Return.to_u8().into(),
            location
        ))?;
        rewrite_ctx!(context, op, rewriter, location);
        let offset = rewriter.make(arith::trunci(offset, uint64, location))?;
        let gas_counter = rewriter.make(rewriter.load(gas_counter_ptr, uint64))?;
        rewriter.create(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::WRITE_RESULT),
            &[syscall_ctx.into(), offset, size, gas_counter, reason],
            &[],
            location,
        ));
        rewriter.create(func::r#return(&[reason], location));

        Ok(())
    }

    pub(crate) fn extcall(
        context: &Context,
        op: &OperationRef<'_, '_>,
        call_type: ExtCallType,
    ) -> Result<()> {
        rewrite_ctx!(context, op, rewriter, location, NoDefer);

        let uint8 = rewriter.i8_ty();

        match call_type {
            ExtCallType::Call => {
                let value = op.operand(3)?;

                block_argument!(op, syscall_ctx);
                // Static call value is zero check
                let ctx_is_static_u8 = rewriter.make(func::call(
                    context,
                    FlatSymbolRefAttribute::new(context, symbols::CTX_IS_STATIC),
                    &[syscall_ctx.into()],
                    &[uint8],
                    location,
                ))?;
                let ctx_is_static =
                    rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, ctx_is_static_u8, 0)?)?;
                let zero = rewriter.make(rewriter.iconst_256_from_u64(0)?)?;
                let value_is_not_zero =
                    rewriter.make(rewriter.icmp(IntCC::NotEqual, value, zero))?;
                let revert_flag =
                    rewriter.make(arith::andi(ctx_is_static, value_is_not_zero, location))?;

                maybe_revert_here!(
                    op,
                    rewriter,
                    revert_flag,
                    ExitStatusCode::CallNotAllowedInsideStatic
                );

                Self::ext_intern_call(context, op, value, call_type)?;
            }
            ExtCallType::Staticcall | ExtCallType::Delegatecall => {
                Self::ext_intern_call(
                    context,
                    op,
                    rewriter.make(rewriter.iconst_256_from_u64(0)?)?,
                    call_type,
                )?;
            }
        };
        Ok(())
    }

    fn ext_intern_call(
        context: &Context,
        op: &OperationRef<'_, '_>,
        value: Value<'_, '_>,
        call_type: ExtCallType,
    ) -> Result<()> {
        block_argument!(op, syscall_ctx, gas_counter_ptr);
        operands!(op, target_address, input_offset, input_size);
        rewrite_ctx!(context, op, rewriter, location, NoDefer);

        let uint8 = rewriter.i8_ty();
        let uint64 = rewriter.i64_ty();
        let uint256 = rewriter.i256_ty();
        let ptr_type = rewriter.ptr_ty();

        u256_to_u64!(op, rewriter, input_size);
        u256_to_u64!(op, rewriter, input_offset);
        let size_is_not_zero =
            rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, input_size, 0)?)?;
        if_here!(op, rewriter, size_is_not_zero, {
            // Input memory resize
            memory::resize_memory(
                context,
                op,
                &rewriter,
                syscall_ctx,
                gas_counter_ptr,
                input_offset,
                input_size,
            )?;
        });

        rewrite_ctx!(context, op, rewriter, NoDefer);
        let remaining_gas = rewriter.make(rewriter.load(gas_counter_ptr, uint64))?;
        let value_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, value, location)?;
        let target_address_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, target_address, location)?;
        let call_type_value = rewriter.make(arith_constant!(
            rewriter,
            context,
            uint8,
            call_type as u8 as i64,
            location
        ))?;
        // TODO : Check `target_address` has any high 12 bytes in 32 byte value itself,
        // since CALLCODE is deprecated and only 20-byte address is allowed.
        let result_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::EXTCALL),
            &[
                syscall_ctx.into(),
                target_address_ptr,
                value_ptr,
                input_offset,
                input_size,
                remaining_gas,
                call_type_value,
            ],
            &[ptr_type],
            location,
        ))?;
        let result = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<u64>, value),
            uint64,
        )?;
        let error = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<u64>, error),
            uint8,
        )?;
        // Check the runtime halt error
        check_runtime_error!(op, rewriter, error);
        rewrite_ctx!(context, op, rewriter, _location, NoDefer);
        let gas = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<u64>, gas_used),
            uint64,
        )?;
        gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, location);
        rewriter.create(arith::extui(result, uint256, location));

        Ok(())
    }

    pub(crate) fn returndataload(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        block_argument!(op, syscall_ctx);
        operands!(op, offset);
        rewrite_ctx!(context, op, rewriter, location);

        let uint1 = rewriter.i1_ty();
        let uint8 = rewriter.i8_ty();
        let uint64 = rewriter.i64_ty();
        let uint256 = rewriter.i256_ty();
        let ptr_type = rewriter.ptr_ty();

        let returndata_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::RETURNDATA),
            &[syscall_ctx.into()],
            &[ptr_type],
            location,
        ))?;
        let returndata_size = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::RETURNDATA_SIZE),
            &[syscall_ctx.into()],
            &[uint64],
            location,
        ))?;
        // convert `offset` from u16 to u256
        let offset = rewriter.make(arith::extui(offset, uint256, location))?;
        // convert `returndata_size` from u64 to u256
        let returndata_size = rewriter.make(arith::extui(returndata_size, uint256, location))?;
        // Define the maximum slice width (32 bytes)
        let max_slice_width = rewriter.make(rewriter.iconst_256_from_u64(32)?)?;
        // Compare offset with `returndata_size`
        let offset_cmpi = rewriter.make(arith::cmpi(
            context,
            arith::CmpiPredicate::Ult,
            offset,
            returndata_size,
            location,
        ))?;
        let zero = rewriter.make(rewriter.iconst_256(BigUint::from(0_u8))?)?;

        rewriter.make(scf::r#if(
            offset_cmpi,
            &[uint256],
            {
                let region = Region::new();
                let offset_ok_block = region.append_block(Block::new(&[]));
                let rewriter = Rewriter::new_with_block(context, offset_ok_block);
                // A stack slot is a u256 ptr
                let stack_slot_ptr = create_var!(rewriter, context, location);
                rewriter.create(llvm::store(
                    context,
                    zero,
                    stack_slot_ptr,
                    location,
                    LoadStoreOptions::new(),
                ));
                // Calculate returndata pointer at offset
                let returndata_ptr_at_offset = rewriter.make(llvm::get_element_ptr_dynamic(
                    context,
                    returndata_ptr,
                    &[offset],
                    uint8,
                    ptr_type,
                    location,
                ))?;
                // Calculate length of slice (min(returndata_size - offset, 32))
                let len_sub = rewriter.make(arith::subi(returndata_size, offset, location))?;
                let len_min = rewriter.make(arith::minui(len_sub, max_slice_width, location))?;

                // Copy returndata[offset..offset + len] to the stack slot
                rewriter.create(
                    ods::llvm::intr_memcpy(
                        context,
                        stack_slot_ptr,
                        returndata_ptr_at_offset,
                        len_min,
                        IntegerAttribute::new(uint1, 0),
                        location,
                    )
                    .into(),
                );
                let mut value = rewriter.make(rewriter.load(stack_slot_ptr, uint256))?;
                if cfg!(target_endian = "little") {
                    // convert it to big endian
                    value = rewriter.make(llvm::intr_bswap(value, uint256, location))?;
                }
                rewriter.create(scf::r#yield(&[value], location));
                region
            },
            {
                let region = Region::new();
                let offset_bad_block = region.append_block(Block::new(&[]));
                let builder = OpBuilder::new_with_block(context, offset_bad_block);
                builder.create(scf::r#yield(&[zero], location));
                region
            },
            location,
        ))?;
        Ok(())
    }
}
