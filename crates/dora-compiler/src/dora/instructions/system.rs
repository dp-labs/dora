use crate::{
    arith_constant,
    backend::IntCC,
    block_argument, check_op_oog, check_runtime_error,
    conversion::{
        builder::OpBuilder,
        rewriter::{DeferredRewriter, Rewriter},
    },
    create_var,
    dora::{
        conversion::ConversionPass,
        gas::{compute_copy_cost, compute_keccak256_cost},
        memory::{self, allocate_u256_and_assign_value},
    },
    errors::Result,
    gas_or_fail, if_here, load_var, maybe_revert_here, operands, rewrite_ctx, u256_to_u64,
};
use dora_runtime::symbols;
use dora_runtime::ExitStatusCode;
use melior::{
    dialect::{
        arith, cf, func,
        llvm::{self, AllocaOptions, LoadStoreOptions},
        ods, scf,
    },
    ir::{
        attribute::{FlatSymbolRefAttribute, IntegerAttribute, TypeAttribute},
        r#type::IntegerType,
        Block, OperationRef, Region,
    },
    Context,
};
use num_bigint::BigUint;
use std::mem::offset_of;

impl<'c> ConversionPass<'c> {
    pub(crate) fn keccak256(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, offset, size);
        block_argument!(op, syscall_ctx, gas_counter_ptr);
        let rewriter = Rewriter::new_with_op(context, *op);
        u256_to_u64!(op, rewriter, size);
        let size_is_not_zero = rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, size, 0)?)?;
        if_here!(op, rewriter, size_is_not_zero, {
            let gas = compute_keccak256_cost(&rewriter, size)?;
            gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
            let rewriter = Rewriter::new_with_op(context, *op);
            u256_to_u64!(op, rewriter, offset);
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
        let offset = rewriter.make(arith::trunci(offset, rewriter.intrinsics.i64_ty, location))?;
        let hash_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::KECCAK256_HASHER,
            &[offset, size, hash_ptr],
            [],
            hash_ptr,
            rewriter.intrinsics.i256_ty,
            location
        );
        Ok(())
    }

    pub(crate) fn address(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint160 = IntegerType::new(context, 160).into();
        let uint256 = rewriter.intrinsics.i256_ty;
        let ptr_type = rewriter.ptr_ty();

        // Call to get the address pointer
        let address_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::ADDRESS),
            &[syscall_ctx.into()],
            &[ptr_type],
            location,
        ))?;
        // Load the address from the pointer
        let address = rewriter.make(rewriter.load(address_ptr, uint160))?;
        let address = if cfg!(target_endian = "little") {
            rewriter.make(llvm::intr_bswap(address, uint160, location))?
        } else {
            address
        };
        rewriter.make(arith::extui(address, uint256, location))?;
        Ok(())
    }

    pub(crate) fn caller(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let caller_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::CALLER,
            &[caller_ptr],
            [],
            caller_ptr,
            rewriter.intrinsics.i256_ty,
            location
        );
        Ok(())
    }

    pub(crate) fn callvalue(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);
        let callvalue_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::CALLVALUE,
            &[callvalue_ptr],
            [],
            callvalue_ptr,
            rewriter.intrinsics.i256_ty,
            location
        );
        Ok(())
    }

    pub(crate) fn calldataload(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        block_argument!(op, syscall_ctx);
        operands!(op, offset);
        rewrite_ctx!(context, op, rewriter, location);

        let uint1 = rewriter.intrinsics.i1_ty;
        let uint8 = rewriter.intrinsics.i8_ty;
        let uint256 = rewriter.intrinsics.i256_ty;
        let ptr_type = rewriter.intrinsics.ptr_ty;

        let calldata_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::CALLDATA),
            &[syscall_ctx.into()],
            &[ptr_type],
            location,
        ))?;
        let calldata_size = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::CALLDATA_SIZE),
            &[syscall_ctx.into()],
            &[rewriter.intrinsics.i64_ty],
            location,
        ))?;
        // convert calldata_size from u64 to u256
        let calldata_size = rewriter.make(arith::extui(calldata_size, uint256, location))?;
        // Define the maximum slice width (32 bytes)
        let max_slice_width = rewriter.make(rewriter.iconst_256_from_u64(32)?)?;
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
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint256 = rewriter.intrinsics.i256_ty;
        let calldata_size = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::CALLDATA_SIZE),
            &[syscall_ctx.into()],
            &[rewriter.intrinsics.i64_ty],
            location,
        ))?;
        rewriter.make(arith::extui(calldata_size, uint256, location))?;
        Ok(())
    }

    pub(crate) fn calldatacopy(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, memory_offset, data_offset, size);
        block_argument!(op, syscall_ctx, gas_counter_ptr);
        let rewriter = Rewriter::new_with_op(context, *op);
        u256_to_u64!(op, rewriter, size);
        let size_is_not_zero = rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, size, 0)?)?;
        if_here!(op, rewriter, size_is_not_zero, {
            let gas = compute_copy_cost(&rewriter, size)?;
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
        let memory_offset = rewriter.make(arith::trunci(
            memory_offset,
            rewriter.intrinsics.i64_ty,
            location,
        ))?;
        let data_offset =
            allocate_u256_and_assign_value(context, &rewriter, data_offset, location)?;
        rewriter.create(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::CALLDATA_COPY),
            &[syscall_ctx.into(), memory_offset, data_offset, size],
            &[],
            location,
        ));
        Ok(())
    }

    pub(crate) fn dataload(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, offset);
        rewrite_ctx!(context, op, rewriter, location, NoDefer);

        let eof_data_section_ptr = load_by_addr!(
            rewriter,
            constants::EOF_DATA_SECTION_PTR_GLOBAL,
            rewriter.ptr_ty()
        );

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
        operands!(op, memory_offset, code_offset, size);
        block_argument!(op, syscall_ctx, gas_counter_ptr);
        let rewriter = Rewriter::new_with_op(context, *op);

        u256_to_u64!(op, rewriter, size);
        let size_is_not_zero = rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, size, 0)?)?;
        if_here!(op, rewriter, size_is_not_zero, {
            let gas = compute_copy_cost(&rewriter, size)?;
            gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
            let rewriter = Rewriter::new_with_op(context, *op);
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
        let memory_offset = rewriter.make(arith::trunci(
            memory_offset,
            rewriter.intrinsics.i64_ty,
            location,
        ))?;
        let code_offset =
            allocate_u256_and_assign_value(context, &rewriter, code_offset, location)?;
        rewriter.create(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::CODE_COPY),
            &[syscall_ctx.into(), code_offset, size, memory_offset],
            &[],
            location,
        ));
        Ok(())
    }

    pub(crate) fn returndatasize(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint256 = rewriter.intrinsics.i256_ty;
        let data_size = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::RETURNDATA_SIZE),
            &[syscall_ctx.into()],
            &[rewriter.intrinsics.i64_ty],
            location,
        ))?;
        rewriter.create(arith::extui(data_size, uint256, location));
        Ok(())
    }

    pub(crate) fn returndatacopy(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, memory_offset, data_offset, size);
        block_argument!(op, syscall_ctx, gas_counter_ptr);
        let rewriter = Rewriter::new_with_op(context, *op);
        let ptr_type = rewriter.ptr_ty();
        u256_to_u64!(op, rewriter, size);
        let gas = compute_copy_cost(&rewriter, size)?;
        gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
        let rewriter = Rewriter::new_with_op(context, *op);
        let data_offset_ptr = allocate_u256_and_assign_value(
            context,
            &rewriter,
            data_offset,
            rewriter.get_insert_location(),
        )?;
        let size_is_not_zero = rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, size, 0)?)?;
        if_here!(op, rewriter, size_is_not_zero, {
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
        let memory_offset = rewriter.make(arith::trunci(
            memory_offset,
            rewriter.intrinsics.i64_ty,
            location,
        ))?;
        let result_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::RETURNDATA_COPY),
            &[syscall_ctx.into(), memory_offset, data_offset_ptr, size],
            &[ptr_type],
            location,
        ))?;
        let error = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<()>, error),
            rewriter.intrinsics.i8_ty,
        )?;
        // Check the runtime memory data copy out of offset halt error
        check_runtime_error!(op, rewriter, error);
        Ok(())
    }

    pub(crate) fn gas(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        rewrite_ctx!(context, op, rewriter, location);
        block_argument!(op, _syscall_ctx, gas_counter_ptr);
        let gas_counter =
            rewriter.make(rewriter.load(gas_counter_ptr, rewriter.intrinsics.i64_ty))?;
        rewriter.create(arith::extui(
            gas_counter,
            rewriter.intrinsics.i256_ty,
            location,
        ));
        Ok(())
    }
}
