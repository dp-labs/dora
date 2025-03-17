use crate::{
    backend::IntCC,
    block_argument,
    conversion::rewriter::Rewriter,
    dora::{conversion::ConversionPass, gas, memory},
    errors::Result,
    gas_or_fail, if_here, operands, rewrite_ctx, u256_as_usize_or_fail,
};
use dora_runtime::{ExitStatusCode, symbols};
use melior::{
    Context,
    dialect::{arith, func},
    ir::{Block, OperationRef, attribute::FlatSymbolRefAttribute},
};

impl ConversionPass<'_> {
    pub(crate) fn dataload(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, offset);
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);
        let offset_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, offset, location)?;
        rewriter.create(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::DATA_LOAD),
            &[syscall_ctx.into(), offset_ptr],
            &[],
            location,
        ));
        rewriter.create(rewriter.load(offset_ptr, rewriter.i256_ty()));
        Ok(())
    }

    pub(crate) fn datasize(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let data_size = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::DATA_SECTION_SIZE),
            &[syscall_ctx.into()],
            &[rewriter.i64_ty()],
            location,
        ))?;
        rewriter.make(arith::extui(data_size, rewriter.i256_ty(), location))?;
        Ok(())
    }

    pub(crate) fn datacopy(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, memory_offset, data_offset, size);
        block_argument!(op, syscall_ctx, gas_counter_ptr);
        let rewriter = Rewriter::new_with_op(context, *op);
        u256_as_usize_or_fail!(op, rewriter, size);
        let size_is_not_zero = rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, size, 0)?)?;
        if_here!(op, rewriter, size_is_not_zero, {
            let gas = gas::compute_copy_cost(&rewriter, size)?;
            gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
            rewrite_ctx!(context, op, rewriter, _location, NoDefer);
            u256_as_usize_or_fail!(op, rewriter, memory_offset);
            memory::resize_memory(
                context,
                op,
                &rewriter,
                syscall_ctx,
                gas_counter_ptr,
                memory_offset,
                size,
            )?;
        });
        rewrite_ctx!(context, op, rewriter, location);
        let memory_offset =
            rewriter.make(arith::trunci(memory_offset, rewriter.i64_ty(), location))?;
        let data_offset =
            memory::allocate_u256_and_assign_value(context, &rewriter, data_offset, location)?;
        rewriter.create(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::DATA_SECTION_COPY),
            &[syscall_ctx.into(), memory_offset, data_offset, size],
            &[],
            location,
        ));
        Ok(())
    }
}
