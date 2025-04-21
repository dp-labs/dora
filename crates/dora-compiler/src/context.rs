use melior::{
    Context as MLIRContext,
    dialect::DialectRegistry,
    utility::{register_all_dialects, register_all_llvm_translations, register_all_passes},
};

/// The `Context` struct represents the MLIR (Multi-Level Intermediate Representation)
/// context in which various operations, types, and other IR constructs exist.
/// It serves as the core environment for creating, managing, and manipulating
/// MLIR-based intermediate representations.
///
/// # Fields:
/// - `mlir_context`: The `MLIRContext` is the underlying context used to manage
///   all MLIR-related constructs, ensuring proper memory management and scoping
///   of IR elements.
///
/// # Example Usage:
/// ```no_check
/// let ctx = Context::new();
/// ```
///
/// This struct is essential when working with MLIR in a Rust-based environment,
/// providing the foundational context to operate on IR.
#[derive(Debug, Eq, PartialEq)]
pub struct Context {
    pub mlir_context: MLIRContext,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        let mlir_context = initialize_mlir();
        Self { mlir_context }
    }
}

/// Initializes and configures a new `MLIRContext` with all available dialects and passes.
///
/// This function sets up an `MLIRContext` by performing the following steps:
/// - Creates a new `MLIRContext`.
/// - Appends a registry of all available dialects.
/// - Attaches a diagnostic handler that logs diagnostics to `stderr`.
/// - Loads all available dialects into the context.
/// - Allows the use of unregistered dialects within the context.
/// - Registers all available optimization and transformation passes.
/// - Registers LLVM translations for further compilation steps.
///
/// # Returns
///
/// A fully initialized `MLIRContext` ready for compiling and optimizing MLIR-based operations.
///
/// # Behavior
///
/// - This context is configured to handle diagnostics and allows unregistered dialects to be used.
/// - It includes all MLIR dialects and passes available in the current setup.
/// - The context supports LLVM translations for lowering MLIR to LLVM IR.
pub fn initialize_mlir() -> MLIRContext {
    let context = MLIRContext::new();
    context.append_dialect_registry(&{
        let registry = DialectRegistry::new();
        register_all_dialects(&registry);
        registry
    });
    context.attach_diagnostic_handler(|diagnostic| {
        eprintln!("{diagnostic}");
        true
    });
    context.load_all_available_dialects();
    context.set_allow_unregistered_dialects(true);
    register_all_passes();
    register_all_llvm_translations(&context);
    context
}
