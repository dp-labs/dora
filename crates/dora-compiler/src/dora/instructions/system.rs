use crate::{
    arith_constant,
    conversion::{
        builder::OpBuilder,
        rewriter::{DeferredRewriter, Rewriter},
    },
    create_var,
    dora::{conversion::ConversionPass, gas, memory},
    errors::Result,
    load_by_addr, load_var, operands, rewrite_ctx, syscall_ctx,
};
use dora_runtime::constants;
use dora_runtime::symbols;
use melior::{
    dialect::{
        arith::{self, CmpiPredicate},
        func,
        llvm::{self, AllocaOptions, LoadStoreOptions},
        ods, scf,
    },
    ir::{
        attribute::{FlatSymbolRefAttribute, IntegerAttribute, TypeAttribute},
        r#type::IntegerType,
        Block, OperationRef, Region, Value,
    },
    Context,
};
use num_bigint::BigUint;

impl<'c> ConversionPass<'c> {
    pub(crate) fn keccak256(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, offset, size);
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint32 = rewriter.intrinsics.i32_ty;
        let offset = rewriter.make(arith::trunci(offset, uint32, location))?;
        let size = rewriter.make(arith::trunci(size, uint32, location))?;
        let required_memory_size = rewriter.make(arith::addi(offset, size, location))?;

        // dynamic_gas_cost = 3 * (size + 31) / 32 gas

        memory::resize_memory(
            required_memory_size,
            context,
            &rewriter,
            syscall_ctx,
            location,
        )?;

        let hash_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::KECCAK256_HASHER,
            &[offset, size, hash_ptr],
            hash_ptr,
            rewriter.intrinsics.i256_ty,
            location
        );
        Ok(())
    }

    pub(crate) fn address(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint160 = IntegerType::new(context, 160);
        let uint256 = rewriter.intrinsics.i256_ty;
        let ptr_type = rewriter.intrinsics.ptr_ty;

        // Call to get the address pointer
        let address_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::GET_ADDRESS_PTR),
            &[syscall_ctx.into()],
            &[ptr_type],
            location,
        ))?;
        // Load the address from the pointer
        let address = rewriter.make(llvm::load(
            context,
            address_ptr,
            uint160.into(),
            location,
            LoadStoreOptions::new()
                .align(IntegerAttribute::new(IntegerType::new(context, 64).into(), 1).into()),
        ))?;
        let address = if cfg!(target_endian = "little") {
            rewriter.make(llvm::intr_bswap(address, uint160.into(), location))?
        } else {
            address
        };
        rewriter.make(arith::extui(address, uint256, location))?;
        Ok(())
    }

    pub(crate) fn caller(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let caller_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::STORE_IN_CALLER_PTR,
            &[caller_ptr],
            rewriter.intrinsics.i256_ty,
            location
        );
        Ok(())
    }

    pub(crate) fn callvalue(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);
        let callvalue_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::STORE_IN_CALLVALUE_PTR,
            &[callvalue_ptr],
            rewriter.intrinsics.i256_ty,
            location
        );
        Ok(())
    }

    pub(crate) fn calldataload(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, offset);
        rewrite_ctx!(context, op, rewriter, location);

        let uint1 = rewriter.intrinsics.i1_ty;
        let uint8 = rewriter.intrinsics.i8_ty;
        let uint32 = rewriter.intrinsics.i32_ty;
        let uint256 = rewriter.intrinsics.i256_ty;
        let ptr_type = rewriter.intrinsics.ptr_ty;

        let calldata_ptr =
            load_by_addr!(rewriter, constants::CALLDATA_PTR_GLOBAL, rewriter.ptr_ty());
        // Define the maximum slice width (32 bytes)
        let max_slice_width = rewriter.make(rewriter.iconst_256(BigUint::from(32_u8))?)?;
        let calldata_size = load_by_addr!(rewriter, constants::CALLDATA_SIZE_GLOBAL, uint32);
        // convert calldata_size from u32 to u256
        let calldata_size = rewriter.make(arith::extui(calldata_size, uint256, location))?;
        // Compare offset with calldata size
        let offset_cmpi = rewriter.make(arith::cmpi(
            context,
            arith::CmpiPredicate::Ult,
            offset,
            calldata_size,
            location,
        ))?;
        let zero = rewriter.make(rewriter.iconst_256(BigUint::from(0_u8))?)?;

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
                // Calculate calldata pointer at offset
                let calldata_ptr_at_offset = rewriter.make(llvm::get_element_ptr_dynamic(
                    context,
                    calldata_ptr,
                    &[op.operand(0)?],
                    uint8,
                    ptr_type,
                    location,
                ))?;
                // Calculate length of slice (min(calldata_size - offset, 32))
                let len_sub =
                    rewriter.make(arith::subi(calldata_size, op.operand(0)?, location))?;
                let len_min = rewriter.make(arith::minui(len_sub, max_slice_width, location))?;

                // Copy calldata[offset..offset + len] to the stack slot
                rewriter.create(
                    ods::llvm::intr_memcpy(
                        context,
                        stack_slot_ptr,
                        calldata_ptr_at_offset,
                        len_min,
                        IntegerAttribute::new(uint1, 0),
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

    pub(crate) fn calldatasize(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint32 = rewriter.intrinsics.i32_ty;
        let uint256 = rewriter.intrinsics.i256_ty;
        let call_data_size = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::GET_CALLDATA_SIZE),
            &[syscall_ctx.into()],
            &[uint32],
            location,
        ))?;
        rewriter.make(arith::extui(call_data_size, uint256, location))?;
        Ok(())
    }

    pub(crate) fn calldatacopy(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, dest_offset, call_data_offset, length);
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint32 = rewriter.intrinsics.i32_ty;
        let call_data_offset = rewriter.make(arith::trunci(call_data_offset, uint32, location))?;
        let dest_offset = rewriter.make(arith::trunci(dest_offset, uint32, location))?;
        let length = rewriter.make(arith::trunci(length, uint32, location))?;

        // required size = dest_offset + size
        let required_memory_size = rewriter.make(arith::addi(dest_offset, length, location))?;

        // dynamic gas cost = 3 * (size + 31) / 32

        memory::resize_memory(
            required_memory_size,
            context,
            &rewriter,
            syscall_ctx,
            location,
        )?;

        let memory_ptr = memory::get_memory_pointer(context, &rewriter, location)?;
        let memory_dest = rewriter.make(llvm::get_element_ptr_dynamic(
            context,
            memory_ptr,
            &[dest_offset],
            rewriter.intrinsics.i8_ty,
            rewriter.intrinsics.ptr_ty,
            location,
        ))?;
        let zero = rewriter.make(rewriter.iconst_8(0))?;
        rewriter.create(
            ods::llvm::intr_memset(
                context,
                memory_dest,
                zero,
                length,
                IntegerAttribute::new(IntegerType::new(context, 1).into(), 0),
                location,
            )
            .into(),
        );
        let call_data_size = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::GET_CALLDATA_SIZE),
            &[syscall_ctx.into()],
            &[uint32],
            location,
        ))?;
        let flag = rewriter.make(arith::cmpi(
            context,
            CmpiPredicate::Ugt,
            call_data_offset,
            call_data_size,
            location,
        ))?;
        rewriter.create(scf::r#if(
            flag,
            &[],
            {
                let region = Region::new();
                let valid_offset_block = region.append_block(Block::new(&[]));
                let builder = OpBuilder::new_with_block(context, valid_offset_block);
                let remaining_calldata_size =
                    builder.make(arith::subi(call_data_size, call_data_offset, location))?;
                let memcpy_len =
                    builder.make(arith::minui(remaining_calldata_size, length, location))?;
                let calldata_ptr = builder.make(func::call(
                    builder.context(),
                    FlatSymbolRefAttribute::new(builder.context(), symbols::GET_CALLDATA_PTR),
                    &[syscall_ctx.into()],
                    &[builder.ptr_ty()],
                    builder.get_insert_location(),
                ))?;
                let calldata_src = builder.make(llvm::get_element_ptr_dynamic(
                    context,
                    calldata_ptr,
                    &[call_data_offset],
                    builder.intrinsics.i8_ty,
                    builder.ptr_ty(),
                    location,
                ))?;
                builder.create(
                    ods::llvm::intr_memcpy(
                        context,
                        memory_dest,
                        calldata_src,
                        memcpy_len,
                        IntegerAttribute::new(IntegerType::new(context, 1).into(), 0),
                        location,
                    )
                    .into(),
                );
                builder.create(scf::r#yield(&[], location));
                region
            },
            {
                let region = Region::new();
                let invalid_offset_block = region.append_block(Block::new(&[]));
                let builder = OpBuilder::new_with_block(context, invalid_offset_block);
                builder.create(scf::r#yield(&[], location));
                region
            },
            location,
        ));
        Ok(())
    }

    pub(crate) fn codesize(
        context: &Context,
        op: &OperationRef<'_, '_>,
        code_size: u32,
    ) -> Result<()> {
        rewrite_ctx!(context, op, rewriter, _location);
        rewriter.create(rewriter.iconst_256_from_u64(code_size as u64)?);
        Ok(())
    }

    pub(crate) fn codecopy(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, dest_offset, offset, length);
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint32 = rewriter.intrinsics.i32_ty;
        let offset = rewriter.make(arith::trunci(offset, uint32, location))?;
        let dest_offset = rewriter.make(arith::trunci(dest_offset, uint32, location))?;
        let length = rewriter.make(arith::trunci(length, uint32, location))?;

        // required size = dest_offset + size
        let required_memory_size = rewriter.make(arith::addi(dest_offset, length, location))?;

        // dynamic gas cost = 3 * (size + 31) / 32

        memory::resize_memory(
            required_memory_size,
            context,
            &rewriter,
            syscall_ctx,
            location,
        )?;

        rewriter.create(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::COPY_CODE_TO_MEMORY),
            &[syscall_ctx.into(), offset, length, dest_offset],
            &[],
            location,
        ));
        Ok(())
    }

    pub(crate) fn returndatasize(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint32 = rewriter.intrinsics.i32_ty;
        let uint256 = rewriter.intrinsics.i256_ty;
        let data_size = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::GET_RETURN_DATA_SIZE),
            &[syscall_ctx.into()],
            &[uint32],
            location,
        ))?;
        rewriter.create(arith::extui(data_size, uint256, location));
        Ok(())
    }

    pub(crate) fn returndatacopy(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, dest_offset, offset, size);
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let dest_offset = rewriter.make(arith::trunci(
            dest_offset,
            rewriter.intrinsics.i32_ty,
            location,
        ))?;
        let offset = rewriter.make(arith::trunci(offset, rewriter.intrinsics.i32_ty, location))?;
        let size = rewriter.make(arith::trunci(size, rewriter.intrinsics.i32_ty, location))?;
        // Extend memory to required size
        let req_mem_size: Value = rewriter.make(arith::addi(dest_offset, size, location))?;
        memory::resize_memory(req_mem_size, context, &rewriter, syscall_ctx, location)?;
        rewriter.create(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::COPY_RETURN_DATA_INTO_MEMORY),
            &[syscall_ctx.into(), dest_offset, offset, size],
            &[],
            location,
        ));
        Ok(())
    }

    pub(crate) fn gas(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        rewrite_ctx!(context, op, rewriter, location);

        let gas_counter = gas::get_gas_counter(&rewriter)?;
        rewriter.create(arith::extui(
            gas_counter,
            rewriter.intrinsics.i256_ty,
            location,
        ));
        Ok(())
    }
}