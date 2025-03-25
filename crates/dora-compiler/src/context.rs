use melior::{
    Context as MLIRContext,
    dialect::DialectRegistry,
    utility::{register_all_dialects, register_all_llvm_translations, register_all_passes},
};
use std::path::PathBuf;

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
/// ```
/// use dora_compiler::Context;
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

/// The `Session` struct represents a session in which an MLIR file is processed.
/// It contains paths to the raw and after-pass MLIR files, allowing the user to track
/// different stages of the MLIR transformation process.
///
/// # Fields:
/// - `raw_mlir_path`: This field holds an optional `PathBuf` pointing to the path
///   of the raw MLIR file before any transformations or passes have been applied.
///   It is `None` if the raw MLIR file path is not provided.
/// - `after_pass_mlir_path`: This field holds an optional `PathBuf` pointing to the path
///   of the MLIR file after various passes and transformations have been applied.
///   It is `None` if the after-pass MLIR file path is not provided.
///
/// # Example Usage:
/// ```
/// use std::path::PathBuf;
/// use dora_compiler::context::Session;
/// 
/// let session = Session {
///     raw_mlir_path: Some(PathBuf::from("input.mlir")),
///     after_pass_mlir_path: Some(PathBuf::from("output.mlir")),
/// };
/// ```
///
/// The `Session` struct helps to keep track of MLIR files at different stages
/// during compilation, especially in workflows where passes are applied to modify
/// the IR in successive steps.
#[derive(Debug, Clone, Default)]
pub struct Session {
    /// The path for the raw mlir file before any transformation passes.
    pub raw_mlir_path: Option<PathBuf>,
    /// The path for the MLIR file after transformation passes have been applied.
    pub after_pass_mlir_path: Option<PathBuf>,
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
