//! Reference: https://github.com/wasmerio/wasmer/tree/main/lib/compiler-llvm
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

use dora_primitives::config::OptimizationLevel;
use func::FuncTranslator;
use melior::ir::operation::OperationBuilder;
use melior::ir::{Block, Region};
use melior::ir::{Location, Module as MLIRModule};
use std::sync::Arc;
use wasmer_compiler::{
    FunctionBodyData, ModuleEnvironment, ModuleMiddleware, ModuleTranslationState,
};
use wasmer_types::entity::{EntityRef, PrimaryMap};
use wasmer_types::{
    CompileModuleInfo, Features, FunctionIndex, LocalFunctionIndex, SectionIndex, SignatureIndex,
    Symbol, SymbolRegistry,
};

use crate::errors::CompileError;
use crate::{Context, Module};

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
pub struct WASMCompiler<'c> {
    /// The MLIR context used for generating operations and managing their lifetime. It encapsulates the state
    /// of the MLIR infrastructure, including types, modules, and operations. This context is tied to the
    /// lifetime `'c` of the EVMCompiler.
    pub ctx: &'c Context,
    /// The configuration settings for the Wasm compiler.
    pub config: Config,
}

impl<'c> WASMCompiler<'c> {
    /// Creates a new instance of the `WASMCompiler` with the given configuration.
    ///
    /// # Arguments:
    /// - `config`: The `Config` object that defines the compilation settings, including optimization levels
    ///   and any middleware.
    ///
    /// # Returns:
    /// - A `WASMCompiler` instance initialized with the provided configuration.
    #[inline]
    pub fn new(ctx: &'c Context, config: Config) -> WASMCompiler {
        WASMCompiler { ctx, config }
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

    /// Compile the WASM bytes using LLVM/MLIR.
    pub fn compile(&self, data: &[u8]) -> Result<Module<'c>, CompileError> {
        let environ = ModuleEnvironment::new();
        let translation = environ
            .translate(data)
            .map_err(|err| CompileError::Codegen(err.to_string()))?;
        let module = translation.module;
        let compile_info = CompileModuleInfo {
            module: Arc::new(module),
            features: Features::default(),
            memory_styles: PrimaryMap::new(),
            table_styles: PrimaryMap::new(),
        };
        self.compile_module(
            &compile_info,
            translation.module_translation_state.as_ref().unwrap(),
            translation.function_body_inputs,
        )
    }

    fn compile_module(
        &self,
        compile_info: &CompileModuleInfo,
        module_translation: &ModuleTranslationState,
        function_body_inputs: PrimaryMap<LocalFunctionIndex, FunctionBodyData<'_>>,
    ) -> Result<Module<'c>, CompileError> {
        // WASM Module information
        let memory_styles = &compile_info.memory_styles;
        let table_styles = &compile_info.table_styles;
        let wasm_module = &compile_info.module;
        let function_body_inputs = function_body_inputs
            .iter()
            .collect::<Vec<(LocalFunctionIndex, &FunctionBodyData<'_>)>>();
        let functions: Vec<_> = function_body_inputs
            .iter()
            .map(|(i, input)| {
                FuncTranslator::translate(
                    self.ctx,
                    &self.config,
                    wasm_module,
                    module_translation,
                    i,
                    input,
                    memory_styles,
                    table_styles,
                    &ShortNames {},
                )
            })
            .collect();
        // MLIR Module region and block
        let module_block = Block::new(&[]);
        for func in functions {
            module_block
                .append_operation(func.map_err(|err| CompileError::Codegen(err.to_string()))?);
        }
        let module_region = Region::new();
        module_region.append_block(module_block);
        // Build main module
        let op = OperationBuilder::new("builtin.module", Location::unknown(&self.ctx.mlir_context))
            .add_regions([module_region])
            .build()?;
        Ok(Module::new(
            MLIRModule::from_operation(op).expect("module failed to create"),
        ))
    }
}

struct ShortNames {}

impl SymbolRegistry for ShortNames {
    fn symbol_to_name(&self, symbol: Symbol) -> String {
        match symbol {
            Symbol::Metadata => "M".to_string(),
            Symbol::LocalFunction(index) => format!("f{}", index.index()),
            Symbol::Section(index) => format!("s{}", index.index()),
            Symbol::FunctionCallTrampoline(index) => format!("t{}", index.index()),
            Symbol::DynamicFunctionTrampoline(index) => format!("d{}", index.index()),
        }
    }

    fn name_to_symbol(&self, name: &str) -> Option<Symbol> {
        if name.len() < 2 {
            return None;
        }
        let (ty, idx) = name.split_at(1);
        if ty.starts_with('M') {
            return Some(Symbol::Metadata);
        }

        let idx = idx.parse::<u32>().ok()?;
        match ty.chars().next().unwrap() {
            'f' => Some(Symbol::LocalFunction(LocalFunctionIndex::from_u32(idx))),
            's' => Some(Symbol::Section(SectionIndex::from_u32(idx))),
            't' => Some(Symbol::FunctionCallTrampoline(SignatureIndex::from_u32(
                idx,
            ))),
            'd' => Some(Symbol::DynamicFunctionTrampoline(FunctionIndex::from_u32(
                idx,
            ))),
            _ => None,
        }
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
