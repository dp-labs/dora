use crate::{
    block_argument,
    conversion::{builder::OpBuilder, rewriter::Rewriter},
    create_var,
    dora::conversion::ConversionPass,
    errors::Result,
    operands, rewrite_ctx,
};
use dora_runtime::symbols;
use melior::{
    dialect::{
        arith, func,
        llvm::{self, AllocaOptions, LoadStoreOptions},
        ods, scf,
    },
    ir::{
        attribute::{FlatSymbolRefAttribute, IntegerAttribute, TypeAttribute},
        Block, OperationRef, Region,
    },
    Context,
};
use num_bigint::BigUint;

impl<'c> ConversionPass<'c> {
    pub(crate) fn dataload(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        block_argument!(op, syscall_ctx);
        operands!(op, offset);
        rewrite_ctx!(context, op, rewriter, location);

        // List data types needed
        let uint1 = rewriter.intrinsics.i1_ty;
        let uint8 = rewriter.intrinsics.i8_ty;
        let uint16 = rewriter.intrinsics.i16_ty;
        let uint256 = rewriter.intrinsics.i256_ty;
        let ptr_type = rewriter.intrinsics.ptr_ty;

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
            &[uint16],
            location,
        ))?;
        // Convert `data_section_size` from u16 to u256
        let data_section_size =
            rewriter.make(arith::extui(data_section_size, uint256, location))?;

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

        let uint16 = rewriter.intrinsics.i16_ty;

        rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::DATA_SECTION_SIZE),
            &[syscall_ctx.into()],
            &[uint16],
            location,
        ))?;
        Ok(())
    }
}
