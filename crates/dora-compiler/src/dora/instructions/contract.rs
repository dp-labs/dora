use crate::{
    arith_constant,
    backend::IntCC,
    conversion::{
        builder::OpBuilder,
        rewriter::{DeferredRewriter, Replacer, Rewriter},
    },
    dora::{conversion::ConversionPass, gas, memory},
    errors::Result,
    operands, rewrite_ctx, syscall_ctx,
};
use dora_runtime::constants::CallType;
use dora_runtime::symbols;
use dora_runtime::symbols::CTX_IS_STATIC;
use dora_runtime::ExitStatusCode;
use melior::{
    dialect::{
        arith::{self, CmpiPredicate},
        cf, func,
        llvm::{self, LoadStoreOptions},
    },
    ir::{
        attribute::{FlatSymbolRefAttribute, IntegerAttribute},
        operation::OperationRef,
        Value,
    },
    Context,
};

impl<'c> ConversionPass<'c> {
    pub(crate) fn create(
        context: &Context,
        op: &OperationRef<'_, '_>,
        is_create2: bool,
    ) -> Result<()> {
        operands!(op, value, offset, size);
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint8 = rewriter.intrinsics.i8_ty;
        let uint64 = rewriter.intrinsics.i64_ty;
        let uint256 = rewriter.intrinsics.i256_ty;
        let offset = rewriter.make(arith::trunci(offset, uint64, location))?;
        let size = rewriter.make(arith::trunci(size, uint64, location))?;

        // required_size = offset + size
        let required_memory_size = rewriter.make(arith::addi(offset, size, location))?;
        memory::resize_memory(
            required_memory_size,
            context,
            &rewriter,
            syscall_ctx,
            location,
        )?;

        let value_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, value, location)?;
        let gas_ptr = gas::create_var_with_gas_counter(context, &rewriter, location)?;

        let result = if is_create2 {
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
                &[uint8],
                location,
            ))?
        } else {
            rewriter.make(func::call(
                context,
                FlatSymbolRefAttribute::new(context, symbols::CREATE),
                &[syscall_ctx.into(), size, offset, value_ptr, gas_ptr],
                &[uint8],
                location,
            ))?
        };

        let zero = rewriter.make(rewriter.iconst_8(0))?;
        let flag = rewriter.make(arith::cmpi(
            context,
            CmpiPredicate::Eq,
            zero,
            result,
            location,
        ))?;
        // Deferred rewriter is need to be the op generation scope.
        rewriter.make(llvm::load(
            context,
            value_ptr,
            uint256,
            location,
            LoadStoreOptions::default(),
        ))?;

        if let Some(block) = op.block() {
            if let Some(region) = block.parent_region() {
                if let Some(setup_block) = region.first_block() {
                    if let Some(revert_block) = setup_block.next_in_region() {
                        if let Some(insert_point) = rewriter.get_insert_point() {
                            let next_block = rewriter.split_block(block, Some(insert_point))?;
                            let builder = OpBuilder::new_with_block(context, block);
                            builder.create(cf::cond_br(
                                context,
                                flag,
                                &next_block,
                                &revert_block,
                                &[],
                                &[],
                                location,
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn call(
        context: &Context,
        op: &OperationRef<'_, '_>,
        call_type: CallType,
    ) -> Result<()> {
        let rewriter = Rewriter::new_with_op(context, *op);
        let location = rewriter.get_insert_location();
        let block = op.block().unwrap();
        match call_type {
            CallType::Call | CallType::CallCode => {
                // Static call value is zero check
                if let Some(region) = block.parent_region() {
                    if let Some(setup_block) = region.first_block() {
                        if let Some(revert_block) = setup_block.next_in_region() {
                            if let Some(insert_point) = rewriter.get_insert_point() {
                                let next_block = rewriter.split_block(block, Some(insert_point))?;
                                let builder = OpBuilder::new_with_block(context, block);
                                let ctx_is_static_ptr = builder
                                    .make(builder.addressof(CTX_IS_STATIC, builder.ptr_ty()))?;
                                let ctx_is_static = builder.make(
                                    builder.load(ctx_is_static_ptr, builder.intrinsics.i1_ty),
                                )?;
                                let zero = builder.make(builder.iconst_256_from_u64(0)?)?;
                                let value = op.operand(2)?;
                                let value_is_not_zero =
                                    builder.make(builder.icmp(IntCC::NotEqual, value, zero))?;
                                let revert_flag = builder.make(arith::andi(
                                    ctx_is_static,
                                    value_is_not_zero,
                                    location,
                                ))?;
                                builder.create(cf::cond_br(
                                    context,
                                    revert_flag,
                                    &revert_block,
                                    &next_block,
                                    &[],
                                    &[],
                                    location,
                                ));
                                Self::intern_call(context, op, value, 3)?;
                            }
                        }
                    }
                }
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

        let gas = rewriter.make(arith::trunci(gas, rewriter.intrinsics.i64_ty, location))?;
        let args_offset = rewriter.make(arith::trunci(
            args_offset,
            rewriter.intrinsics.i64_ty,
            location,
        ))?;
        let args_size = rewriter.make(arith::trunci(
            args_size,
            rewriter.intrinsics.i64_ty,
            location,
        ))?;
        let ret_offset = rewriter.make(arith::trunci(
            ret_offset,
            rewriter.intrinsics.i64_ty,
            location,
        ))?;
        let ret_size = rewriter.make(arith::trunci(
            ret_size,
            rewriter.intrinsics.i64_ty,
            location,
        ))?;
        let req_arg_mem_size = rewriter.make(arith::addi(args_offset, args_size, location))?;
        let req_ret_mem_size = rewriter.make(arith::addi(ret_offset, ret_size, location))?;

        let req_mem_size =
            rewriter.make(arith::maxui(req_arg_mem_size, req_ret_mem_size, location))?;
        memory::resize_memory(req_mem_size, context, &rewriter, syscall_ctx, location)?;

        let available_gas = gas::get_gas_counter(&rewriter)?;
        let value_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, value, location)?;
        let address_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, address, location)?;

        let gas_ptr = gas::create_gas_var(context, &rewriter, location)?;
        let call_type_value = rewriter.make(arith_constant!(
            rewriter,
            context,
            rewriter.intrinsics.i8_ty,
            CallType::Call as u8 as i64,
            location
        ))?;

        let result = rewriter.make(func::call(
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
            &[rewriter.intrinsics.i8_ty],
            location,
        ))?;
        rewriter.create(llvm::load(
            context,
            gas_ptr,
            rewriter.intrinsics.i64_ty,
            location,
            LoadStoreOptions::default(),
        ));
        rewriter.replace_op(
            *op,
            arith::extui(result, rewriter.intrinsics.i256_ty, location),
        )?;

        Ok(())
    }

    pub(crate) fn creturn(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, offset, size);
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let offset = rewriter.make(arith::trunci(offset, rewriter.intrinsics.i64_ty, location))?;
        let size = rewriter.make(arith::trunci(size, rewriter.intrinsics.i64_ty, location))?;
        let required_size = rewriter.make(arith::addi(size, offset, location))?;
        let gas_counter = gas::get_gas_counter(&rewriter)?;
        memory::resize_memory(required_size, context, &rewriter, syscall_ctx, location)?;
        let reason = rewriter.make(arith_constant!(
            rewriter,
            context,
            rewriter.intrinsics.i8_ty,
            ExitStatusCode::Return.to_u8().into(),
            location
        ))?;
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
