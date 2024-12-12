use crate::backend::IntCC;
use crate::dora::gas::{memory_gas_cost, num_words};
use crate::{
    arith_constant,
    conversion::{builder::OpBuilder, rewriter::Rewriter},
    create_var,
    errors::Result,
    store_var,
};
use crate::{check_op_oog, check_runtime_error, gas_or_fail, if_here, maybe_revert_here};
use block::BlockArgument;
use dora_runtime::constants;
use dora_runtime::constants::GAS_COUNTER_GLOBAL;
use dora_runtime::symbols;
use dora_runtime::ExitStatusCode;
use melior::dialect::arith::CmpiPredicate;
use melior::{
    dialect::{
        arith, cf, func,
        llvm::{self, AllocaOptions, LoadStoreOptions},
        scf,
    },
    ir::{
        attribute::{FlatSymbolRefAttribute, IntegerAttribute, TypeAttribute},
        r#type::IntegerType,
        *,
    },
    Context,
};
use std::mem::offset_of;

/// Allocates memory for a 256-byte value, stores the value in the memory
/// and returns a pointer to the memory
pub(crate) fn allocate_u256_and_assign_value<'c>(
    context: &'c Context,
    rewriter: &'c Rewriter,
    value: Value<'c, '_>,
    location: Location<'c>,
) -> Result<Value<'c, 'c>> {
    let var_ptr = create_var!(rewriter, context, location);
    rewriter.create(store_var!(
        rewriter,
        context,
        value,
        var_ptr,
        location,
        LoadStoreOptions::default()
            .align(IntegerAttribute::new(IntegerType::new(context, 64).into(), 1).into())
    ));
    Ok(var_ptr)
}

pub(crate) fn resize_memory<'c>(
    context: &'c Context,
    op: &OperationRef<'c, 'c>,
    rewriter: &'c Rewriter,
    syscall_ctx: BlockArgument<'c, 'c>,
    offset: Value<'c, 'c>,
    len: Value<'c, 'c>,
) -> Result<()> {
    let location = rewriter.get_insert_location();
    let required_size = rewriter.make(arith::addi(offset, len, location))?;
    // Check the memory offset halt error
    check_op_oog!(op, rewriter, required_size);
    let ptr_type = rewriter.ptr_ty();
    let required_size_words = num_words(&rewriter, required_size, location)?;
    let contant_32 = rewriter.make(rewriter.iconst_64(32))?;
    let rounded_required_size =
        rewriter.make(arith::muli(required_size_words, contant_32, location))?;
    // Load memory size
    let memory_size_ptr =
        rewriter.make(rewriter.addressof(constants::MEMORY_SIZE_GLOBAL, rewriter.ptr_ty()))?;
    let memory_size = rewriter.make(llvm::load(
        context,
        memory_size_ptr,
        rewriter.intrinsics.i64_ty,
        location,
        LoadStoreOptions::default(),
    ))?;
    let memory_size_words = num_words(&rewriter, memory_size, location)?;
    let rounded_memory_size =
        rewriter.make(arith::muli(memory_size_words, contant_32, location))?;
    let extension_flag = rewriter.make(arith::cmpi(
        context,
        CmpiPredicate::Ult,
        rounded_memory_size,
        rounded_required_size,
        location,
    ))?;
    if_here!(op, rewriter, extension_flag, {
        // dynamic gas computation in the gas pass
        let memory_cost_before = memory_gas_cost(&rewriter, memory_size_words)?;
        let memory_cost_after = memory_gas_cost(&rewriter, required_size_words)?;
        let gas = rewriter.make(arith::subi(memory_cost_after, memory_cost_before, location))?;
        gas_or_fail!(op, rewriter, gas);
        let rewriter = Rewriter::new_with_op(context, *op);
        let result_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::EXTEND_MEMORY),
            &[syscall_ctx.into(), rounded_required_size],
            &[ptr_type],
            location,
        ))?;
        let new_memory_ptr = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<*mut u8>, value),
            ptr_type,
        )?;
        let error = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<*mut u8>, error),
            rewriter.intrinsics.i8_ty,
        )?;
        // Check the runtime memory resize halt error
        check_runtime_error!(op, rewriter, error);
        let rewriter = Rewriter::new_with_op(context, *op);
        // Load memory ptr
        let memory_ptr_ptr =
            rewriter.make(rewriter.addressof(constants::MEMORY_PTR_GLOBAL, ptr_type))?;
        rewriter.create(llvm::store(
            context,
            new_memory_ptr,
            memory_ptr_ptr,
            location,
            LoadStoreOptions::default(),
        ));
        // Load memory size
        let memory_size_ptr =
            rewriter.make(rewriter.addressof(constants::MEMORY_SIZE_GLOBAL, ptr_type))?;
        rewriter.create(llvm::store(
            context,
            rounded_required_size,
            memory_size_ptr,
            location,
            LoadStoreOptions::default(),
        ));
    });
    Ok(())
}
