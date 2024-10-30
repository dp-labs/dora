pub(crate) mod symbols;
pub(crate) mod value;

pub mod backend;
pub mod context;
pub mod conversion;
pub mod dora;
pub mod errors;
pub mod evm;
pub mod intrinsics;
pub mod module;
pub mod pass;
pub mod state;
pub mod wasm;

pub use context::Context;
pub use evm::EVMCompiler;
pub use module::Module;
pub use wasm::WASMCompiler;

/// The `Compiler` trait provides an abstraction for compiling modules into a target-specific format.
/// This trait defines the core operations necessary to perform compilation, including specifying the
/// target architecture, handling errors, and producing a compilation result.
pub trait Compiler {
    /// The type representing a module that is to be compiled.
    type Module;

    /// The type representing the target environment or architecture for the compilation.
    type Target;

    /// The type representing the result of the compilation process.
    type Compilation;

    /// The type representing an error that can occur during the compilation process.
    type CompileError;

    /// Returns the name of the compiler as a string slice.
    ///
    /// # Returns:
    /// - A string slice (`&str`) containing the name of the compiler.
    fn name(&self) -> &str;

    /// Compiles a given module for a specific target.
    ///
    /// # Parameters:
    /// - `module`: The module to be compiled.
    /// - `target`: The target environment or architecture for which the module is being compiled.
    ///
    /// # Returns:
    /// - `Result<Self::Compilation, Self::CompileError>`: A `Result` containing either the compilation result or an error if compilation fails.
    fn compile(
        &self,
        module: &Self::Module,
        target: &Self::Target,
    ) -> Result<Self::Compilation, Self::CompileError>;
}

/// The `CompilerFactory` trait provides an abstraction for creating instances of a `Compiler`.
/// This trait is used to decouple the creation logic of a compiler from its usage, enabling flexibility
/// and reuse of compiler logic with different targets or contexts.
pub trait CompilerFactory {
    /// The type of compiler this factory creates, which must implement the `Compiler` trait.
    type CompilerType: Compiler;

    /// Creates a new instance of the `CompilerType` for a given target context.
    ///
    /// # Parameters:
    /// - `context`: The target context, representing the environment or architecture for which the compiler will generate code.
    ///
    /// # Returns:
    /// - An instance of `Self::CompilerType`, the created compiler.
    fn create(&self, context: &<Self::CompilerType as Compiler>::Target) -> Self::CompilerType;
}
