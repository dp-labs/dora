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
pub use evm::{EVMCompileOptions, EVMCompiler};
pub use module::Module;
pub use wasm::{WASMCompileOptions, WASMCompiler};

/// The `Compiler` trait provides an abstraction for compiling modules into a target-specific format.
/// This trait defines the core operations necessary to perform compilation, including specifying the
/// target architecture, handling errors, and producing a compilation result.
pub trait Compiler {
    /// The type representing a module that is to be compiled.
    type Module: ?Sized;

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
    fn compile(&self, module: &Self::Module) -> Result<Self::Compilation, Self::CompileError>;
}
