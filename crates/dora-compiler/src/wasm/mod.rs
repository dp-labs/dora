//! Reference: https://github.com/wasmerio/wasmer/tree/main/lib/compiler-llvm

pub mod backend;
pub(crate) mod code;
pub(crate) mod conversion;
pub mod errors;
pub(crate) mod func;
pub mod intrinsics;
pub mod meter;
pub mod pass;
pub mod state;
pub(crate) mod symbols;
pub mod ty;

pub use conversion::ConversionPass;

#[cfg(test)]
mod tests;

use dora_runtime::wasm::WASMInstance;
use dora_runtime::wasm::{env::WASMEnv, host};
use func::FuncTranslator;
use melior::ir::operation::OperationBuilder;
use melior::ir::{Block, Region};
use melior::ir::{Location, Module as MLIRModule};
use std::sync::Arc;
use symbols::declare_symbols;
use wasmer::{
    imports, AsStoreMut, AsStoreRef, Exports, Extern, Function, FunctionEnv, Imports,
    Module as WasmModule, Store, Target, VMConfig,
};
use wasmer_compiler::types::module::CompileModuleInfo;
use wasmer_compiler::types::section::SectionIndex;
use wasmer_compiler::types::symbols::{Symbol, SymbolRegistry};
use wasmer_compiler::Engine;
use wasmer_compiler::{
    FunctionBodyData, ModuleEnvironment, ModuleMiddleware, ModuleTranslationState,
};
use wasmer_compiler_cli::store::SubsetTunables;
use wasmer_types::entity::{EntityRef, PrimaryMap};
use wasmer_types::{
    Features, FunctionIndex, LocalFunctionIndex, MemoryIndex, MemoryStyle, SignatureIndex,
    TableIndex, TableStyle,
};
use wasmer_vm::VMInstance;

use crate::errors::CompileError;
use crate::{Compiler, Context, Module};

/// Represents a WebAssembly (Wasm) to LLVM/MLIR compiler. The `WASMCompiler` struct is responsible for
/// compiling WebAssembly code into LLVM/MLIR, leveraging the configuration provided at the time of
/// instantiation.
///
/// # Notes:
/// - The `WASMCompiler` struct encapsulates all compiler functionality for transforming WebAssembly bytecode
///   into a more optimized and executable format using LLVM/MLIR.
/// - The compiler that compiles a WebAssembly module with LLVM/MLIR, translating the Wasm to MLIR IR, optimizing
///   it and then translating to assembly or executing it with JIT mode.
#[derive(Debug, Clone)]
pub struct WASMCompiler<'c> {
    /// The MLIR context used for generating operations and managing their lifetime. It encapsulates the state
    /// of the MLIR infrastructure, including types, modules, and operations.
    pub ctx: &'c Context,
    /// The compile options for the WASM compiler.
    pub opts: WASMCompileOptions,
}

impl<'c> Compiler for WASMCompiler<'c> {
    type Module = [u8];
    type Compilation = Module<'c>;
    type CompileError = CompileError;

    fn name(&self) -> &str {
        "wasm-bytecode-to-mlir-wasm-dialect-compiler"
    }

    fn compile(&self, module: &Self::Module) -> Result<Self::Compilation, Self::CompileError> {
        let target = Target::default();
        let tunables = SubsetTunables::for_target(&target);
        let environ = ModuleEnvironment::new();
        let translation = environ
            .translate(module)
            .map_err(|err| CompileError::Codegen(err.to_string()))?;
        let module = translation.module;
        let memory_styles: PrimaryMap<MemoryIndex, MemoryStyle> = module
            .memories
            .values()
            .map(|memory_type| tunables.memory_style(memory_type))
            .collect();
        let table_styles: PrimaryMap<TableIndex, TableStyle> = module
            .tables
            .values()
            .map(|table_type| tunables.table_style(table_type))
            .collect();
        let compile_info = CompileModuleInfo {
            module: Arc::new(module),
            features: Features::default(),
            memory_styles,
            table_styles,
        };
        self.compile_module(
            &compile_info,
            translation.module_translation_state.as_ref().unwrap(),
            translation.function_body_inputs,
        )
    }
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
    pub fn new(ctx: &'c Context, opts: WASMCompileOptions) -> WASMCompiler<'c> {
        WASMCompiler { ctx, opts }
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
    pub fn config(&self) -> &WASMCompileOptions {
        &self.opts
    }

    /// Compile the WASM bytes using LLVM/MLIR.
    pub fn compile(&self, data: &[u8]) -> Result<Module<'c>, CompileError> {
        let target = Target::default();
        let tunables = SubsetTunables::for_target(&target);
        let environ = ModuleEnvironment::new();
        let translation = environ
            .translate(data)
            .map_err(|err| CompileError::Codegen(err.to_string()))?;
        let module = translation.module;
        let memory_styles: PrimaryMap<MemoryIndex, MemoryStyle> = module
            .memories
            .values()
            .map(|memory_type| tunables.memory_style(memory_type))
            .collect();
        let table_styles: PrimaryMap<TableIndex, TableStyle> = module
            .tables
            .values()
            .map(|table_type| tunables.table_style(table_type))
            .collect();
        let compile_info = CompileModuleInfo {
            module: Arc::new(module),
            features: Features::default(),
            memory_styles,
            table_styles,
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
                    &self.opts,
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
        let mlir_module = MLIRModule::from_operation(op).expect("module failed to create");
        declare_symbols(&self.ctx.mlir_context, &mlir_module);
        Ok(Module::new(mlir_module))
    }

    /// Build the WASM instance with imports
    pub fn build_instance_with_imports(
        &self,
        data: &[u8],
        store: &mut Store,
        imports: Imports,
    ) -> Result<(WasmModule, VMInstance), CompileError> {
        let module =
            WasmModule::new(&store, data).map_err(|err| CompileError::Codegen(err.to_string()))?;
        let engine: &Engine = unsafe { std::mem::transmute(store.engine()) };
        let artifact = engine
            .compile(data)
            .map_err(|err| CompileError::Codegen(err.to_string()))?;
        let externs = imports
            .imports_for_module(&module)
            .map_err(|err| CompileError::Codegen(err.to_string()))?;
        let signal_handler = store.as_store_ref().signal_handler();
        let tunables = engine.tunables();
        unsafe {
            let mut instance_handle = artifact
                .instantiate(
                    tunables,
                    &externs.iter().map(Extern::to_vm_extern).collect::<Vec<_>>(),
                    store.objects_mut(),
                )
                .map_err(|err| CompileError::Codegen(err.to_string()))?;

            // After the instance handle is created, we need to initialize
            // the data, call the start function and so. However, if any
            // of this steps traps, we still need to keep the instance alive
            // as some of the Instance elements may have placed in other
            // instance tables.
            artifact
                .finish_instantiation(
                    &VMConfig {
                        wasm_stack_size: None,
                    },
                    signal_handler,
                    &mut instance_handle,
                )
                .map_err(|err| CompileError::Codegen(err.to_string()))?;

            Ok((module, instance_handle))
        }
    }

    /// Build the WASM instance
    pub fn build_instance(&self, data: &[u8]) -> Result<WASMInstance, CompileError> {
        let mut store = Store::default();
        let wasm_env = WASMEnv { memory: None };
        let func_env = FunctionEnv::new(&mut store, wasm_env);
        macro_rules! func {
            ($func:expr) => {
                Function::new_typed_with_env(&mut store, &func_env, $func)
            };
        }
        let imports = imports! {
            "vm_hooks" => {
                // Dora host functions
                "account_balance" => func!(host::account_balance),
                "account_code" => func!(host::account_code),
                "account_code_size" => func!(host::account_code_size),
                "account_codehash" => func!(host::account_codehash),
                "sload" => func!(host::sload),
                "sstore" => func!(host::sstore),
                "tload" => func!(host::tload),
                "tstore" => func!(host::tstore),
                "block_hash" => func!(host::block_hash),
                "block_basefee" => func!(host::block_basefee),
                "block_blobbasefee" => func!(host::block_blobbasefee),
                "block_gas_limit" => func!(host::block_gas_limit),
                "block_number" => func!(host::block_number),
                "block_timestamp" => func!(host::block_timestamp),
                "block_coinbase" => func!(host::block_coinbase),
                "block_prevrandao" => func!(host::block_prevrandao),
                "chainid" => func!(host::chainid),
                "call" => func!(host::call),
                "delegate_call" => func!(host::delegate_call),
                "static_call" => func!(host::static_call),
                "selfdestruct" => func!(host::selfdestruct),
                "contract_address" => func!(host::contract_address),
                "create" => func!(host::create),
                "create2" => func!(host::create2),
                "emit_log" => func!(host::emit_log),
                "gas_left" => func!(host::gas_left),
                "msg_sender" => func!(host::msg_sender),
                "msg_value" => func!(host::msg_value),
                "keccak256" => func!(host::keccak256),
                "code_copy" => func!(host::code_copy),
                "external_code_copy" => func!(host::external_code_copy),
                "call_data_copy" => func!(host::call_data_copy),
                "return_data_copy" => func!(host::return_data_copy),
                "return_data_size" => func!(host::return_data_size),
                "gas_price" => func!(host::gas_price),
                "tx_origin" => func!(host::tx_origin),
                "write_result" => func!(host::write_result),
                "pay_for_memory_grow" => func!(host::pay_for_memory_grow),
                // Arbitrum stylus host functions
                // Reference: https://github.com/OffchainLabs/stylus-sdk-rs/blob/main/stylus-sdk/src/hostio.rs
                "read_args" => func!(host::call_data_copy),
                "msg_reentrant" => func!(host::msg_reentrant),
                "tx_ink_price" => func!(host::tx_ink_price),
                "tx_gas_price" => func!(host::gas_price),
                "native_keccak256" => func!(host::keccak256),
                "read_return_data" => func!(host::return_data_copy),
                "create1" => func!(host::create),
                "storage_cache_bytes32" => func!(host::sstore),
                "storage_load_bytes32" => func!(host::sload),
                "storage_flush_cache" => func!(host::storage_flush_cache),
                "call_contract" => func!(host::call),
                "delegate_call_contract" => func!(host::delegate_call),
                "static_call_contract" => func!(host::static_call),
                "evm_gas_left" => func!(host::gas_left),
                "evm_ink_left" => func!(host::gas_left),
            },
            "env" => {
                // Pharos network host functions
                "getAddress" => func!(host::contract_address),
                "getBlockHash" => func!(host::block_hash),
                "getBlockCoinbase" => func!(host::block_coinbase),
                "getBlockPrevRandao" => func!(host::block_prevrandao),
                "getBlockGasLimit" => func!(host::block_gas_limit),
                "getBlockTimestamp" => func!(host::block_timestamp),
                "getBlockNumber" => func!(host::block_number),
                "getGasLeft" => func!(host::gas_left),
                "getTxGasPrice" => func!(host::gas_price),
                "getTxOrigin" => func!(host::tx_origin),
                "getBaseFee" => func!(host::block_basefee),
                "getChainId" => func!(host::chainid_u128_dest),
                "getExternalCodeHash" => func!(host::account_codehash),
                "transientLoad" => func!(host::tload),
                "transientStore" => func!(host::tstore),
                "storageLoad" => func!(host::sload),
                "storageStore" => func!(host::sstore),
                "codeCopy" => func!(host::code_copy),
                "getCodeSize" => func!(host::code_size),
                "externalCodeCopy" => func!(host::external_code_copy),
                "getExternalCodeSize" => func!(host::account_code_size),
                "callContract" => func!(host::call_contract),
                "callCode" => func!(host::call_code),
                "callDelegate" => func!(host::call_delegate),
                "callStatic" => func!(host::call_static),
                "createContract" => func!(host::create_contract),
                "returnDataCopy" => func!(host::return_data_copy),
                "callDataCopy" => func!(host::call_data_copy_with_size),
                "getCallDataSize" => func!(host::call_data_size),
                "getReturnDataSize" => func!(host::return_data_size),
                "selfDestruct" => func!(host::selfdestruct),
                "getExternalBalance" => func!(host::account_balance),
                "emitLogEvent" => func!(host::emit_log_event),
                "getCaller" => func!(host::msg_sender),
                "getCallValue" => func!(host::msg_value),
                "keccak256" => func!(host::keccak256),
                "sha256" => func!(host::sha256),
                "finish" => func!(host::write_result),
                "revert" => func!(host::revert),
                "debug_i32" => func!(host::debug_i32),
                "debug_i64" => func!(host::debug_i64),
                "debug_bytes" => func!(host::debug_bytes),
            },
            "console" => {
                "log_txt" => func!(host::debug_bytes),
            }
        };
        let (module, mut instance) = self.build_instance_with_imports(data, &mut store, imports)?;

        let exports = module
            .exports()
            .map(|export| {
                let name = export.name().to_string();
                let export = instance.lookup(&name).expect("export");
                let extern_ = Extern::from_vm_extern(&mut store, export);
                (name, extern_)
            })
            .collect::<Exports>();
        let memory = exports.get_memory("memory").ok().cloned();
        let env = func_env.as_mut(&mut store);
        env.memory = memory;
        Ok(WASMInstance {
            store,
            instance,
            exports,
        })
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
/// let opts = WASMCompileOptions {
///     middlewares: vec![],
///     gas_metering: false,
/// };
/// ```
///
/// # Notes:
/// - The `WASMCompileOptions` struct is essential for controlling the behavior of the `WASMCompiler`. You can tune the
///   compilation process by adjusting these fields based on the requirements of your WebAssembly module.
#[derive(Debug, Clone, Default)]
pub struct WASMCompileOptions {
    /// A collection of middleware components that modify the behavior of the compiler.
    pub middlewares: Vec<Arc<dyn ModuleMiddleware>>,
    /// A flag indicating whether to perform gas metering during compilation.
    pub gas_metering: bool,
}

impl WASMCompileOptions {
    pub fn gas_metering(mut self, gas_metering: bool) -> Self {
        self.gas_metering = gas_metering;
        self
    }
}
