use melior::ir::operation::OperationRefMut;
use melior::ir::{OperationRef, Value, ValueLike, operation::OperationResult};

/// The `ToContextValue` trait defines an interface for converting a value's lifetime
/// from a short-lived lifetime (`'_`) to a longer lifetime (`'c`), which corresponds
/// to the lifetime of the MLIR context.
///
/// # Purpose:
/// The primary purpose of this trait is to ensure that SSA (Static Single Assignment)
/// `Value` objects can be accessed throughout the entire function region in the
/// execution model of a stack-based virtual machine.
///
/// This is necessary to maintain the value's validity for the duration of the MLIR
/// context lifetime, avoiding issues where values are dropped prematurely.
///
/// # Example Usage:
/// When implementing this trait, it is important to ensure that the `Value<'c, 'c>`
/// returned can safely live for the lifetime `'c` of the MLIR context.
pub(crate) trait ToContextValue<'c> {
    fn to_ctx_value(&self) -> Value<'c, 'c>;
}

/// The `IntoContextOperation` trait defines an interface for converting an operation's lifetime
/// from a short-lived lifetime (`'_`) to a longer lifetime (`'c`), which corresponds to the
/// lifetime of the MLIR context.
///
/// # Purpose:
/// The primary purpose of this trait is to ensure that SSA `OperationRef` objects can be accessed
/// throughout the entire function region in the execution model of a stack-based virtual machine.
///
/// By extending the lifetime of operations to match the MLIR context, this allows for
/// more robust and long-lived operation references within the MLIR execution flow.
///
/// # Example Usage:
/// Implement this trait to ensure that operations are accessible for the lifetime `'c`
/// of the MLIR context.
pub(crate) trait IntoContextOperation<'c> {
    /// Converts an operation to a longer-lived `OperationRef<'c, 'c>` that lasts for the
    /// duration of the MLIR context lifetime `'c`.
    ///
    /// # Returns:
    /// - `OperationRef<'c, 'c>`: An `OperationRef` object that can live as long as the
    ///   MLIR context lifetime `'c`.
    fn to_ctx_operation_ref(&self) -> OperationRef<'c, 'c>;
}

impl<'c> ToContextValue<'c> for OperationResult<'c, '_> {
    fn to_ctx_value(&self) -> Value<'c, 'c> {
        unsafe { Value::from_raw(self.to_raw()) }
    }
}

impl<'c> ToContextValue<'c> for Value<'c, '_> {
    fn to_ctx_value(&self) -> Value<'c, 'c> {
        unsafe { Value::from_raw(self.to_raw()) }
    }
}

impl<'c> IntoContextOperation<'c> for OperationRefMut<'_, '_> {
    fn to_ctx_operation_ref(&self) -> OperationRef<'c, 'c> {
        unsafe { OperationRef::from_raw(self.to_raw()) }
    }
}

impl<'c> IntoContextOperation<'c> for OperationRef<'_, '_> {
    fn to_ctx_operation_ref(&self) -> OperationRef<'c, 'c> {
        unsafe { OperationRef::from_raw(self.to_raw()) }
    }
}
