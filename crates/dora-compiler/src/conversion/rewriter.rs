use super::builder::OpBuilder;
use crate::{
    errors::{CompileError, Result},
    value::IntoContextOperation,
};
use anyhow::bail;
use melior::ir::{
    operation::OperationRefMut, Block, BlockRef, Module, Operation, OperationRef, Value, ValueLike,
};
use melior::Context as MLIRContext;
use mlir_sys::{
    mlirBlockAppendOwnedOperation, mlirBlockGetFirstOperation, mlirOperationGetNextInBlock,
    mlirOperationMoveAfter, mlirOperationRemoveFromParent, mlirValueReplaceAllUsesOfWith,
};
use std::{
    cell::RefCell,
    mem::transmute,
    ops::{Deref, DerefMut},
};

/// The [`Rewriter`] struct provides a high-level interface for transforming and rewriting
/// MLIR operations within a given context.
///
/// It leverages the [`OpBuilder`] to manage the insertion and modification of operations,
/// making it suitable for tasks like replacing operations, modifying the operation sequence,
/// and adjusting the MLIR structure.
///
/// # Fields:
/// - `builder`: An [`OpBuilder`] instance used to construct and modify operations.
///   
///   This field provides the core functionality for operation insertion, replacement,
///   and manipulation within the MLIR context.
///
/// # Purpose:
/// The [`Rewriter`] struct is intended for use in situations where MLIR operations need to be rewritten
/// or transformed.
///
/// By encapsulating an [`OpBuilder`], it allows users to make modifications to existing operations and
/// insert new ones with ease.
///
/// # Example Usage:
/// ```ignore
/// let rewriter = Rewriter::new(&mlir_context);
/// // Use the rewriter to modify operations or replace existing ones.
/// ```
///
/// # Notes:
/// - The [`Rewriter`] simplifies the process of rewriting MLIR operations by utilizing the [`OpBuilder`] for
///   operation construction and modification.
/// - The `builder` field provides all the necessary tools for interacting with the MLIR context, allowing
///   fine-grained control over where and how operations are modified or inserted.
#[derive(Debug)]
pub struct Rewriter<'c, 'a> {
    /// The underlying `OpBuilder` used to construct and modify operations.
    builder: OpBuilder<'c, 'a>,
}

impl<'c, 'a> Deref for Rewriter<'c, 'a> {
    type Target = OpBuilder<'c, 'a>;

    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}

impl DerefMut for Rewriter<'_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.builder
    }
}

impl<'c, 'a> AsRef<OpBuilder<'c, 'a>> for Rewriter<'c, 'a> {
    fn as_ref(&self) -> &OpBuilder<'c, 'a> {
        &self.builder
    }
}

impl<'c, 'a> AsMut<OpBuilder<'c, 'a>> for Rewriter<'c, 'a> {
    fn as_mut(&mut self) -> &mut OpBuilder<'c, 'a> {
        &mut self.builder
    }
}

impl<'c, 'a> Rewriter<'c, 'a> {
    /// Creates a new rewriter without an insertion point.
    ///
    /// # Parameters
    /// - `ctx`: A reference to the `MLIRContext` in which the rewriter will operate.
    ///
    /// # Returns
    /// A new instance of `Rewriter`.
    pub fn new(ctx: &'c MLIRContext) -> Self {
        Self {
            builder: OpBuilder::new(ctx),
        }
    }

    /// Creates a new rewriter and sets the insertion point to the end of the specified block.
    ///
    /// # Parameters
    /// - `ctx`: A reference to the `MLIRContext` in which the rewriter will operate.
    /// - `block`: A reference to the block where the insertion point will be set to the end.
    ///
    /// # Returns
    /// A new instance of `Rewriter` with the insertion point set to the end of the block.
    pub fn new_with_block(ctx: &'c MLIRContext, block: BlockRef<'c, 'a>) -> Self {
        Self {
            builder: OpBuilder::new_with_block(ctx, block),
        }
    }

    /// Creates a new rewriter and sets the insertion point to the specified operation.
    ///
    /// This will cause subsequent insertions to occur right before the specified operation.
    ///
    /// # Parameters
    /// - `ctx`: A reference to the `MLIRContext` in which the rewriter will operate.
    /// - `op`: A reference to the operation that will serve as the insertion point.
    ///
    /// # Returns
    /// A new instance of `Rewriter` with the insertion point set to the specified operation.
    pub fn new_with_op(ctx: &'c MLIRContext, op: OperationRef<'c, 'a>) -> Self {
        Self {
            builder: OpBuilder::new_with_op(ctx, op),
        }
    }

    /// Creates a new rewriter and sets the insertion point to the specified mutable operation.
    ///
    /// This will cause subsequent insertions to occur right before the specified operation.
    ///
    /// # Parameters
    /// - `ctx`: A reference to the `MLIRContext` in which the rewriter will operate.
    /// - `op`: A mutable reference to the operation that will serve as the insertion point.
    ///
    /// # Returns
    /// A new instance of `Rewriter` with the insertion point set to the specified mutable operation.
    pub fn new_with_op_mut(ctx: &'c MLIRContext, op: OperationRefMut<'c, 'a>) -> Self {
        Self {
            builder: OpBuilder::new_with_op_mut(ctx, op),
        }
    }
}

/// The `Replacer` trait provides methods for replacing operations and values within the
/// context of MLIR. This trait allows for modifying existing operations or values and
/// replacing them with new ones, ensuring that the changes are propagated throughout the
/// entire IR (Intermediate Representation) structure.
///
/// This trait is useful in scenarios where transformations or optimizations need to
/// modify operations or value usage patterns in an MLIR-based execution model.
///
/// # Lifetime Parameters:
/// - `'c`: The lifetime of the MLIR context in which the operations and values reside.
/// - `'a`: The lifetime of the function region in which the values reside.
pub trait Replacer<'c, 'a> {
    /// Replaces the results of the specified operation (`op`) with the results of a new
    /// operation (`new_op`).
    ///
    /// # Requirements:
    /// - The number of results between the original operation and the new operation
    ///   must match.
    /// - The original operation is erased from the IR after the replacement.
    ///
    /// # Parameters:
    /// - `op`: A mutable reference to the original operation whose results will be replaced.
    /// - `new_op`: The new operation whose results will replace those of the original operation.
    ///
    /// # Returns:
    /// - `Result<()>`: Returns an `Ok(())` on success or an error if the replacement fails.
    ///
    /// # Example Usage:
    /// This method is typically used when performing operation replacements where the
    /// structure of the new operation must align with the original one in terms of the
    /// number of results produced.
    fn replace_op(&self, op: OperationRef<'c, '_>, new_op: Operation<'c>) -> Result<()>;

    /// Replaces the results of the specified operation (`op`) with the results from
    /// a list of new operations (`new_ops`).
    ///
    /// # Parameters:
    /// - `op`: A mutable reference to the original operation whose results will be replaced.
    /// - `new_ops`: A vector of new operations whose results will replace those of the
    ///   original operation.
    ///
    /// # Returns:
    /// - `Result<()>`: Returns an `Ok(())` on success or an error if the replacement fails.
    ///
    /// # Example Usage:
    /// This method is useful when the replacement operation involves multiple new operations
    /// replacing a single original operation, while ensuring that the overall number of
    /// results matches.
    fn replace_ops(op: OperationRef<'c, '_>, new_ops: Vec<Operation<'c>>) -> Result<()>;

    /// Replaces all uses of the value `from` with the value `to` in the IR.
    ///
    /// This method scans through all occurrences of the value `from` and replaces them
    /// with the value `to`, ensuring that any usage of the original value is redirected
    /// to the new value.
    ///
    /// # Parameters:
    /// - `from`: The value that is being replaced.
    /// - `to`: The new value that will replace all occurrences of the original value.
    ///
    /// # Example Usage:
    /// This method is useful in transformations where a particular SSA value needs to
    /// be replaced globally across the IR.
    fn replace_all_value_uses_of_with(&self, from: Value<'c, 'a>, to: Value<'c, 'a>);
}

impl<'c, 'a> Replacer<'c, 'a> for Rewriter<'c, 'a> {
    /// Replace the results of the given (original) operation with the specified new op (replacement).
    /// The number of results of the two operations must match. The replaced op is erased.
    fn replace_op(&self, op: OperationRef<'c, '_>, new_op: Operation<'c>) -> Result<()> {
        let result_count = op.result_count();
        debug_assert!(op.result_count() <= new_op.result_count());
        let block = op.block().ok_or(CompileError::Codegen(
            "invalid op without parent block".to_string(),
        ))?;
        for i in 0..result_count {
            let new_result: Value = new_op.result(i)?.into();
            let old_result: Value = op.result(i)?.into();
            unsafe { mlirValueReplaceAllUsesOfWith(old_result.to_raw(), new_result.to_raw()) }
        }
        unsafe {
            block.insert_operation_before(op, new_op);
            let mut op: Operation = transmute(op);
            op.remove_from_parent();
        }
        Ok(())
    }

    /// Replace the results of the given (original) operation with the specified list of values (replacements).
    fn replace_ops(op: OperationRef<'c, '_>, new_ops: Vec<Operation<'c>>) -> Result<()> {
        if let Some(last_op) = new_ops.last() {
            let result_count = op.result_count();
            debug_assert!(result_count <= last_op.result_count());
            let block = op.block().ok_or(CompileError::Codegen(
                "invalid op without parent block".to_string(),
            ))?;
            for i in 0..result_count {
                let new_result: Value = last_op.result(i)?.into();
                let old_result: Value = op.result(i)?.into();
                unsafe {
                    mlirValueReplaceAllUsesOfWith(old_result.to_raw(), new_result.to_raw());
                }
            }
            let insert_point: OperationRef = unsafe { transmute(op) };
            // Insert new operations before the original operation
            for new_op in new_ops {
                block.insert_operation_before(insert_point, new_op);
            }
            // Finally, remove the original operation from the parent block
            unsafe {
                let mut op: Operation = transmute(op);
                op.remove_from_parent();
            }
        }
        Ok(())
    }

    /// Find uses of the value `from` and replace them with the value `to`.
    fn replace_all_value_uses_of_with(&self, from: Value<'c, 'a>, to: Value<'c, 'a>) {
        unsafe { mlirValueReplaceAllUsesOfWith(from.to_raw(), to.to_raw()) }
    }
}

impl<'c> Rewriter<'c, '_> {
    /// Erases an operation that is known to have no uses.
    ///
    /// # Parameters
    /// - `op`: A mutable reference to the operation to be erased.
    ///
    /// # Safety
    /// This method is unsafe because it assumes that the operation does not have any uses,
    /// and removing it from its parent without this guarantee can lead to undefined behavior.
    pub fn erase_op(&self, op: OperationRef<'c, '_>) {
        let mut op: Operation = unsafe { transmute(op) };
        op.remove_from_parent();
    }

    /// Erases all operations defined in the specified block.
    ///
    /// # Parameters
    /// - `block`: A reference to the block from which all operations will be erased.
    ///
    /// This method will continue to remove operations until no operations remain in the block.
    pub fn erase_all_ops_in_block(&self, block: BlockRef<'c, '_>) {
        while let Some(op) = block.terminator() {
            let mut op: Operation = unsafe { transmute(op) };
            op.remove_from_parent();
        }
    }

    /// Splits the operations starting from the specified operation (inclusive) out of the given block
    /// into a new block, and returns the newly created block.
    ///
    /// If `split_before` is `None`, all operations in the current block will be moved to the new block.
    ///
    /// # Parameters
    /// - `block`: A reference to the block from which operations will be split.
    /// - `split_before`: An optional reference to the operation before which to split.
    ///
    /// # Returns
    /// A result containing a reference to the new block created after the split, or an error if
    /// the operation fails (e.g., if the original block has no parent region).
    ///
    /// # Errors
    /// This function will return a `CompileError` if the block does not belong to a parent region.
    pub fn split_block(
        &self,
        block: BlockRef<'c, '_>,
        split_before: Option<OperationRef<'c, '_>>,
    ) -> Result<BlockRef<'c, '_>> {
        let new_block = Block::new(&[]);
        match block.parent_region() {
            Some(parent_region) => {
                let new_block = parent_region.insert_block_after(block, new_block);
                unsafe {
                    let mut op = match split_before {
                        Some(split_before) => split_before.to_raw(),
                        None => mlirBlockGetFirstOperation(block.to_raw()),
                    };
                    let new_block = new_block.to_raw();
                    while !op.ptr.is_null() {
                        let next_op = mlirOperationGetNextInBlock(op);
                        mlirOperationRemoveFromParent(op);
                        mlirBlockAppendOwnedOperation(new_block, op);
                        op = next_op;
                    }
                    let block = BlockRef::from_raw(new_block);
                    Ok(block)
                }
            }
            None => bail!(CompileError::Codegen(
                "split_block function meets a original block without the parent region".to_string(),
            )),
        }
    }
}

/// The `DeferredRewriter` struct provides a mechanism for deferring the rewriting of MLIR operations.
/// It builds on top of the `Rewriter` functionality and introduces the capability to delay operation
/// modifications until a later point in time. This is useful in scenarios where operations need to be
/// rewritten but the exact time or conditions for the rewrite are deferred.
///
/// # Fields:
/// - `rewriter`: An instance of `Rewriter` that handles the core operation rewriting functionality.
/// - `last_op`: A `RefCell` containing an optional `OperationRef`. This field tracks the most recent
///   operation that has been deferred for rewriting, allowing for later manipulation or transformation
///   when the appropriate conditions are met.
///
/// # Purpose:
/// The `DeferredRewriter` is intended for use in situations where operations need to be modified
/// or rewritten, but these changes are deferred until some future point during execution. By tracking
/// the last deferred operation, the struct allows changes to be made later, ensuring that operations
/// are rewritten when needed without immediate execution.
///
/// # Example Usage:
/// ```no_check
/// let deferred_rewriter = DeferredRewriter::new_with_op(&mlir_context, op);
/// // Defer an operation rewrite and apply changes later when needed.
/// ```
///
/// # Notes:
/// - The `DeferredRewriter` defers operation modifications, allowing users to perform the rewrite at
///   a later time when conditions are suitable.
/// - The `last_op` field keeps track of the last deferred operation, ensuring it can be accessed and
///   modified as needed in future steps.
/// - This struct is useful in scenarios where the transformation or rewrite of operations depends on
///   conditions that are determined after the initial operation is constructed.
#[derive(Debug)]
pub struct DeferredRewriter<'c, 'a> {
    /// The `Rewriter` that provides the core functionality for modifying MLIR operations.
    rewriter: Rewriter<'c, 'a>,
    /// A `RefCell` containing an optional `OperationRef` representing the last operation that has
    /// been deferred for rewriting.
    last_op: RefCell<Option<OperationRef<'c, 'a>>>,
}

impl<'c, 'a> Deref for DeferredRewriter<'c, 'a> {
    type Target = Rewriter<'c, 'a>;

    fn deref(&self) -> &Self::Target {
        &self.rewriter
    }
}

impl DerefMut for DeferredRewriter<'_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rewriter
    }
}

impl<'c, 'a> AsRef<Rewriter<'c, 'a>> for DeferredRewriter<'c, 'a> {
    fn as_ref(&self) -> &Rewriter<'c, 'a> {
        &self.rewriter
    }
}

impl<'c, 'a> AsMut<Rewriter<'c, 'a>> for DeferredRewriter<'c, 'a> {
    fn as_mut(&mut self) -> &mut Rewriter<'c, 'a> {
        &mut self.rewriter
    }
}

impl<'c, 'a> DeferredRewriter<'c, 'a> {
    /// Creates a new rewriter and sets the insertion point to the specified operation.
    ///
    /// This will cause subsequent insertions to occur right before the specified operation.
    ///
    /// # Parameters
    /// - `ctx`: A reference to the MLIR context.
    /// - `op`: A reference to the operation that will serve as the insertion point.
    ///
    /// # Returns
    /// An instance of `DeferredRewriter` initialized with the specified insertion point.
    pub fn new_with_op(ctx: &'c MLIRContext, op: OperationRef<'c, 'a>) -> Self {
        Self {
            rewriter: Rewriter::new_with_op(ctx, op),
            last_op: RefCell::new(None),
        }
    }

    /// Creates a new rewriter and sets the insertion point to the specified mutable operation.
    ///
    /// This will cause subsequent insertions to occur right before the specified operation.
    ///
    /// # Parameters
    /// - `ctx`: A reference to the MLIR context.
    /// - `op`: A mutable reference to the operation that will serve as the insertion point.
    ///
    /// # Returns
    /// An instance of `DeferredRewriter` initialized with the specified insertion point.
    pub fn new_with_op_mut(ctx: &'c MLIRContext, op: OperationRefMut<'c, 'a>) -> Self {
        Self {
            rewriter: Rewriter::new_with_op_mut(ctx, op),
            last_op: RefCell::new(None),
        }
    }

    /// Creates and inserts a new operation while saving the last inserted operation.
    ///
    /// # Parameters
    /// - `op`: The new operation to be created and inserted.
    ///
    /// # Returns
    /// A reference to the created and inserted operation.
    pub fn create(&self, op: Operation<'c>) -> OperationRef<'c, '_> {
        let op = self.rewriter.create(op);
        let mut last_op = self.last_op.borrow_mut();
        *last_op = Some(op.to_ctx_operation_ref());
        op
    }

    /// Creates and inserts a new operation and returns the first result.
    ///
    /// # Parameters
    /// - `op`: The new operation to be created and inserted.
    ///
    /// # Returns
    /// A result containing the first created and inserted operation result.
    pub fn make(&self, op: Operation<'c>) -> Result<Value<'c, '_>> {
        let op = self.rewriter.create(op);
        let mut last_op = self.last_op.borrow_mut();
        *last_op = Some(op.to_ctx_operation_ref());
        Ok(op.result(0)?.into())
    }

    /// Creates and inserts a new operation and returns the first `N` results.
    ///
    /// # Parameters
    /// - `op`: The new operation to be created and inserted.
    ///
    /// # Returns
    /// A result containing an array of the first `N` created and inserted operation results.
    ///
    /// # Panics
    /// This method panics if `N` is zero.
    pub fn make_n<const N: usize>(&self, op: Operation<'c>) -> Result<[Value<'c, '_>; N]> {
        debug_assert_ne!(N, 0);
        let op = self.create(op);
        let mut last_op = self.last_op.borrow_mut();
        *last_op = Some(op.to_ctx_operation_ref());
        Ok(std::array::from_fn(|i| op.result(i).unwrap().into()))
    }

    /// Removes the insertion point operation if it exists.
    ///
    /// This method replaces all uses of the old result values from the last inserted operation
    /// with the new result values from the current insertion point operation.
    ///
    /// # Safety
    /// This method is unsafe as it performs a raw pointer operation to remove the insertion point operation.
    pub(crate) fn remove(&self) {
        let last_op = self.last_op.borrow_mut().take();
        if let (Some(op), Some(last_op)) = (self.get_insert_point(), last_op) {
            let result_count = op.result_count();
            debug_assert!(
                op.result_count() <= last_op.result_count(),
                "Please check if the last created operator {} can be reasonably replaced by the existing operator {}. The existing operator result count is {} and the last created operator result count is {}",
                last_op.name().as_string_ref().as_str().unwrap(),
                op.name().as_string_ref().as_str().unwrap(),
                op.result_count(),
                last_op.result_count()
            );
            for i in 0..result_count {
                let new_result: Value = last_op.result(i).unwrap().into();
                let old_result: Value = op.result(i).unwrap().into();
                self.replace_all_value_uses_of_with(old_result, new_result);
            }
            unsafe {
                let mut op: Operation = transmute(op);
                op.remove_from_parent();
            }
        }
    }
}

/// Replaces an operation with a new operation, removing the old operation from its parent block.
///
/// This function replaces the specified operation (`op`) with a new operation (`new_op`).
/// Note that the old operation will be removed from its parent block, and all its uses
/// will be replaced with the new operation's results.
///
/// # Parameters
/// - `op`: A mutable reference to the operation that will be replaced.
/// - `new_op`: The new operation that will replace the old operation.
///
/// # Panics
/// This function asserts that the number of results from the old operation is less than or
/// equal to the number of results from the new operation. If this condition is violated,
/// a panic will occur.
///
/// # Safety
/// This function performs unsafe operations, including raw pointer manipulation to replace
/// and remove the operation. It is the caller's responsibility to ensure that the operation
/// is valid and properly handled.
pub fn replace_op<'c>(op: OperationRef<'c, '_>, new_op: Operation<'c>) {
    let result_count = op.result_count();
    debug_assert!(op.result_count() <= new_op.result_count());
    let block = op.block().unwrap();
    for i in 0..result_count {
        let new_result: Value = new_op.result(i).unwrap().into();
        let old_result: Value = op.result(i).unwrap().into();
        unsafe { mlirValueReplaceAllUsesOfWith(old_result.to_raw(), new_result.to_raw()) }
    }
    unsafe {
        block.insert_operation_before(op, new_op);
        let mut op: Operation = transmute(op);
        op.remove_from_parent();
    }
}

/// Move all operations from one module before another operation
///
/// # Arguments
/// * `from_module` - Source module to move operations from
/// * `to_op` - Target operation to move operations to
pub fn move_all_ops_before_op(from_module: &Module, to_op: OperationRef) {
    let to_op = to_op.to_raw();
    let operation = from_module.as_operation();
    if let Ok(region) = operation.region(0) {
        if let Some(block) = region.first_block() {
            if let Some(from_op) = block.first_operation() {
                unsafe {
                    let mut current_op = from_op.to_raw();
                    while !current_op.ptr.is_null() {
                        let next_op = mlirOperationGetNextInBlock(current_op);
                        mlirOperationMoveAfter(current_op, to_op);
                        current_op = next_op;
                    }
                }
            }
        }
    }
}
