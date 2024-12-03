use super::utils;
use crate::backend::IntCC;
use crate::{
    arith_constant,
    conversion::{builder::OpBuilder, rewriter::Rewriter},
    create_var,
    errors::Result,
    store_var,
};
use crate::{check_op_oog, check_runtime_error, maybe_revert_here};
use block::BlockArgument;
use dora_runtime::constants;
use dora_runtime::symbols;
use dora_runtime::ExitStatusCode;
use melior::{
    dialect::{
        arith, cf, func,
        llvm::{self, AllocaOptions, LoadStoreOptions},
    },
    ir::{
        attribute::{FlatSymbolRefAttribute, IntegerAttribute, TypeAttribute},
        r#type::IntegerType,
        *,
    },
    Context,
};
use std::mem::offset_of;

/// Allocates memory for a 64-byte value, stores the value in the memory
/// and returns a pointer to the memory
pub(crate) fn allocate_u64_and_assign_value<'c>(
    context: &'c Context,
    rewriter: &'c Rewriter,
    value: Value<'c, '_>,
    location: Location<'c>,
) -> Result<Value<'c, 'c>> {
    let var_ptr = create_var!(rewriter, context, rewriter.intrinsics.i64_ty, location);
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

pub(crate) fn get_memory_pointer<'c>(
    context: &'c Context,
    rewriter: &'c Rewriter,
    location: Location<'c>,
) -> Result<Value<'c, 'c>> {
    // Define the pointer type
    let ptr_type = rewriter.ptr_ty();

    let memory_ptr_ptr =
        rewriter.make(rewriter.addressof(constants::MEMORY_PTR_GLOBAL, ptr_type))?;
    let memory_ptr = rewriter.make(llvm::load(
        context,
        memory_ptr_ptr,
        ptr_type,
        location,
        LoadStoreOptions::default(),
    ))?;
    Ok(memory_ptr)
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
    let rounded_required_size = utils::round_up_32(required_size, context, &rewriter, location)?;
    let result_ptr = rewriter.make(func::call(
        context,
        FlatSymbolRefAttribute::new(context, symbols::EXTEND_MEMORY),
        &[syscall_ctx.into(), rounded_required_size],
        &[ptr_type],
        location,
    ))?;
    let new_memory_ptr = rewriter.get_field_value(
        result_ptr,
        offset_of!(dora_runtime::context::Result<*mut u8>, value),
        ptr_type,
    )?;
    let error = rewriter.get_field_value(
        result_ptr,
        offset_of!(dora_runtime::context::Result<*mut u8>, error),
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

    Ok(())
}
