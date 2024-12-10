use crate::{
    arith_constant,
    backend::IntCC,
    check_op_oog, check_runtime_error,
    conversion::{
        builder::OpBuilder,
        rewriter::{DeferredRewriter, Rewriter},
    },
    dora::{
        conversion::ConversionPass,
        gas::{self, compute_create2_cost, compute_initcode_cost},
        memory,
    },
    ensure_non_staticcall,
    errors::Result,
    gas_or_fail, if_here, maybe_revert_here, operands, rewrite_ctx, syscall_ctx, u256_to_64,
};
use dora_primitives::SpecId;
use dora_runtime::constants::{gas_cost, GAS_COUNTER_GLOBAL};
use dora_runtime::constants::{gas_cost::MAX_INITCODE_SIZE, CallType};
use dora_runtime::symbols;
use dora_runtime::symbols::CTX_IS_STATIC;
use dora_runtime::ExitStatusCode;
use melior::{
    dialect::{arith, cf, func, scf},
    ir::{
        attribute::{FlatSymbolRefAttribute, IntegerAttribute},
        operation::OperationRef,
        Block, Region, Value,
    },
    Context,
};
use std::mem::offset_of;

impl<'c> ConversionPass<'c> {
    pub(crate) fn create(
        context: &Context,
        op: &OperationRef<'_, '_>,
        is_create2: bool,
        spec_id: SpecId,
        limit_contract_code_size: Option<usize>,
    ) -> Result<()> {
        operands!(op, value, offset, size);
        syscall_ctx!(op, syscall_ctx);
        let rewriter = Rewriter::new_with_op(context, *op);
        ensure_non_staticcall!(op, rewriter);
        let rewriter = Rewriter::new_with_op(context, *op);
        let location = rewriter.get_insert_location();
        let uint256 = rewriter.intrinsics.i256_ty;
        let ptr_type = rewriter.ptr_ty();
        u256_to_64!(op, rewriter, offset);
        u256_to_64!(op, rewriter, size);
        let size_is_not_zero = rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, size, 0)?)?;
        if_here!(op, rewriter, size_is_not_zero, {
            if spec_id.is_enabled_in(SpecId::SHANGHAI) {
                // Limit is set as double of max contract bytecode size.
                let max_initcode_size = limit_contract_code_size
                    .map(|limit| limit.saturating_mul(2))
                    .unwrap_or(MAX_INITCODE_SIZE);
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
                gas_or_fail!(op, rewriter, gas);
            }
            let rewriter = Rewriter::new_with_op(context, *op);
            memory::resize_memory(context, op, &rewriter, syscall_ctx, offset, size)?;
        });
        let rewriter = Rewriter::new_with_op(context, *op);
        let gas = if is_create2 {
            compute_create2_cost(&rewriter, size)
        } else {
            rewriter.make(rewriter.iconst_64(gas_cost::CREATE))
        }?;
        gas_or_fail!(op, rewriter, gas);
        let rewriter = Rewriter::new_with_op(context, *op);
        let value_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, value, location)?;
        let remaining_gas = gas::get_gas_counter(&rewriter)?;

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
            offset_of!(dora_runtime::context::RuntimeResult<*mut u8>, error),
            rewriter.intrinsics.i8_ty,
        )?;
        // Check the runtime halt error
        check_runtime_error!(op, rewriter, error);
        rewrite_ctx!(context, op, rewriter, _location, NoDefer);
        let gas = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<*mut u8>, gas_used),
            rewriter.intrinsics.i64_ty,
        )?;
        gas_or_fail!(op, rewriter, gas);
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
        let rewriter = Rewriter::new_with_op(context, *op);
        let location = rewriter.get_insert_location();
        match call_type {
            CallType::Call | CallType::CallCode => {
                // Static call value is zero check
                let ctx_is_static_ptr =
                    rewriter.make(rewriter.addressof(CTX_IS_STATIC, rewriter.ptr_ty()))?;
                let ctx_is_static =
                    rewriter.make(rewriter.load(ctx_is_static_ptr, rewriter.intrinsics.i1_ty))?;
                let zero = rewriter.make(rewriter.iconst_256_from_u64(0)?)?;
                let value = op.operand(2)?;
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
        syscall_ctx!(op, syscall_ctx);
        let rewriter = Rewriter::new_with_op(context, *op);
        let location = rewriter.get_insert_location();
        let (args_offset, args_size, ret_offset, ret_size) = (
            op.operand(o_index)?,
            op.operand(o_index + 1)?,
            op.operand(o_index + 2)?,
            op.operand(o_index + 3)?,
        );

        let uint8 = rewriter.intrinsics.i8_ty;
        let uint256 = rewriter.intrinsics.i256_ty;
        let ptr_type = rewriter.ptr_ty();
        u256_to_64!(op, rewriter, gas);
        u256_to_64!(op, rewriter, args_offset);
        u256_to_64!(op, rewriter, args_size);
        u256_to_64!(op, rewriter, ret_offset);
        u256_to_64!(op, rewriter, ret_size);

        let size_is_not_zero =
            rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, args_size, 0)?)?;
        if_here!(op, rewriter, size_is_not_zero, {
            // Input memory resize
            memory::resize_memory(context, op, &rewriter, syscall_ctx, args_offset, args_size)?;
        });
        let rewriter = Rewriter::new_with_op(context, *op);
        let size_is_not_zero = rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, ret_size, 0)?)?;
        if_here!(op, rewriter, size_is_not_zero, {
            // Output memery resize
            memory::resize_memory(context, op, &rewriter, syscall_ctx, ret_offset, ret_size)?;
        });
        let rewriter = Rewriter::new_with_op(context, *op);
        let remaining_gas = gas::get_gas_counter(&rewriter)?;
        let value_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, value, location)?;
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
                gas,
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
            offset_of!(dora_runtime::context::RuntimeResult<u8>, value),
            uint8,
        )?;
        let error = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<*mut u8>, error),
            rewriter.intrinsics.i8_ty,
        )?;
        // Check the runtime halt error
        check_runtime_error!(op, rewriter, error);
        rewrite_ctx!(context, op, rewriter, _location, NoDefer);
        let gas = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<*mut u8>, gas_used),
            rewriter.intrinsics.i64_ty,
        )?;
        gas_or_fail!(op, rewriter, gas);
        rewrite_ctx!(context, op, rewriter, location);
        rewriter.create(arith::extui(result, uint256, location));

        Ok(())
    }

    pub(crate) fn creturn(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, offset, size);
        syscall_ctx!(op, syscall_ctx);
        let rewriter = Rewriter::new_with_op(context, *op);
        let location = rewriter.get_insert_location();
        let uint8 = rewriter.intrinsics.i8_ty;
        let ptr_type = rewriter.ptr_ty();

        u256_to_64!(op, rewriter, offset);
        u256_to_64!(op, rewriter, size);
        let size_is_not_zero = rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, size, 0)?)?;
        if_here!(op, rewriter, size_is_not_zero, {
            memory::resize_memory(context, op, &rewriter, syscall_ctx, offset, size)?;
        });
        let reason = rewriter.make(arith_constant!(
            rewriter,
            context,
            uint8,
            ExitStatusCode::Return.to_u8().into(),
            location
        ))?;
        rewrite_ctx!(context, op, rewriter, location);
        let gas_counter = gas::get_gas_counter(&rewriter)?;
        rewriter.create(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::WRITE_RESULT),
            &[syscall_ctx.into(), offset, size, gas_counter, reason],
            &[ptr_type],
            location,
        ));
        rewriter.create(func::r#return(&[reason], location));

        Ok(())
    }
}
