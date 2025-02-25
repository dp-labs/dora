use crate::{
    backend::IntCC,
    block_argument,
    conversion::{builder::OpBuilder, rewriter::Rewriter},
    create_var,
    dora::{conversion::ConversionPass, gas, memory},
    errors::Result,
    gas_or_fail, if_here, operands, rewrite_ctx, u256_to_u64,
};
use dora_runtime::{ExitStatusCode, symbols};
use melior::{
    Context,
    dialect::{
        arith, func,
        llvm::{self, LoadStoreOptions},
        ods, scf,
    },
    ir::{
        Block, OperationRef, Region,
        attribute::{FlatSymbolRefAttribute, IntegerAttribute, TypeAttribute},
    },
};
use num_bigint::BigUint;

impl ConversionPass<'_> {
    pub(crate) fn dataload(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, offset);
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        // List data types needed
        let uint1 = rewriter.i1_ty();
        let uint8 = rewriter.i8_ty();
        let uint64 = rewriter.i64_ty();
        let uint256 = rewriter.i256_ty();
        let ptr_type = rewriter.ptr_ty();

        // Define the maximum slice width (32 bytes)
        let max_slice_width = rewriter.make(rewriter.iconst_256_from_u64(32)?)?;
        let zero = rewriter.make(rewriter.iconst_256(BigUint::from(0_u8))?)?;
        let r#false = IntegerAttribute::new(uint1, 0);

        let data_section_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::DATA_SECTION),
            &[syscall_ctx.into()],
            &[ptr_type],
            location,
        ))?;
        let data_section_size = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::DATA_SECTION_SIZE),
            &[syscall_ctx.into()],
            &[uint64],
            location,
        ))?;
        // Convert `data_section_size` from u64 to u256
        let data_section_size =
            rewriter.make(arith::extui(data_section_size, uint256, location))?;

        // Convert `offset` from u16 to u256 in case `DATALOADN`
        let offset: melior::ir::Value<'_, '_> =
            rewriter.make(arith::extui(offset, uint256, location))?;
        // Compare offset with data section size
        let offset_cmpi = rewriter.make(arith::cmpi(
            context,
            arith::CmpiPredicate::Ult,
            offset,
            data_section_size,
            location,
        ))?;

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
                // Calculate data section pointer at offset
                let data_section_ptr_at_offset = rewriter.make(llvm::get_element_ptr_dynamic(
                    context,
                    data_section_ptr,
                    &[offset],
                    uint8,
                    ptr_type,
                    location,
                ))?;
                // Calculate length of slice (min(data_section_size - offset, 32))
                let len_sub = rewriter.make(arith::subi(data_section_size, offset, location))?;
                let len_min = rewriter.make(arith::minui(len_sub, max_slice_width, location))?;

                // Copy data_section[offset..offset + len] to the stack slot
                rewriter.create(
                    ods::llvm::intr_memcpy(
                        context,
                        stack_slot_ptr,
                        data_section_ptr_at_offset,
                        len_min,
                        r#false,
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

    pub(crate) fn datasize(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint64 = rewriter.i64_ty();
        let uint256 = rewriter.i256_ty();

        let data_size = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::DATA_SECTION_SIZE),
            &[syscall_ctx.into()],
            &[uint64],
            location,
        ))?;
        rewriter.make(arith::extui(data_size, uint256, location))?;
        Ok(())
    }

    pub(crate) fn datacopy(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, memory_offset, data_offset, size);
        block_argument!(op, syscall_ctx, gas_counter_ptr);
        let rewriter = Rewriter::new_with_op(context, *op);
        u256_to_u64!(op, rewriter, size);
        let size_is_not_zero = rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, size, 0)?)?;
        if_here!(op, rewriter, size_is_not_zero, {
            let gas = gas::compute_copy_cost(&rewriter, size)?;
            gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
            rewrite_ctx!(context, op, rewriter, _location, NoDefer);
            u256_to_u64!(op, rewriter, memory_offset);
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
