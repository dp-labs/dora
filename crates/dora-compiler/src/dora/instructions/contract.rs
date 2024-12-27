use crate::{
    arith_constant,
    backend::IntCC,
    block_argument, check_runtime_error,
    conversion::rewriter::Rewriter,
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
use dora_runtime::constants::gas_cost;
use dora_runtime::constants::CallType;
use dora_runtime::symbols;
use dora_runtime::ExitStatusCode;
use melior::{
    dialect::{arith, func},
    ir::{attribute::FlatSymbolRefAttribute, operation::OperationRef, Block, Value},
    Context,
};
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

        let uint64 = rewriter.intrinsics.i64_ty;
        let uint256 = rewriter.intrinsics.i256_ty;
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
            rewriter.intrinsics.i8_ty,
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

        let uint8 = rewriter.uint8_ty();
        let uint64 = rewriter.uint64_ty();

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
            rewriter.intrinsics.i8_ty,
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
        let rewriter = Rewriter::new_with_op(context, *op);
        ensure_non_staticcall!(op, rewriter, syscall_ctx);
        let rewriter = Rewriter::new_with_op(context, *op);
        let location = rewriter.get_insert_location();
        let uint256 = rewriter.intrinsics.i256_ty;
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
        let remaining_gas =
            rewriter.make(rewriter.load(gas_counter_ptr, rewriter.intrinsics.i64_ty))?;
        let offset = rewriter.make(arith::trunci(offset, rewriter.intrinsics.i64_ty, location))?;
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
            rewriter.intrinsics.i8_ty,
        )?;
        // Check the runtime halt error
        check_runtime_error!(op, rewriter, error);
        rewrite_ctx!(context, op, rewriter, _location, NoDefer);
        let gas = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<()>, gas_used),
            rewriter.intrinsics.i64_ty,
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
                        &[rewriter.intrinsics.i8_ty],
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

        let uint8 = rewriter.intrinsics.i8_ty;
        let uint64 = rewriter.intrinsics.i64_ty;
        let uint256 = rewriter.intrinsics.i256_ty;
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
        let args_offset = rewriter.make(arith::trunci(
            args_offset,
            rewriter.intrinsics.i64_ty,
            location,
        ))?;
        let ret_offset = rewriter.make(arith::trunci(
            ret_offset,
            rewriter.intrinsics.i64_ty,
            location,
        ))?;
        let rewriter = Rewriter::new_with_op(context, *op);
        let remaining_gas =
            rewriter.make(rewriter.load(gas_counter_ptr, rewriter.intrinsics.i64_ty))?;
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
            rewriter.intrinsics.i8_ty,
        )?;
        // Check the runtime halt error
        check_runtime_error!(op, rewriter, error);
        rewrite_ctx!(context, op, rewriter, _location, NoDefer);
        let gas = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<u64>, gas_used),
            rewriter.intrinsics.i64_ty,
        )?;
        gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, location);
        rewriter.create(arith::extui(result, uint256, location));

        Ok(())
    }

    pub(crate) fn creturn(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, offset, size);
        block_argument!(op, syscall_ctx, gas_counter_ptr);
        let rewriter = Rewriter::new_with_op(context, *op);
        let location = rewriter.get_insert_location();
        let uint8 = rewriter.intrinsics.i8_ty;
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
        let offset = rewriter.make(arith::trunci(offset, rewriter.intrinsics.i64_ty, location))?;
        let gas_counter =
            rewriter.make(rewriter.load(gas_counter_ptr, rewriter.intrinsics.i64_ty))?;
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
}
