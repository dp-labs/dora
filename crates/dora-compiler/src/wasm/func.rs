use super::backend::WASMBackend;
use super::code::{FunctionCodeCtx, FunctionCodeGenerator};
use super::intrinsics::CtxType;
use super::ty::{type_to_mlir, type_to_mlir_zero_attribute};
use super::Config;
use super::{intrinsics::WASMIntrinsics, ty::func_type_to_mlir};
use crate::context::Context;
use crate::errors::Result;
use crate::module::Module;
use melior::dialect::{arith, cf, func};
use melior::ir::attribute::{StringAttribute, TypeAttribute};
use melior::ir::{Block, Location, Module as MLIRModule, Region};
use wasmer_compiler::{
    wptype_to_type, FunctionBinaryReader, FunctionBodyData, MiddlewareBinaryReader,
    ModuleMiddlewareChain, ModuleTranslationState,
};
use wasmer_types::entity::PrimaryMap;
use wasmer_types::{
    LocalFunctionIndex, MemoryIndex, MemoryStyle, ModuleInfo, Symbol, SymbolRegistry, TableIndex,
    TableStyle,
};

const FUNCTION_SECTION: &str = "__TEXT,wasmer_function";

/// Responsible for translating WebAssembly functions into the target intermediate representation
/// (IR). The `FuncTranslator` struct holds the necessary context and configuration to guide
/// the function translation process.
///
/// # Fields:
/// - `context`: The `Context` in which the translation takes place. This contains the environment
///   and state required for building and managing the intermediate representation (IR), such as
///   the mapping of WebAssembly operations to target operations.
/// - `config`: The `Config` structure that provides the configuration options, such as optimization
///   levels and verifier settings, to control how the function is translated.
///
/// # Example Usage:
/// ```no_check
/// let func_translator = FuncTranslator {
///     context: /* Context instance */,
///     config: /* Config instance */,
/// };
///
/// // Use func_translator to translate WebAssembly functions into the target IR.
/// func_translator.translate_function(/* FunctionCodeCtx, MLIRModule, etc. */);
/// ```
///
/// # Notes:
/// - The `FuncTranslator` struct is a core component in the WebAssembly-to-IR translation process.
///   It takes function-level WebAssembly code and translates it into a lower-level or intermediate
///   representation (IR) that can be further optimized and executed.
/// - The `context` field holds all necessary data structures and state required to manage and
///   construct the IR during the translation phase.
/// - The `config` field allows users to customize the translation process, enabling or disabling
///   features like optimization passes and code validation.
pub struct FuncTranslator {
    /// Context for managing the intermediate representation during translation.
    context: Context,

    /// Configuration options for the translation process, such as optimization levels.
    config: Config,
}

impl FuncTranslator {
    /// Creates a new instance of the `FuncTranslator` with the default configuration.
    ///
    /// This constructor initializes the `FuncTranslator` with a new context and
    /// the default configuration. It is intended for use when translating WebAssembly functions
    /// into an MLIR-based representation.
    ///
    /// # Returns
    /// A new instance of `FuncTranslator`.
    #[inline]
    pub fn new() -> Self {
        Self {
            context: Context::new(),
            config: Config::default(),
        }
    }

    /// Translates a WebAssembly function to an MLIR module and returns the translated module.
    ///
    /// This method takes various inputs, such as the WebAssembly module, function body, memory styles,
    /// and table styles, and translates them into an MLIR representation. It uses these to generate
    /// the corresponding MLIR module.
    ///
    /// # Parameters
    /// - `wasm_module`: A reference to the WebAssembly `ModuleInfo` to be translated.
    /// - `module_translation`: A reference to the module translation state.
    /// - `local_func_index`: A reference to the local function index within the WebAssembly module.
    /// - `function_body`: The body data of the WebAssembly function.
    /// - `memory_styles`: A map containing memory styles for each memory index in the module.
    /// - `table_styles`: A map containing table styles for each table index in the module.
    /// - `symbol_registry`: A reference to a `SymbolRegistry` used for symbol resolution.
    ///
    /// # Returns
    /// A `Result` containing the translated MLIR `Module` on success, or an error on failure.
    #[allow(clippy::too_many_arguments)]
    pub fn translate_to_module(
        &self,
        wasm_module: &ModuleInfo,
        module_translation: &ModuleTranslationState,
        local_func_index: &LocalFunctionIndex,
        function_body: &FunctionBodyData,
        memory_styles: &PrimaryMap<MemoryIndex, MemoryStyle>,
        table_styles: &PrimaryMap<TableIndex, TableStyle>,
        symbol_registry: &dyn SymbolRegistry,
    ) -> Result<Module> {
        let module = MLIRModule::new(Location::unknown(&self.context.mlir_context));
        self.translate(
            &module,
            wasm_module,
            module_translation,
            local_func_index,
            function_body,
            memory_styles,
            table_styles,
            symbol_registry,
        )?;
        Ok(Module::new(module))
    }

    /// Translates a WebAssembly function to an MLIR module.
    ///
    /// This method performs the core translation of the WebAssembly function body into
    /// an MLIR-based function. It converts WebAssembly types, memory, and table styles
    /// into their corresponding MLIR types and operations.
    ///
    /// # Parameters
    /// - `module`: The MLIR module to append the translated function.
    /// - `wasm_module`: A reference to the WebAssembly `ModuleInfo`.
    /// - `module_translation`: A reference to the module translation state.
    /// - `local_func_index`: A reference to the local function index within the WebAssembly module.
    /// - `function_body`: The body data of the WebAssembly function.
    /// - `memory_styles`: A map containing memory styles for each memory index in the module.
    /// - `table_styles`: A map containing table styles for each table index in the module.
    /// - `symbol_registry`: A reference to a `SymbolRegistry` used for symbol resolution.
    ///
    /// # Returns
    /// A `Result` indicating success or failure during the translation process.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn translate(
        &self,
        module: &MLIRModule,
        wasm_module: &ModuleInfo,
        module_translation: &ModuleTranslationState,
        local_func_index: &LocalFunctionIndex,
        function_body: &FunctionBodyData,
        memory_styles: &PrimaryMap<MemoryIndex, MemoryStyle>,
        table_styles: &PrimaryMap<TableIndex, TableStyle>,
        symbol_registry: &dyn SymbolRegistry,
    ) -> Result<()> {
        let func_index = wasm_module.func_index(*local_func_index);
        let function_name =
            symbol_registry.symbol_to_name(Symbol::LocalFunction(*local_func_index));
        let _module_name = match wasm_module.name.as_ref() {
            None => format!("<anonymous module> function {}", function_name),
            Some(module_name) => format!("module {} function {}", module_name, function_name),
        };
        let wasm_fn_type = wasm_module
            .signatures
            .get(wasm_module.functions[func_index])
            .unwrap();
        let intrinsics = WASMIntrinsics::declare(&self.context);
        let func_type = func_type_to_mlir(&self.context, &intrinsics, wasm_fn_type);
        let block = self.block_from_func_ty(&func_type)?;
        let func = func::func(
            &self.context.mlir_context,
            StringAttribute::new(&self.context.mlir_context, &function_name),
            TypeAttribute::new(func_type.into()),
            {
                let region = Region::new();
                region.append_block(block);
                region
            },
            &[],
            intrinsics.unknown_loc,
        );
        let body_block = module.body();
        let func = body_block.append_operation(func);

        let region = func.region(0).unwrap();
        let setup_block = region.first_block().unwrap();
        let mut reader = MiddlewareBinaryReader::new_with_offset(
            function_body.data,
            function_body.module_offset,
        );
        reader.set_middleware_chain(
            self.config
                .middlewares
                .generate_function_middleware_chain(*local_func_index),
        );
        let mut params = vec![];
        for idx in 0..wasm_fn_type.params().len() {
            let ty = wasm_fn_type.params()[idx];
            let ty = type_to_mlir(&intrinsics, ty);
            let value = setup_block.argument(idx)?;
            params.push((ty, value.into()));
        }

        let mut locals = vec![];
        let num_locals = reader.read_local_count()?;
        for _ in 0..num_locals {
            let (count, ty) = reader.read_local_decl()?;
            let ty = wptype_to_type(ty)?;
            let mlir_ty = type_to_mlir(&intrinsics, ty);
            let mlir_val = type_to_mlir_zero_attribute(&self.context.mlir_context, &intrinsics, ty);
            for _ in 0..count {
                let zero_u32 = setup_block
                    .append_operation(arith::constant(
                        &self.context.mlir_context,
                        mlir_val,
                        Location::unknown(&self.context.mlir_context),
                    ))
                    .result(0)?
                    .into();
                locals.push((mlir_ty, zero_u32));
            }
        }

        let mut params_locals = params.clone();
        params_locals.extend(locals.iter().cloned());
        let mut backend = WASMBackend::new(&self.context);
        let mut fcx = FunctionCodeCtx {
            locals: params_locals,
            ctx: CtxType::new(wasm_module),
            unreachable_depth: 0,
            memory_styles,
            _table_styles: table_styles,
            module,
            module_translation,
            wasm_module,
            symbol_registry,
            config: &self.config,
        };
        let func = setup_block.first_operation().unwrap();
        fcx.ctx.add_func(func_index, func, func_type);
        let mut last_block = setup_block;
        while backend.state.has_control_frames() {
            let pos = reader.current_position() as u32;
            let op = reader.read_operator()?;
            let (block_start, block_end) =
                FunctionCodeGenerator::translate(op, &mut fcx, &mut backend, &region, pos)?;
            last_block.append_operation(cf::br(&block_start, &[], intrinsics.unknown_loc));
            last_block = block_end;
        }
        FunctionCodeGenerator::finalize(&mut backend, last_block, wasm_fn_type, &func_type)?;
        Ok(())
    }

    /// Creates a new MLIR block based on the function type.
    ///
    /// This method constructs an MLIR block with the necessary argument types based on the given
    /// MLIR function type. Each input argument is added to the block with its corresponding type
    /// and location.
    ///
    /// # Parameters
    /// - `ty`: A reference to the MLIR `FunctionType` that defines the input types for the block.
    ///
    /// # Returns
    /// A `Result` containing the new `Block` on success, or an error on failure.
    pub fn block_from_func_ty<'c>(
        &self,
        ty: &melior::ir::r#type::FunctionType<'c>,
    ) -> Result<Block<'c>> {
        let block = Block::new(&[]);
        let count = ty.input_count();
        for i in 0..count {
            let ty = ty.input(i)?;
            block.add_argument(ty, Location::unknown(&self.context.mlir_context));
        }
        Ok(block)
    }
}
