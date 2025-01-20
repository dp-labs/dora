use crate::context::RuntimeContext;
use scoped_tls::scoped_thread_local;
use std::cell::UnsafeCell;

scoped_thread_local!(static INSTANCE: UnsafeCell<RuntimeContext<'static>>);

/// Sets the runtime context for the duration of the given closure.
///
/// This function takes a `RuntimeContext` and a closure `f`. It stores the context in a thread-local variable
/// and executes the closure within the scope of that context.
///
/// # Safety
/// The `runtime_context` is transmuted to a `'static` lifetime, which requires that the context remains valid
/// for the entire duration of the closure `f`. This is safe as long as the context is not dropped or invalidated
/// during the execution of `f`.
///
/// # Parameters
/// - `runtime_context`: The runtime context to be set.
/// - `f`: The closure to execute within the scope of the runtime context.
///
/// # Returns
/// The result of the closure `f`.
pub fn set_runtime_context<F, R>(runtime_context: RuntimeContext<'_>, f: F) -> R
where
    F: FnOnce() -> R,
{
    // Convert the lifetime of `runtime_context` to `'static`.
    // This is safe as long as the context remains valid for the duration of the closure `f`.
    let runtime_context = unsafe {
        std::mem::transmute::<RuntimeContext<'_>, RuntimeContext<'static>>(runtime_context)
    };

    // Wrap the runtime context in an `UnsafeCell` to allow interior mutability.
    let cell = UnsafeCell::new(runtime_context);

    // Set the thread-local variable and execute the closure.
    INSTANCE.set(&cell, f)
}

/// Executes a closure with access to the current runtime context.
///
/// This function provides a way to access the runtime context stored in the thread-local variable.
/// It retrieves the context, converts it back to a mutable reference, and passes it to the closure.
///
/// # Safety
/// The context is retrieved from the `UnsafeCell`, which allows safe interior mutability.
/// The use of `unsafe` is necessary to convert the raw pointer back to a mutable reference.
///
/// # Parameters
/// - `f`: The closure to execute with access to the runtime context.
///
/// # Returns
/// The result of the closure `f`.
#[inline]
pub fn with_runtime_context<R, F>(f: F) -> R
where
    F: FnOnce(&mut RuntimeContext<'_>) -> R,
{
    INSTANCE.with(|cell| {
        // Retrieve the mutable reference from the `UnsafeCell`.
        // This is safe as long as no other references to the same data exist.
        let context = unsafe { &mut *cell.get() };
        // Execute the closure with the mutable context.
        f(context)
    })
}
