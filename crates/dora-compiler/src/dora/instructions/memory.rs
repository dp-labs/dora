use crate::{
    backend::IntCC,
    conversion::rewriter::{DeferredRewriter, Rewriter},
    dora::{conversion::ConversionPass, memory},
    errors::Result,
    load_by_addr, maybe_revert_here, operands, rewrite_ctx, syscall_ctx,
};
use dora_runtime::constants;
use melior::{
    dialect::{
        arith::{self},
        cf,
        llvm::{self, LoadStoreOptions},
        ods,
    },
    ir::{attribute::IntegerAttribute, operation::OperationRef, r#type::IntegerType},
    Context,
};

impl<'c> ConversionPass<'c> {
    pub(crate) fn mload(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, offset);
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint8 = IntegerType::new(context, 8);
        let uint256 = rewriter.intrinsics.i256_ty;
        let offset = rewriter.make(arith::trunci(offset, rewriter.intrinsics.i64_ty, location))?;
        let value_size = rewriter.make(arith::constant(
            context,
            IntegerAttribute::new(rewriter.intrinsics.i64_ty, 32).into(),
            location,
        ))?;
        let required_size = rewriter.make(arith::addi(offset, value_size, location))?;

        memory::resize_memory(required_size, context, &rewriter, syscall_ctx, location)?;

        let memory_ptr = load_by_addr!(rewriter, constants::MEMORY_PTR_GLOBAL, rewriter.ptr_ty());
        let memory_destination = rewriter.make(llvm::get_element_ptr_dynamic(
            context,
            memory_ptr,
            &[offset],
            uint8.into(),
            rewriter.ptr_ty(),
            location,
        ))?;
        let read_value = rewriter.make(llvm::load(
            context,
            memory_destination,
            uint256,
            location,
            LoadStoreOptions::new()
                .align(IntegerAttribute::new(IntegerType::new(context, 64).into(), 1).into()),
        ))?;

        // Check system endianness before storing the value
        if cfg!(target_endian = "little") {
            // Convert the value to big-endian if the system is little-endian
            rewriter.make(llvm::intr_bswap(read_value, uint256, location))?;
        }
        Ok(())
    }

    pub(crate) fn mstore(
        context: &Context,
        op: &OperationRef<'_, '_>,
        byte_size: u32, // Pass `32` for `mstore` and `1` for `mstore8`
    ) -> Result<()> {
        operands!(op, offset, value);
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint8 = rewriter.intrinsics.i8_ty;
        let uint64 = rewriter.intrinsics.i64_ty;
        // If byte_size is 1 (mstore8), truncate value to 1 byte
        let value = if byte_size == 1 {
            rewriter.make(arith::trunci(value, rewriter.intrinsics.i8_ty, location))?
        } else {
            value
        };
        let offset = rewriter.make(arith::trunci(offset, uint64, location))?;
        // Calculate value size (1 byte for mstore8, 32 bytes for mstore)
        let value_size = rewriter.make(rewriter.iconst_64(byte_size as i64))?;
        let required_size = rewriter.make(arith::addi(offset, value_size, location))?;

        memory::resize_memory(required_size, context, &rewriter, syscall_ctx, location)?;

        let memory_ptr = load_by_addr!(rewriter, constants::MEMORY_PTR_GLOBAL, rewriter.ptr_ty());
        // Memory_destination = memory_ptr + offset
        let memory_destination = rewriter.make(llvm::get_element_ptr_dynamic(
            context,
            memory_ptr,
            &[offset],
            uint8,
            rewriter.ptr_ty(),
            location,
        ))?;

        // Check system endianness before storing the value
        let value = if byte_size > 1 && cfg!(target_endian = "little") {
            // Convert the value to big-endian if the system is little-endian
            rewriter.make(llvm::intr_bswap(
                value,
                rewriter.intrinsics.i256_ty,
                location,
            ))?
        } else {
            value
        };
        rewriter.create(llvm::store(
            context,
            value,
            memory_destination,
            location,
            LoadStoreOptions::new()
                .align(IntegerAttribute::new(IntegerType::new(context, 64).into(), 1).into()),
        ));
        Ok(())
    }

    pub(crate) fn msize(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        rewrite_ctx!(context, op, rewriter, location);

        let uint64 = rewriter.intrinsics.i64_ty;
        let uint256 = rewriter.intrinsics.i256_ty;
        let memory_size = load_by_addr!(rewriter, constants::MEMORY_SIZE_GLOBAL, uint64);
        rewriter.make(arith::extui(memory_size, uint256, location))?;
        Ok(())
    }

    pub(crate) fn mcopy(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, dest_offset, offset, size);
        syscall_ctx!(op, syscall_ctx);
        let rewriter = Rewriter::new_with_op(context, *op);
        let location = rewriter.get_insert_location();
        let uint8 = rewriter.intrinsics.i8_ty;
        let uint64 = rewriter.intrinsics.i64_ty;
        let dest_offset = rewriter.make(arith::trunci(dest_offset, uint64, location))?;
        let offset = rewriter.make(arith::trunci(offset, uint64, location))?;
        let size = rewriter.make(arith::trunci(size, uint64, location))?;
        // required size = dest_offset + size
        let src_required_size = rewriter.make(arith::addi(offset, size, location))?;
        // dest_required_size = dest_offset + size
        let dest_required_size = rewriter.make(arith::addi(dest_offset, size, location))?;
        let required_memory_size = rewriter.make(arith::maxui(
            src_required_size,
            dest_required_size,
            location,
        ))?;
        let zero = rewriter.make(rewriter.iconst_64(0))?;
        // Check the memory offset halt error
        let overflow =
            rewriter.make(rewriter.icmp(IntCC::SignedLessThan, required_memory_size, zero))?;
        maybe_revert_here!(op, rewriter, overflow);
        rewrite_ctx!(context, op, rewriter, location);
        memory::resize_memory(
            required_memory_size,
            context,
            &rewriter,
            syscall_ctx,
            location,
        )?;
        let memory_ptr = load_by_addr!(rewriter, constants::MEMORY_PTR_GLOBAL, rewriter.ptr_ty());
        // memory_destination = memory_ptr + offset
        let source = rewriter.make(llvm::get_element_ptr_dynamic(
            context,
            memory_ptr,
            &[offset],
            uint8,
            rewriter.ptr_ty(),
            location,
        ))?;
        // memory_destination = memory_ptr + dest_offset
        let destination = rewriter.make(llvm::get_element_ptr_dynamic(
            context,
            memory_ptr,
            &[dest_offset],
            uint8,
            rewriter.ptr_ty(),
            location,
        ))?;
        // Perform memory move
        rewriter.create(
            ods::llvm::intr_memmove(
                context,
                destination,
                source,
                size,
                IntegerAttribute::new(IntegerType::new(context, 1).into(), 0),
                location,
            )
            .into(),
        );

        Ok(())
    }
}
