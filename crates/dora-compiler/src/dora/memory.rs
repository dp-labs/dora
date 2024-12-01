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
        arith::{self, CmpiPredicate},
        cf, func,
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

pub(crate) fn resize_memory_with_gas_cost<'c>(
    rewriter: &'c Rewriter,
    required_size: Value<'c, 'c>,
    fixed_gas: i64,
) -> Result<Value<'c, 'c>> {
    let context = rewriter.context();
    let location = rewriter.get_insert_location();
    let ptr_type = rewriter.ptr_ty();
    let uint64 = rewriter.intrinsics.i64_ty;

    // Load memory size
    let memory_size_ptr =
        rewriter.make(rewriter.addressof(constants::MEMORY_SIZE_GLOBAL, ptr_type))?;
    let memory_size = rewriter.make(llvm::load(
        context,
        memory_size_ptr,
        uint64,
        location,
        LoadStoreOptions::default(),
    ))?;
    let rounded_required_size = utils::round_up_32(required_size, context, rewriter, location)?;
    let extension_flag = rewriter.make(arith::cmpi(
        context,
        CmpiPredicate::Ult,
        memory_size,
        rounded_required_size,
        location,
    ))?;
    let fixed_gas_value = rewriter.make(rewriter.iconst_64(fixed_gas))?;
    let dynamic_gas_value = rewriter.make(scf::r#if(
        extension_flag,
        &[rewriter.intrinsics.i64_ty],
        {
            let region = Region::new();
            let block = region.append_block(Block::new(&[]));
            let rewriter = Rewriter::new_with_block(context, block);

            // dynamic gas computation in the gas pass
            let memory_cost_before = memory_gas_cost(&rewriter, memory_size)?;
            let memory_cost_after = memory_gas_cost(&rewriter, rounded_required_size)?;
            let dynamic_gas_value =
                rewriter.make(arith::subi(memory_cost_after, memory_cost_before, location))?;

            rewriter.create(scf::r#yield(&[dynamic_gas_value], location));
            region
        },
        {
            let region = Region::new();
            let block = region.append_block(Block::new(&[]));
            let rewriter = Rewriter::new_with_block(context, block);
            rewriter.create(scf::r#yield(
                &[rewriter.make(rewriter.iconst_64(0))?],
                location,
            ));
            region
        },
        location,
    ))?;

    let total_gas = rewriter.make(arith::addi(dynamic_gas_value, fixed_gas_value, location))?;
    Ok(total_gas)
}

pub(crate) fn resize_memory<'c>(
    context: &'c Context,
    op: &OperationRef<'c, 'c>,
    rewriter: &'c Rewriter,
    syscall_ctx: BlockArgument<'c, 'c>,
    required_size: Value<'c, 'c>,
) -> Result<()> {
    // Check the memory offset halt error
    check_op_oog!(op, rewriter, required_size);
    let location = rewriter.get_insert_location();
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

// This function computes memory gas cost, which is given by the following equations.
// memory_size_word = (memory_byte_size + 31) / 32
// memory_cost = (memory_size_word ** 2) / 512 + (3 * memory_size_word)
pub(crate) fn memory_gas_cost<'c>(
    rewriter: &'c Rewriter,
    memory_byte_size: Value<'c, 'c>,
) -> Result<Value<'c, 'c>> {
    let location = rewriter.get_insert_location();
    // Helper function to create constants
    let make_constant =
        |val: i64| -> Result<Value<'c, 'c>> { rewriter.make(rewriter.iconst_64(val)) };

    // Predefined constants
    let constant_3 = make_constant(3)?;
    let constant_31 = make_constant(31)?;
    let constant_32 = make_constant(32)?;
    let constant_512 = make_constant(512)?;

    // Memory calculations
    let memory_byte_size_plus_31 =
        rewriter.make(arith::addi(memory_byte_size, constant_31, location))?;
    let memory_size_word = rewriter.make(arith::divui(
        memory_byte_size_plus_31,
        constant_32,
        location,
    ))?;

    // Word-based calculations
    let memory_size_word_squared =
        rewriter.make(arith::muli(memory_size_word, memory_size_word, location))?;
    let memory_size_word_squared_divided_by_512 = rewriter.make(arith::divui(
        memory_size_word_squared,
        constant_512,
        location,
    ))?;
    let memory_size_word_times_3 =
        rewriter.make(arith::muli(memory_size_word, constant_3, location))?;

    // Final memory cost calculation
    let memory_cost = rewriter.make(arith::addi(
        memory_size_word_squared_divided_by_512,
        memory_size_word_times_3,
        location,
    ))?;
    Ok(memory_cost)
}
