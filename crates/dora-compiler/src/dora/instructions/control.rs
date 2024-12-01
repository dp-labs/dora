use crate::{
    arith_constant,
    backend::IntCC,
    check_op_oog,
    conversion::builder::OpBuilder,
    conversion::rewriter::{DeferredRewriter, Rewriter},
    dora::{conversion::ConversionPass, memory},
    errors::Result,
    load_by_addr, maybe_revert_here, operands, rewrite_ctx, syscall_ctx, u256_to_64,
};
use dora_runtime::ExitStatusCode;
use dora_runtime::{constants, symbols};
use melior::{
    dialect::{arith, cf, func},
    ir::{
        attribute::{FlatSymbolRefAttribute, IntegerAttribute},
        operation::OperationRef,
    },
    Context,
};

impl<'c> ConversionPass<'c> {
    pub(crate) fn revert(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, offset, size);
        syscall_ctx!(op, syscall_ctx);
        let rewriter = Rewriter::new_with_op(context, *op);
        let location = rewriter.get_insert_location();
        let uint8 = rewriter.intrinsics.i8_ty;
        let uint64 = rewriter.intrinsics.i64_ty;
        let ptr_type = rewriter.ptr_ty();

        u256_to_64!(op, rewriter, offset);
        u256_to_64!(op, rewriter, size);

        // required_size = offset + size
        let required_memory_size = rewriter.make(arith::addi(offset, size, location))?;
        memory::resize_memory(context, op, &rewriter, syscall_ctx, required_memory_size)?;
        rewrite_ctx!(context, op, rewriter, location);

        let gas_counter = load_by_addr!(rewriter, constants::GAS_COUNTER_GLOBAL, uint64);
        let reason = rewriter.make(arith_constant!(
            rewriter,
            context,
            uint8,
            ExitStatusCode::Revert.to_u8().into(),
            location
        ))?;
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

    pub(crate) fn stop(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        let rewriter = Rewriter::new(context);
        rewriter.erase_op(*op);
        Ok(())
    }

    pub(crate) fn invalid(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        let rewriter = Rewriter::new(context);
        rewriter.erase_op(*op);
        Ok(())
    }
}
