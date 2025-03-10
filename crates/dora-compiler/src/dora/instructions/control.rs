use crate::{
    arith_constant,
    backend::IntCC,
    block_argument,
    conversion::rewriter::Rewriter,
    dora::{conversion::ConversionPass, memory},
    errors::Result,
    if_here, operands, rewrite_ctx, u256_as_usize_or_fail,
};
use dora_runtime::ExitStatusCode;
use dora_runtime::symbols;
use melior::{
    Context,
    dialect::{arith, func},
    ir::{Block, attribute::FlatSymbolRefAttribute, operation::OperationRef},
};

impl ConversionPass<'_> {
    pub(crate) fn revert(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, offset, size);
        block_argument!(op, syscall_ctx, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, NoDefer);

        let uint8 = rewriter.i8_ty();
        let uint64 = rewriter.i64_ty();

        u256_as_usize_or_fail!(op, rewriter, size);
        let size_is_not_zero = rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, size, 0)?)?;
        if_here!(op, rewriter, size_is_not_zero, {
            u256_as_usize_or_fail!(op, rewriter, offset);
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
        rewrite_ctx!(context, op, rewriter, location);
        let offset = rewriter.make(arith::trunci(offset, uint64, location))?;
        let gas_counter = rewriter.make(rewriter.load(gas_counter_ptr, uint64))?;
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
            &[],
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
