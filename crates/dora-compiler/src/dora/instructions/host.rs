use crate::backend::IntCC;
use crate::check_u256_to_u64_overflow;
use crate::{
    arith_constant, check_resize_memory,
    conversion::rewriter::{DeferredRewriter, Rewriter},
    create_var,
    dora::{conversion::ConversionPass, memory},
    errors::{CompileError, Result},
    load_var, maybe_revert_here, operands, rewrite_ctx, syscall_ctx,
};
use dora_runtime::symbols;
use melior::{
    dialect::{
        arith::{self},
        cf, func,
        llvm::{self, AllocaOptions, LoadStoreOptions},
    },
    ir::{
        attribute::{FlatSymbolRefAttribute, IntegerAttribute, TypeAttribute},
        operation::OperationRef,
        Value,
    },
    Context,
};
use num_bigint::BigUint;

impl<'c> ConversionPass<'c> {
    pub(crate) fn balance(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, account);
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let address_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, account, location)?;
        let balance_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::STORE_IN_BALANCE,
            &[address_ptr, balance_ptr],
            balance_ptr,
            rewriter.intrinsics.i256_ty,
            location
        );

        Ok(())
    }

    pub(crate) fn selfbalance(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let selfbalance_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::STORE_IN_SELFBALANCE_PTR,
            &[selfbalance_ptr],
            rewriter.intrinsics.i256_ty,
            location
        );
        Ok(())
    }

    pub(crate) fn extcodesize(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, address);
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let codesize_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, address, location)?;
        let codesize = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::GET_CODESIZE_FROM_ADDRESS),
            &[syscall_ctx.into(), codesize_ptr],
            &[rewriter.intrinsics.i64_ty],
            location,
        ))?;

        rewriter.make(arith::extui(
            codesize,
            rewriter.intrinsics.i256_ty,
            location,
        ))?;
        Ok(())
    }

    pub(crate) fn extcodehash(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, address);
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let code_hash_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, address, location)?;
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::GET_CODE_HASH,
            &[code_hash_ptr],
            rewriter.intrinsics.i256_ty,
            location
        );
        Ok(())
    }

    pub(crate) fn extcodecopy(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, address, dest_offset, offset, size);
        syscall_ctx!(op, syscall_ctx);
        let rewriter = Rewriter::new_with_op(context, *op);
        let location = rewriter.get_insert_location();

        let uint64 = rewriter.intrinsics.i64_ty;
        let offset = rewriter.make(arith::trunci(offset, uint64, location))?;
        let size = rewriter.make(arith::trunci(size, uint64, location))?;
        let dest_offset = rewriter.make(arith::trunci(dest_offset, uint64, location))?;

        let address_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, address, location)?;

        // required size = dest_offset + size
        let required_memory_size = rewriter.make(arith::addi(dest_offset, size, location))?;

        // consume 3 * (size + 31) / 32 gas
        // dynamic gas computation

        check_resize_memory!(op, rewriter, required_memory_size);
        rewrite_ctx!(context, op, rewriter, location);
        memory::resize_memory(
            required_memory_size,
            context,
            &rewriter,
            syscall_ctx,
            location,
        )?;
        rewriter.create(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::COPY_EXT_CODE_TO_MEMORY),
            &[syscall_ctx.into(), address_ptr, offset, size, dest_offset],
            &[],
            location,
        ));
        Ok(())
    }

    pub(crate) fn blockhash(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, block_number);
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let block_hash_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, block_number, location)?;
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::GET_BLOCK_HASH,
            &[block_hash_ptr],
            rewriter.intrinsics.i256_ty,
            location
        );
        Ok(())
    }

    pub(crate) fn sload(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, key);
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let key_ptr = memory::allocate_u256_and_assign_value(context, &rewriter, key, location)?;
        let value_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::STORAGE_READ,
            &[key_ptr, value_ptr],
            value_ptr,
            rewriter.intrinsics.i256_ty,
            location
        );
        Ok(())
    }

    pub(crate) fn sstore(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, key, value);
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        // Allocate and store the key and value
        let key_ptr = memory::allocate_u256_and_assign_value(context, &rewriter, key, location)?;
        let value_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, value, location)?;

        rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::STORAGE_WRITE),
            &[syscall_ctx.into(), key_ptr, value_ptr],
            &[rewriter.intrinsics.i64_ty],
            location,
        ))?;

        // Check gas cost in the gas pass.

        Ok(())
    }

    pub(crate) fn tstore(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, key, value);
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let key_ptr = memory::allocate_u256_and_assign_value(context, &rewriter, key, location)?;
        let value_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, value, location)?;
        rewriter.create(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::TRANSIENT_STORAGE_WRITE),
            &[syscall_ctx.into(), key_ptr, value_ptr],
            &[],
            location,
        ));
        Ok(())
    }

    pub(crate) fn tload(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, key);
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let key_ptr = memory::allocate_u256_and_assign_value(context, &rewriter, key, location)?;
        let value_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::TRANSIENT_STORAGE_READ,
            &[key_ptr, value_ptr],
            value_ptr,
            rewriter.intrinsics.i256_ty,
            location
        );
        Ok(())
    }

    pub(crate) fn log(
        context: &Context,
        op: &OperationRef<'_, '_>,
        num_topics: usize,
    ) -> Result<()> {
        operands!(op, offset, size);
        syscall_ctx!(op, syscall_ctx);
        let rewriter = Rewriter::new_with_op(context, *op);
        let location = rewriter.get_insert_location();
        let uint64 = rewriter.intrinsics.i64_ty;

        // Check the log mem offset and size overflow error
        check_u256_to_u64_overflow!(op, rewriter, size);
        let rewriter = Rewriter::new_with_op(context, *op);
        check_u256_to_u64_overflow!(op, rewriter, offset);
        let rewriter = Rewriter::new_with_op(context, *op);

        let offset = rewriter.make(arith::trunci(offset, uint64, location))?;
        let size = rewriter.make(arith::trunci(size, uint64, location))?;

        // dynamic gas computation in the gas pass

        // required_size = offset + size
        let required_memory_size = rewriter.make(arith::addi(offset, size, location))?;
        check_resize_memory!(op, rewriter, required_memory_size);
        rewrite_ctx!(context, op, rewriter, location);
        memory::resize_memory(
            required_memory_size,
            context,
            &rewriter,
            syscall_ctx,
            location,
        )?;

        // Handle topics dynamically
        let mut topic_pointers = vec![];
        for i in 0..num_topics {
            let topic = op.operand(2 + i)?;
            let topic_ptr =
                memory::allocate_u256_and_assign_value(context, &rewriter, topic, location)?;
            topic_pointers.push(topic_ptr);
        }

        let mut call_args: Vec<Value> = vec![syscall_ctx.into(), offset, size];
        call_args.append(&mut topic_pointers);
        debug_assert!(num_topics <= 4);
        let symbol = match num_topics {
            0 => symbols::APPEND_LOG,
            1 => symbols::APPEND_LOG_ONE_TOPIC,
            2 => symbols::APPEND_LOG_TWO_TOPICS,
            3 => symbols::APPEND_LOG_THREE_TOPICS,
            4 => symbols::APPEND_LOG_FOUR_TOPICS,
            _ => {
                return Err(anyhow::anyhow!(CompileError::Codegen(format!(
                    "invalid log topic numbers {num_topics}, expect [0, 4]"
                ))))
            }
        };
        rewriter.create(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbol),
            &call_args,
            &[],
            location,
        ));

        Ok(())
    }

    pub(crate) fn selfdestruct(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, address);
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let address_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, address, location)?;
        rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::SELFDESTRUCT),
            &[syscall_ctx.into(), address_ptr],
            &[rewriter.intrinsics.i64_ty],
            location,
        ))?;

        // dynamic gas computation in the gas pass

        Ok(())
    }
}
