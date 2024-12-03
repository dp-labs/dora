use crate::{
    arith_constant,
    backend::IntCC,
    check_op_oog, check_runtime_error,
    conversion::{
        builder::OpBuilder,
        rewriter::{DeferredRewriter, Rewriter},
    },
    dora::{conversion::ConversionPass, gas, memory},
    errors::Result,
    maybe_revert_here, operands, rewrite_ctx, syscall_ctx, u256_to_64,
};
use dora_runtime::constants::CallType;
use dora_runtime::symbols;
use dora_runtime::symbols::CTX_IS_STATIC;
use dora_runtime::ExitStatusCode;
use melior::{
    dialect::{
        arith, cf, func,
        llvm::{self, LoadStoreOptions},
    },
    ir::{
        attribute::{FlatSymbolRefAttribute, IntegerAttribute},
        operation::OperationRef,
        Value,
    },
    Context,
};
use std::mem::offset_of;

impl<'c> ConversionPass<'c> {
    pub(crate) fn create(
        context: &Context,
        op: &OperationRef<'_, '_>,
        is_create2: bool,
    ) -> Result<()> {
        operands!(op, value, offset, size);
        syscall_ctx!(op, syscall_ctx);
        let rewriter = Rewriter::new_with_op(context, *op);
        let location = rewriter.get_insert_location();
        let uint256 = rewriter.intrinsics.i256_ty;
        let ptr_type = rewriter.ptr_ty();
        u256_to_64!(op, rewriter, offset);
        u256_to_64!(op, rewriter, size);
        // required_size = offset + size
        let required_memory_size = rewriter.make(arith::addi(offset, size, location))?;
        memory::resize_memory(context, op, &rewriter, syscall_ctx, required_memory_size)?;
        let rewriter = Rewriter::new_with_op(context, *op);
        let value_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, value, location)?;
        let gas_ptr = gas::create_var_with_gas_counter(context, &rewriter, location)?;

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
                    gas_ptr,
                    salt_ptr,
                ],
                &[ptr_type],
                location,
            ))?
        } else {
            rewriter.make(func::call(
                context,
                FlatSymbolRefAttribute::new(context, symbols::CREATE),
                &[syscall_ctx.into(), size, offset, value_ptr, gas_ptr],
                &[ptr_type],
                location,
            ))?
        };
        let error = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::Result<*mut u8>, error),
            rewriter.intrinsics.i8_ty,
        )?;
        // Check the runtime halt error
        check_runtime_error!(op, rewriter, error);
        rewrite_ctx!(context, op, rewriter, location);
        // Deferred rewriter is need to be the op generation scope.
        rewriter.make(llvm::load(
            context,
            value_ptr,
            uint256,
            location,
            LoadStoreOptions::default(),
        ))?;

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
                Self::intern_call(context, op, value, 3)?;
            }
            CallType::StaticCall | CallType::DelegateCall => {
                Self::intern_call(
                    context,
                    op,
                    rewriter.make(rewriter.iconst_256_from_u64(0)?)?,
                    2,
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
        let uint64 = rewriter.intrinsics.i64_ty;
        let uint256 = rewriter.intrinsics.i256_ty;
        let ptr_type = rewriter.ptr_ty();
        let gas = rewriter.make(arith::trunci(gas, uint64, location))?;
        u256_to_64!(op, rewriter, args_offset);
        u256_to_64!(op, rewriter, args_size);
        u256_to_64!(op, rewriter, ret_offset);
        u256_to_64!(op, rewriter, ret_size);
        let req_arg_mem_size = rewriter.make(arith::addi(args_offset, args_size, location))?;
        let req_ret_mem_size = rewriter.make(arith::addi(ret_offset, ret_size, location))?;
        let req_mem_size =
            rewriter.make(arith::maxui(req_arg_mem_size, req_ret_mem_size, location))?;
        memory::resize_memory(context, op, &rewriter, syscall_ctx, req_mem_size)?;
        let rewriter = Rewriter::new_with_op(context, *op);
        let available_gas = gas::get_gas_counter(&rewriter)?;
        let value_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, value, location)?;
        let address_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, address, location)?;

        let gas_ptr = gas::create_gas_var(context, &rewriter, location)?;
        let call_type_value = rewriter.make(arith_constant!(
            rewriter,
            context,
            uint8,
            CallType::Call as u8 as i64,
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
                available_gas,
                gas_ptr,
                call_type_value,
            ],
            &[ptr_type],
            location,
        ))?;
        let result = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::Result<u8>, value),
            uint8,
        )?;
        let error = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::Result<*mut u8>, error),
            rewriter.intrinsics.i8_ty,
        )?;
        // Check the runtime halt error
        check_runtime_error!(op, rewriter, error);
        rewrite_ctx!(context, op, rewriter, location);
        rewriter.create(llvm::load(
            context,
            gas_ptr,
            uint64,
            location,
            LoadStoreOptions::default(),
        ));
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
        let required_size = rewriter.make(arith::addi(size, offset, location))?;
        let gas_counter = gas::get_gas_counter(&rewriter)?;
        memory::resize_memory(context, op, &rewriter, syscall_ctx, required_size)?;
        let reason = rewriter.make(arith_constant!(
            rewriter,
            context,
            uint8,
            ExitStatusCode::Return.to_u8().into(),
            location
        ))?;
        rewrite_ctx!(context, op, rewriter, location);
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
