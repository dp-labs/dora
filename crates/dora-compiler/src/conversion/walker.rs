use crate::errors::Result;
use melior::ir::operation::OperationRef;
use mlir_sys::{mlirOperationWalk, MlirOperation, MlirWalkOrder_MlirWalkPreOrder};
use std::ffi::c_void;

pub type WalkFn<'c> = Box<dyn FnMut(OperationRef<'_, '_>) -> Result<()> + 'c>;

/// Walks through the operations in a given operation, applying a specified function to each operation.
///
/// This function allows for traversing the operations in a block, invoking a callback function
/// (`walk_fn`) for each operation encountered. The traversal is performed in pre-order, meaning
/// that the callback is called before processing the operation's successors.
///
/// # Parameters
/// - `op`: A mutable reference to the operation to be walked.
/// - `walk_fn`: A callback function that will be applied to each operation. The function
///   should return a `Result<()>`, allowing for error handling during the walk.
///
/// # Returns
/// Returns a `Result<()>`, indicating success or failure of the walking operation. If the
/// callback function returns an error at any point, the traversal will stop, and the error will
/// be propagated.
///
/// # Safety
/// This function performs unsafe operations, including raw pointer manipulation. The caller
/// must ensure that the `walk_fn` is valid and that no memory is accessed after it has been freed.
///
/// # Example
/// ```no_check
/// walk_operation(op, |op| {
///     // Perform some operations on `op`.
///     Ok(())
/// })?;
/// ```
#[inline]
pub fn walk_operation<'c>(op: OperationRef<'c, 'c>, walk_fn: WalkFn<'c>) -> Result<()> {
    struct WalkContext<'c> {
        walk_fn: WalkFn<'c>,
        result: Result<()>,
    }

    unsafe extern "C" fn callback(op: MlirOperation, user_data: *mut c_void) -> u32 {
        let context = &mut *(user_data as *mut WalkContext);
        if context.result.is_ok() {
            let op = OperationRef::from_raw(op);
            context.result = (context.walk_fn)(op);
        }
        0
    }

    let mut context = WalkContext {
        walk_fn,
        result: Ok(()),
    };

    unsafe {
        mlirOperationWalk(
            op.to_raw(),
            Some(callback),
            &mut context as *mut _ as *mut c_void,
            MlirWalkOrder_MlirWalkPreOrder,
        );
    }

    context.result
}
