#![allow(dead_code)]

mod backend;
mod code;
mod errors;
mod func;
mod intrinsics;
mod state;
mod ty;

#[cfg(test)]
mod tests;

use std::sync::Arc;

use wasmer_compiler::ModuleMiddleware;

use dora_primitives::config::OptimizationLevel;

/// Represents a WebAssembly (Wasm) to LLVM/MLIR compiler. The `WASMCompiler` struct is responsible for
/// compiling WebAssembly code into LLVM/MLIR, leveraging the configuration provided at the time of
/// instantiation.
///
/// # Fields:
/// - `config`: The configuration settings for the compiler, encapsulated in the `Config` struct.
///
/// # Example Usage:
/// ```no_check
/// let config = Config {
///     enable_nan_canonicalization: true,
///     enable_verifier: false,
///     opt_level: OptimizationLevel::O2,
///     middlewares: vec![],
/// };
/// let wasm_compiler = WASMCompiler::new(config);
///
/// // Access the compiler's configuration
/// let compiler_config = wasm_compiler.config();
/// ```
///
/// # Notes:
/// - The `WASMCompiler` struct encapsulates all compiler functionality for transforming WebAssembly bytecode
///   into a more optimized and executable format using LLVM/MLIR.
/// - The compiler that compiles a WebAssembly module with LLVM/MLIR, translating the Wasm to MLIR IR, optimizing
///   it and then translating to assembly or executing it with JIT mode.
#[derive(Debug, Clone)]
pub struct WASMCompiler {
    /// The configuration settings for the Wasm compiler.
    config: Config,
}

impl WASMCompiler {
    /// Creates a new instance of the `WASMCompiler` with the given configuration.
    ///
    /// # Arguments:
    /// - `config`: The `Config` object that defines the compilation settings, including optimization levels
    ///   and any middleware.
    ///
    /// # Returns:
    /// - A `WASMCompiler` instance initialized with the provided configuration.
    #[inline]
    pub fn new(config: Config) -> WASMCompiler {
        WASMCompiler { config }
    }

    /// Retrieves the configuration for this `WASMCompiler`.
    ///
    /// # Returns:
    /// - A reference to the `Config` object that was used to configure the compiler.
    ///
    /// # Example:
    /// ```no_check
    /// let compiler_config = wasm_compiler.config();
    /// ```
    #[inline]
    fn config(&self) -> &Config {
        &self.config
    }
}

/// Represents the configuration options for the `WASMCompiler`. The `Config` struct defines various
/// settings that influence the compilation process, such as optimization levels and verification options.
///
/// # Fields:
/// - `enable_nan_canonicalization`: A boolean flag indicating whether to enable NaN canonicalization, which
///   standardizes NaN representations in floating-point operations.
/// - `enable_verifier`: A boolean flag to enable or disable the verification phase during compilation, ensuring
///   that the compiled code meets correctness constraints.
/// - `opt_level`: The optimization level to be applied during compilation, which can affect the performance and
///   size of the compiled output.
/// - `middlewares`: A vector of middleware components that modify or extend the behavior of the compilation
///   process. Each middleware is represented as a trait object implementing the `ModuleMiddleware` trait.
///
/// # Example Usage:
/// ```no_check
/// let config = Config {
///     enable_nan_canonicalization: true,
///     enable_verifier: true,
///     opt_level: OptimizationLevel::O3,
///     middlewares: vec![],
/// };
/// ```
///
/// # Notes:
/// - The `Config` struct is essential for controlling the behavior of the `WASMCompiler`. You can tune the
///   compilation process by adjusting these fields based on the requirements of your WebAssembly module.
#[derive(Debug, Clone, Default)]
pub struct Config {
    /// A flag to enable NaN canonicalization during floating-point operations.
    pub(crate) enable_nan_canonicalization: bool,

    /// A flag to enable or disable the verifier during compilation.
    pub(crate) enable_verifier: bool,

    /// The optimization level for the compilation process.
    pub(crate) opt_level: OptimizationLevel,

    /// A collection of middleware components that modify the behavior of the compiler.
    pub(crate) middlewares: Vec<Arc<dyn ModuleMiddleware>>,
}
