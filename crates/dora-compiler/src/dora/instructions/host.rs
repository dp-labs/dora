use crate::backend::IntCC;
use crate::dora::gas::{compute_copy_cost, compute_log_dynamic_cost};
use crate::{
    block_argument,
    conversion::rewriter::Rewriter,
    create_var,
    dora::{conversion::ConversionPass, memory},
    errors::{CompileError, Result},
    load_var, operands, rewrite_ctx, u256_to_u64,
};
use crate::{check_runtime_error, ensure_non_staticcall, gas_or_fail, if_here};
use dora_runtime::symbols;
use dora_runtime::ExitStatusCode;
use melior::{
    dialect::{
        arith::{self},
        func,
    },
    ir::{
        attribute::{FlatSymbolRefAttribute, TypeAttribute},
        operation::OperationRef,
        Block,
    },
    Context,
};
use std::mem::offset_of;

impl ConversionPass<'_> {
    pub(crate) fn balance(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, account);
        block_argument!(op, syscall_ctx, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, location, NoDefer);

        let uint64 = rewriter.uint64_ty();
        let uint256 = rewriter.uint256_ty();

        let address_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, account, location)?;
        let result_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::STORE_IN_BALANCE),
            &[syscall_ctx.into(), address_ptr],
            &[rewriter.ptr_ty()],
            location,
        ))?;
        let gas = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<()>, gas_used),
            uint64,
        )?;
        gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, _location);
        rewriter.make(rewriter.load(address_ptr, uint256))?;
        Ok(())
    }

    pub(crate) fn selfbalance(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint256 = rewriter.uint256_ty();

        let selfbalance_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::STORE_IN_SELFBALANCE_PTR,
            &[selfbalance_ptr],
            uint256,
            location
        );
        Ok(())
    }

    pub(crate) fn extcodesize(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, address);
        block_argument!(op, syscall_ctx, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, location, NoDefer);

        let uint64 = rewriter.uint64_ty();
        let uint256 = rewriter.uint256_ty();
        let ptr_type = rewriter.ptr_ty();

        let codesize_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, address, location)?;
        let result_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::EXT_CODE_SIZE),
            &[syscall_ctx.into(), codesize_ptr],
            &[ptr_type],
            location,
        ))?;
        // We don't need to check for errors here, as no errors will be returned.
        let codesize = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<u64>, value),
            uint64,
        )?;
        let gas = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<u64>, gas_used),
            uint64,
        )?;
        gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, location);
        rewriter.make(arith::extui(codesize, uint256, location))?;
        Ok(())
    }

    pub(crate) fn extcodehash(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, address);
        block_argument!(op, syscall_ctx, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, location, NoDefer);

        let uint64 = rewriter.uint64_ty();
        let uint256 = rewriter.uint256_ty();

        let code_hash_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, address, location)?;
        let result_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::EXT_CODE_HASH),
            &[syscall_ctx.into(), code_hash_ptr],
            &[rewriter.ptr_ty()],
            location,
        ))?;
        let gas = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<()>, gas_used),
            uint64,
        )?;
        gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, _location);
        rewriter.make(rewriter.load(code_hash_ptr, uint256))?;
        Ok(())
    }

    pub(crate) fn extcodecopy(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, address, memory_offset, code_offset, size);
        block_argument!(op, syscall_ctx, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, NoDefer);

        let uint64 = rewriter.uint64_ty();
        let ptr_type = rewriter.ptr_ty();

        u256_to_u64!(op, rewriter, size);
        let gas = compute_copy_cost(&rewriter, size)?;
        gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, NoDefer);
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
        let memory_offset = rewriter.make(arith::trunci(memory_offset, uint64, location))?;
        let address_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, address, location)?;
        let code_offset =
            memory::allocate_u256_and_assign_value(context, &rewriter, code_offset, location)?;

        let result_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::EXT_CODE_COPY),
            &[
                syscall_ctx.into(),
                address_ptr,
                code_offset,
                size,
                memory_offset,
            ],
            &[ptr_type],
            location,
        ))?;
        let gas = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<()>, gas_used),
            uint64,
        )?;
        gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
        Ok(())
    }

    pub(crate) fn blockhash(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, block_number);
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint256 = rewriter.uint256_ty();

        let block_hash_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, block_number, location)?;
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::BLOCK_HASH,
            &[block_hash_ptr],
            uint256,
            location
        );
        Ok(())
    }

    pub(crate) fn sload(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, key);
        block_argument!(op, syscall_ctx, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, location, NoDefer);

        let uint64 = rewriter.uint64_ty();
        let uint256 = rewriter.uint256_ty();

        let key_ptr = memory::allocate_u256_and_assign_value(context, &rewriter, key, location)?;
        let value_ptr = create_var!(rewriter, context, location);
        let result_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::SLOAD),
            &[syscall_ctx.into(), key_ptr, value_ptr],
            &[rewriter.ptr_ty()],
            location,
        ))?;
        let gas = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<()>, gas_used),
            uint64,
        )?;
        gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, _location);
        rewriter.make(rewriter.load(value_ptr, uint256))?;
        Ok(())
    }

    pub(crate) fn sstore(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, key, value);
        block_argument!(op, syscall_ctx, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, location, NoDefer);

        let uint8 = rewriter.uint8_ty();
        let uint64 = rewriter.uint64_ty();
        let ptr_type = rewriter.ptr_ty();

        // Allocate and store the key and value
        let key_ptr = memory::allocate_u256_and_assign_value(context, &rewriter, key, location)?;
        let value_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, value, location)?;
        let gas_counter = rewriter.make(rewriter.load(gas_counter_ptr, uint64))?;

        let result_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::SSTORE),
            &[syscall_ctx.into(), key_ptr, value_ptr, gas_counter],
            &[ptr_type],
            location,
        ))?;
        let error = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<()>, error),
            uint8,
        )?;
        // Check the runtime sstore halt error
        check_runtime_error!(op, rewriter, error);
        rewrite_ctx!(context, op, rewriter, _location);
        let gas = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<()>, gas_used),
            uint64,
        )?;
        gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
        Ok(())
    }

    pub(crate) fn tstore(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, key, value);
        block_argument!(op, syscall_ctx);
        let rewriter = Rewriter::new_with_op(context, *op);
        ensure_non_staticcall!(op, rewriter, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let key_ptr = memory::allocate_u256_and_assign_value(context, &rewriter, key, location)?;
        let value_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, value, location)?;

        rewriter.create(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::TSTORE),
            &[syscall_ctx.into(), key_ptr, value_ptr],
            &[],
            location,
        ));

        Ok(())
    }

    pub(crate) fn tload(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, key);
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint256 = rewriter.uint256_ty();

        let key_ptr = memory::allocate_u256_and_assign_value(context, &rewriter, key, location)?;
        let value_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::TLOAD,
            &[key_ptr, value_ptr],
            [],
            value_ptr,
            uint256,
            location
        );
        Ok(())
    }

    pub(crate) fn log(
        context: &Context,
        op: &OperationRef<'_, '_>,
        num_topics: usize,
    ) -> Result<()> {
        debug_assert!(num_topics <= 4, "invalid log topic count: {num_topics}");
        operands!(op, offset, size);
        block_argument!(op, syscall_ctx, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, NoDefer);

        let uint64 = rewriter.uint64_ty();

        ensure_non_staticcall!(op, rewriter, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, NoDefer);

        // Check the log mem offset and size overflow error
        u256_to_u64!(op, rewriter, size);
        let gas = compute_log_dynamic_cost(&rewriter, size)?;
        gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, NoDefer);

        let size_is_not_zero = rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, size, 0)?)?;
        if_here!(op, rewriter, size_is_not_zero, {
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
        let offset = rewriter.make(arith::trunci(offset, uint64, location))?;
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
        block_argument!(op, syscall_ctx, gas_counter_ptr);
        // Ensure non static call before the gas computation.
        rewrite_ctx!(context, op, rewriter, NoDefer);
        ensure_non_staticcall!(op, rewriter, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint64 = rewriter.uint64_ty();
        let ptr_type = rewriter.ptr_ty();

        let address_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, address, location)?;
        let result_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::SELFDESTRUCT),
            &[syscall_ctx.into(), address_ptr],
            &[ptr_type],
            location,
        ))?;
        let gas = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<u64>, gas_used),
            uint64,
        )?;
        gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
        Ok(())
    }
}
