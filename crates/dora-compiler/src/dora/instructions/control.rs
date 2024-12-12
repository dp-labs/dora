use crate::{
    arith_constant,
    backend::IntCC,
    check_op_oog,
    conversion::{
        builder::OpBuilder,
        rewriter::{DeferredRewriter, Rewriter},
    },
    dora::{conversion::ConversionPass, gas, memory},
    errors::Result,
    if_here, maybe_revert_here, operands, rewrite_ctx, syscall_ctx, u256_to_u64,
};
use dora_runtime::symbols;
use dora_runtime::ExitStatusCode;
use melior::{
    dialect::{arith, cf, func},
    ir::{
        attribute::{FlatSymbolRefAttribute, IntegerAttribute},
        operation::OperationRef,
        Block,
    },
    Context,
};

impl<'c> ConversionPass<'c> {
    pub(crate) fn revert(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, offset, size);
        syscall_ctx!(op, syscall_ctx);
        let rewriter = Rewriter::new_with_op(context, *op);
        let uint8 = rewriter.intrinsics.i8_ty;

        u256_to_u64!(op, rewriter, size);
        let size_is_not_zero = rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, size, 0)?)?;
        if_here!(op, rewriter, size_is_not_zero, {
            u256_to_u64!(op, rewriter, offset);
            memory::resize_memory(context, op, &rewriter, syscall_ctx, offset, size)?;
        });
        rewrite_ctx!(context, op, rewriter, location);
        let offset = rewriter.make(arith::trunci(offset, rewriter.intrinsics.i64_ty, location))?;
        let gas_counter = gas::get_gas_counter(&rewriter)?;
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
