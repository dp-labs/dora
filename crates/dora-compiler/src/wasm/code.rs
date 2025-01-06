#![allow(unused)]

use std::borrow::Borrow;
use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;

use crate::backend::IntCC;
use crate::context::Context;
use crate::conversion::builder;
use crate::conversion::builder::OpBuilder;
use crate::errors::Result;
use crate::intrinsics::{is_f32_arithmetic, is_f64_arithmetic};
use crate::value::ToContextValue;

use super::backend;
use super::backend::is_zero;
use super::backend::WASMBackend;
use super::backend::WASMBuilder;
use super::intrinsics::FunctionCache;
use super::intrinsics::GlobalCache;
use super::intrinsics::{CtxType, WASMIntrinsics};
use super::ty::{type_to_mlir, type_to_mlir_zero_attribute};
use super::Config;
use crate::errors::CompileError;
use crate::module::Module;
use crate::state::BasicBlock;
use crate::state::ControlFrame;
use crate::state::ExtraInfo;
use crate::state::IfElseState;
use crate::state::{PhiValue, State};
use dora_runtime::symbols;
use melior::dialect::arith::{CmpfPredicate, CmpiPredicate};
use melior::dialect::{arith, cf, func, llvm};
use melior::ir::attribute::FlatSymbolRefAttribute;
use melior::ir::attribute::{FloatAttribute, IntegerAttribute};
use melior::ir::{Block, BlockRef, Location, Region, RegionRef, Type, Value};
use melior::ir::{Module as MLIRModule, TypeLike, ValueLike};
use melior::Context as MLIRContext;
use mlir_sys::mlirIntegerTypeGetWidth;
use smallvec::SmallVec;
use wasmer_compiler::from_binaryreadererror_wasmerror;
use wasmer_compiler::wasmparser::Operator;
use wasmer_compiler::wptype_to_type;
use wasmer_compiler::{wpheaptype_to_type, ModuleTranslationState};
use wasmer_types::entity::PrimaryMap;
use wasmer_types::FunctionIndex;
use wasmer_types::GlobalIndex;
use wasmer_types::Symbol;
use wasmer_types::TrapCode;
use wasmer_types::WasmResult;
use wasmer_types::{
    FunctionType, MemoryIndex, MemoryStyle, ModuleInfo, SymbolRegistry, TableIndex, TableStyle,
};

macro_rules! op {
    ($builder:ident, $state:ident, $op:ident, i32) => {
        let (input, _) = $state.pop1_extra()?;
        let result = $builder
            .create(
                dora_ir::wasm::$op(
                    $builder.context(),
                    $builder.uint32_ty(),
                    input,
                    $builder.unknown_loc(),
                )
                .into(),
            )
            .result(0)
            .map_err(|e| CompileError::Codegen(e.to_string()))?
            .to_ctx_value();
        $state.push1(result);
    };
    ($builder:ident, $state:ident, $op:ident, i64) => {
        let (input, _) = $state.pop1_extra()?;
        let result = $builder
            .create(
                dora_ir::wasm::$op(
                    $builder.context(),
                    $builder.uint64_ty(),
                    input,
                    $builder.unknown_loc(),
                )
                .into(),
            )
            .result(0)
            .map_err(|e| CompileError::Codegen(e.to_string()))?
            .to_ctx_value();
        $state.push1(result);
    };
    ($builder:ident, $state:ident, $op:ident, i64 => i32) => {
        let (input, _) = $state.pop1_extra()?;
        let result = $builder
            .create(
                dora_ir::wasm::$op(
                    $builder.context(),
                    $builder.uint32_ty(),
                    input,
                    $builder.unknown_loc(),
                )
                .into(),
            )
            .result(0)
            .map_err(|e| CompileError::Codegen(e.to_string()))?
            .to_ctx_value();
        $state.push1(result);
    };
    ($builder:ident, $state:ident, $op:ident, f32) => {
        let (input, _) = $state.pop1_extra()?;
        let result = $builder
            .create(
                dora_ir::wasm::$op(
                    $builder.context(),
                    $builder.intrinsics.f32_ty,
                    input,
                    $builder.unknown_loc(),
                )
                .into(),
            )
            .result(0)
            .map_err(|e| CompileError::Codegen(e.to_string()))?
            .to_ctx_value();
        $state.push1(result);
    };
    ($builder:ident, $state:ident, $op:ident, f64) => {
        let (input, _) = $state.pop1_extra()?;
        let result = $builder
            .create(
                dora_ir::wasm::$op(
                    $builder.context(),
                    $builder.intrinsics.f64_ty,
                    input,
                    $builder.unknown_loc(),
                )
                .into(),
            )
            .result(0)
            .map_err(|e| CompileError::Codegen(e.to_string()))?
            .to_ctx_value();
        $state.push1(result);
    };
}

macro_rules! bin_op {
    ($builder:ident, $state:ident, $op:ident, i32) => {
        let ((v1, _), (v2, _)) = $state.pop2_extra()?;
        let result = $builder
            .create(
                dora_ir::wasm::$op(
                    $builder.context(),
                    $builder.uint32_ty(),
                    v1,
                    v2,
                    $builder.unknown_loc(),
                )
                .into(),
            )
            .result(0)
            .map_err(|e| CompileError::Codegen(e.to_string()))?
            .to_ctx_value();
        $state.push1(result);
    };
    ($builder:ident, $state:ident, $op:ident, i64) => {
        let ((v1, _), (v2, _)) = $state.pop2_extra()?;
        let result = $builder
            .create(
                dora_ir::wasm::$op(
                    $builder.context(),
                    $builder.uint64_ty(),
                    v1,
                    v2,
                    $builder.unknown_loc(),
                )
                .into(),
            )
            .result(0)
            .map_err(|e| CompileError::Codegen(e.to_string()))?
            .to_ctx_value();
        $state.push1(result);
    };
    ($builder:ident, $state:ident, $op:ident, i64 => i32) => {
        let ((v1, _), (v2, _)) = $state.pop2_extra()?;
        let result = $builder
            .create(
                dora_ir::wasm::$op(
                    $builder.context(),
                    $builder.uint32_ty(),
                    v1,
                    v2,
                    $builder.unknown_loc(),
                )
                .into(),
            )
            .result(0)
            .map_err(|e| CompileError::Codegen(e.to_string()))?
            .to_ctx_value();
        $state.push1(result);
    };
    ($builder:ident, $state:ident, $op:ident, f32) => {
        let ((v1, _), (v2, _)) = $state.pop2_extra()?;
        let result = $builder
            .create(
                dora_ir::wasm::$op(
                    $builder.context(),
                    $builder.intrinsics.f32_ty,
                    v1,
                    v2,
                    $builder.unknown_loc(),
                )
                .into(),
            )
            .result(0)
            .map_err(|e| CompileError::Codegen(e.to_string()))?
            .to_ctx_value();
        $state.push1(result);
    };
    ($builder:ident, $state:ident, $op:ident, f32 => i32) => {
        let ((v1, _), (v2, _)) = $state.pop2_extra()?;
        let result = $builder
            .create(
                dora_ir::wasm::$op(
                    $builder.context(),
                    $builder.uint32_ty(),
                    v1,
                    v2,
                    $builder.unknown_loc(),
                )
                .into(),
            )
            .result(0)
            .map_err(|e| CompileError::Codegen(e.to_string()))?
            .to_ctx_value();
        $state.push1(result);
    };
    ($builder:ident, $state:ident, $op:ident, f64) => {
        let ((v1, _), (v2, _)) = $state.pop2_extra()?;
        let result = $builder
            .create(
                dora_ir::wasm::$op(
                    $builder.context(),
                    $builder.intrinsics.f64_ty,
                    v1,
                    v2,
                    $builder.unknown_loc(),
                )
                .into(),
            )
            .result(0)
            .map_err(|e| CompileError::Codegen(e.to_string()))?
            .to_ctx_value();
        $state.push1(result);
    };
    ($builder:ident, $state:ident, $op:ident, f64 => i32) => {
        let ((v1, _), (v2, _)) = $state.pop2_extra()?;
        let result = $builder
            .create(
                dora_ir::wasm::$op(
                    $builder.context(),
                    $builder.uint32_ty(),
                    v1,
                    v2,
                    $builder.unknown_loc(),
                )
                .into(),
            )
            .result(0)
            .map_err(|e| CompileError::Codegen(e.to_string()))?
            .to_ctx_value();
        $state.push1(result);
    };
}

/// Contains the execution context required for generating WebAssembly function code. The
/// `FunctionCodeCtx` struct is responsible for storing function-specific information such as local
/// variables, context references, module details, and configuration necessary for code generation.
///
/// # Fields:
/// - `locals`: A vector of tuples, where each tuple consists of a `Type` and a `Value`. This represents
///   the function's local variables and parameters, helping manage data during execution.
/// - `ctx`: The `CtxType` structure, which provides the execution context with cached references
///   to memories, tables, globals, and function offsets required for accessing WebAssembly components.
/// - `unreachable_depth`: A count representing how deep the code is inside unreachable blocks. This is
///   used to manage control flow, especially for branches that cannot be reached during execution.
/// - `memory_styles`: A reference to a `PrimaryMap` of `MemoryIndex` to `MemoryStyle`, representing
///   how memories are organized in the module and controlling properties like memory access strategies.
/// - `_table_styles`: A reference to a `PrimaryMap` of `TableIndex` to `TableStyle`, representing the
///   table types and their styles in the WebAssembly module.
/// - `module`: A reference to the MLIR (Multi-Level Intermediate Representation) `MLIRModule` that
///   represents the structure and instructions of the current WebAssembly module.
/// - `module_translation`: A reference to the `ModuleTranslationState`, which contains the state and
///   context needed for translating the WebAssembly module's components, such as types and signatures.
/// - `wasm_module`: A reference to the `ModuleInfo`, which contains the metadata of the WebAssembly module
///   including details about imports, exports, and module-level properties.
/// - `symbol_registry`: A reference to the `SymbolRegistry` that resolves symbols within the module
///   and aids in linking external components or function calls.
/// - `config`: A reference to the `Config`, containing configuration options and flags that control
///   the code generation process.
///
/// # Example Usage:
/// ```no_check
/// let function_code_ctx = FunctionCodeCtx {
///     locals: vec![(i32_type, value)],
///     ctx: execution_ctx,
///     unreachable_depth: 0,
///     memory_styles: &memory_map,
///     _table_styles: &table_map,
///     module: &mlir_module,
///     module_translation: &module_translation_state,
///     wasm_module: &wasm_module_info,
///     symbol_registry: &symbol_registry_impl,
///     config: &wasm_config,
/// };
/// ```
///
/// # Notes:
/// - The `FunctionCodeCtx` struct is critical for managing the state of code generation during
///   WebAssembly function translation. It encapsulates the necessary elements for maintaining
///   local variables, handling memory styles, and coordinating module translation.
/// - This context is essential for WebAssembly to MLIR-based compilation pipelines, where it
///   ensures proper management of local and global data.
pub struct FunctionCodeCtx<'c, 'a> {
    /// Local variables and function parameters as a vector of type-value pairs.
    pub locals: Vec<(Type<'c>, Value<'c, 'a>)>,
    /// Execution context that provides access to WebAssembly components like memories and globals.
    pub ctx: CtxType<'c, 'a>,
    /// Depth count of unreachable code blocks to manage unreachable code paths.
    pub unreachable_depth: usize,
    /// A reference to memory styles for the WebAssembly module.
    pub memory_styles: &'a PrimaryMap<MemoryIndex, MemoryStyle>,
    /// A reference to table styles for the WebAssembly module.
    pub _table_styles: &'a PrimaryMap<TableIndex, TableStyle>,
    /// State for translating the WebAssembly module.
    pub module_translation: &'a ModuleTranslationState,
    /// Metadata about the WebAssembly module, including imports and exports.
    pub wasm_module: &'a ModuleInfo,
    /// A symbol registry for resolving module symbols.
    pub symbol_registry: &'a dyn SymbolRegistry,
    /// Configuration settings for the code generation process.
    pub config: &'a Config,
}

pub struct FunctionCodeGenerator;

impl FunctionCodeGenerator {
    /// Translates a WebAssembly function operation into a MLIR module.
    ///
    /// This method handles various WebAssembly operators, translating them into corresponding
    /// MLIR operations within a specified region. It manages the control flow and handles
    /// unreachable code scenarios by keeping track of unreachable depth.
    ///
    /// # Parameters
    /// - `op`: The WebAssembly operator to be translated.
    /// - `fcx`: A mutable reference to the function context containing state and translation data.
    /// - `backend`: A mutable reference to the WASM backend containing context and state information.
    /// - `region`: A reference to the MLIR region where the translated operations will be appended.
    /// - `_source_loc`: A source location identifier (not used in the translation).
    ///
    /// # Returns
    /// A result containing a tuple of `BlockRef<'c, 'a>`, which represents the start and end blocks
    /// of the translated operation.
    ///
    /// # Errors
    /// This function may return an error if the translation of specific operations fails.
    pub fn translate_op<'c, 'a>(
        op: Operator,
        fcx: &mut FunctionCodeCtx<'c, 'a>,
        backend: &mut WASMBackend<'c>,
        region: &'c Region<'c>,
        block: BlockRef<'c, 'a>,
        _source_loc: u32,
    ) -> Result<BlockRef<'c, 'a>>
    where
        'a: 'c,
    {
        let builder = OpBuilder::new_with_block(&backend.ctx.mlir_context, block);
        let state = &mut backend.state;
        if !state.reachable {
            match op {
                Operator::Block { blockty: _ }
                | Operator::Loop { blockty: _ }
                | Operator::If { blockty: _ } => {
                    fcx.unreachable_depth += 1;
                    return Ok(block);
                }
                Operator::Else => {
                    if fcx.unreachable_depth != 0 {
                        return Ok(block);
                    }
                }
                Operator::End => {
                    if fcx.unreachable_depth != 0 {
                        fcx.unreachable_depth -= 1;
                        return Ok(block);
                    }
                }
                _ => {
                    return Ok(block);
                }
            }
        }
        match op {
            /***************************
             * Control Flow instructions.
             * https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md#control-flow-instructions
             ***************************/
            Operator::Unreachable => {
                builder.create(
                    dora_ir::wasm::unreachable(builder.context(), builder.unknown_loc()).into(),
                );
            }
            Operator::Nop => {
                // Do nothing.
            }
            Operator::Block { blockty } => {
                let phis: SmallVec<[(Type<'c>, Location); 1]> = fcx
                    .module_translation
                    .blocktype_params_results(&blockty)?
                    .1
                    .iter()
                    .map(|&wp_ty| {
                        wptype_to_type(wp_ty).map(|wasm_ty| {
                            (
                                type_to_mlir(&backend.intrinsics, &wasm_ty),
                                builder.unknown_loc(),
                            )
                        })
                    })
                    .collect::<WasmResult<_>>()?;
                let end_block = region.append_block(Block::new(&phis));
                backend.state.push_block(end_block, phis);
            }
            Operator::Loop { blockty } => {
                let blocktypes = fcx.module_translation.blocktype_params_results(&blockty)?;
                let phis: SmallVec<[PhiValue<'c>; 1]> = blocktypes
                    .1
                    .iter()
                    .map(|&wp_ty| {
                        wptype_to_type(wp_ty).map(|wasm_ty| {
                            (
                                type_to_mlir(&backend.intrinsics, &wasm_ty),
                                builder.unknown_loc(),
                            )
                        })
                    })
                    .collect::<WasmResult<_>>()?;
                let loop_next = region.append_block(Block::new(&phis));
                let loop_phis: SmallVec<[PhiValue<'c>; 1]> = blocktypes
                    .0
                    .iter()
                    .map(|&wp_ty| {
                        wptype_to_type(wp_ty).map(|wasm_ty| {
                            (
                                type_to_mlir(&backend.intrinsics, &wasm_ty),
                                builder.unknown_loc(),
                            )
                        })
                    })
                    .collect::<WasmResult<_>>()?;
                let loop_body = region.append_block(Block::new(&loop_phis));
                let (value, _) = state.peek1_extra()?;
                builder.create(cf::br(&loop_body, &[value], builder.unknown_loc()));
                state.push_loop(loop_body, loop_next, loop_phis, phis);
            }
            Operator::If { blockty } => {
                let block_param_types = fcx
                    .module_translation
                    .blocktype_params_results(&blockty)?
                    .0
                    .iter()
                    .map(|&wp_ty| {
                        wptype_to_type(wp_ty).map(|wasm_ty| {
                            (
                                type_to_mlir(&backend.intrinsics, &wasm_ty),
                                builder.unknown_loc(),
                            )
                        })
                    })
                    .collect::<WasmResult<Vec<_>>>()?;

                let then_phis: SmallVec<[PhiValue<'c>; 1]> =
                    block_param_types.iter().copied().collect();
                let else_phis: SmallVec<[PhiValue<'c>; 1]> =
                    block_param_types.iter().copied().collect();

                let end_phis: SmallVec<[PhiValue<'c>; 1]> = fcx
                    .module_translation
                    .blocktype_params_results(&blockty)?
                    .1
                    .iter()
                    .map(|&wp_ty| {
                        wptype_to_type(wp_ty).map(|wasm_ty| {
                            (
                                type_to_mlir(&backend.intrinsics, &wasm_ty),
                                builder.unknown_loc(),
                            )
                        })
                    })
                    .collect::<WasmResult<_>>()?;

                let if_then_block = region.append_block(Block::new(&then_phis));
                let if_else_block = region.append_block(Block::new(&else_phis));
                let end_block = region.append_block(Block::new(&end_phis));
                let cond = state.pop1()?;
                let (value, _) = state.peek1_extra()?;
                builder.create(cf::cond_br(
                    builder.context(),
                    cond,
                    &if_then_block,
                    &if_else_block,
                    &[value],
                    &[value],
                    builder.unknown_loc(),
                ));
                state.push_if(
                    if_then_block,
                    if_else_block,
                    end_block,
                    then_phis,
                    else_phis,
                    end_phis,
                );
            }
            Operator::Else => {
                if state.reachable {
                    let frame = state.frame_at_depth(0)?;
                    let values = state.peekn(frame.phis().len())?;
                    let frame = state.frame_at_depth(0)?;
                    builder.create(cf::br(
                        frame.code_after(),
                        &values,
                        builder.get_insert_location(),
                    ));
                }

                {
                    let (if_else_block, if_else_state) = if let ControlFrame::IfElse {
                        if_else,
                        if_else_state,
                        ..
                    } = state.frame_at_depth_mut(0)?
                    {
                        (if_else, if_else_state)
                    } else {
                        unreachable!()
                    };

                    *if_else_state = IfElseState::Else;
                }

                state.reachable = true;
                let frame = state.frame_at_depth(0)?;

                return Ok(*frame.code_after());
            }
            Operator::End => {
                let frame = state.pop_frame()?;
                let current_block = block;
                if state.reachable {
                    let values = state.peekn(frame.phis().len())?;
                    builder.create(cf::br(
                        frame.code_after(),
                        &values,
                        builder.get_insert_location(),
                    ));
                }

                if let ControlFrame::IfElse {
                    if_else,
                    next,
                    if_else_state: IfElseState::If,
                    else_phis,
                    ..
                } = &frame
                {
                    (*if_else).append_operation(cf::br(next, &[], builder.get_insert_location()));
                }
                state.reset_stack(&frame);
                state.reachable = true;

                return Ok(*frame.code_after());
            }
            Operator::Br { relative_depth } => {
                let frame = state.frame_at_depth(relative_depth)?;

                let phis = if frame.is_loop() {
                    frame.loop_body_phis()
                } else {
                    frame.phis()
                };

                let len = phis.len();
                let values = state.peekn(len)?;

                // For each result of the block we're branching to,
                // pop a value off the value stack and load it into
                // the corresponding phi.
                builder.create(cf::br(
                    frame.br_dest(),
                    &values,
                    builder.get_insert_location(),
                ));

                state.popn(len)?;
                state.reachable = false;
            }
            Operator::BrIf { relative_depth } => {
                let cond = state.pop1()?;
                let frame = state.frame_at_depth(relative_depth)?;
                let phis = if frame.is_loop() {
                    frame.loop_body_phis()
                } else {
                    frame.phis()
                };
                let param_stack = state.peekn(phis.len())?;
                let else_block = region.append_block(Block::new(phis));
                let cond_value = builder.create(builder.icmp_imm(IntCC::NotEqual, cond, 0)?);
                builder.create(cf::cond_br(
                    builder.context(),
                    cond_value.result(0)?.into(),
                    frame.br_dest(),
                    &else_block,
                    &param_stack,
                    &[],
                    builder.get_insert_location(),
                ));
                return Ok(else_block);
            }
            Operator::BrTable { targets } => {
                let index = state.pop1()?;
                let default_frame = state.frame_at_depth(targets.default())?;
                let phis = if default_frame.is_loop() {
                    default_frame.loop_body_phis()
                } else {
                    default_frame.phis()
                };
                let args = state.peekn(phis.len())?;

                let mut case_values = Vec::new();
                let mut case_destinations = Vec::new();
                for (case_index, depth) in targets.targets().enumerate() {
                    let depth = depth.map_err(from_binaryreadererror_wasmerror)?;
                    let frame_result = state.frame_at_depth(depth);
                    let frame = match frame_result {
                        Ok(v) => v,
                        Err(e) => return Err(e),
                    };
                    let case_index_literal = builder.make(builder.iconst_64(case_index as i64))?;
                    let phis = if frame.is_loop() {
                        frame.loop_body_phis()
                    } else {
                        frame.phis()
                    };
                    case_values.push(case_index as i64);
                    case_destinations.push(*frame.br_dest());
                }
                let case_destinations: Vec<_> = case_destinations
                    .iter()
                    .map(|b| {
                        let x: (&Block, &[Value]) = (b, &args);
                        x
                    })
                    .collect();

                builder.create(cf::switch(
                    builder.context(),
                    &case_values,
                    index,
                    builder.uint32_ty(),
                    (default_frame.br_dest(), &[]),
                    &case_destinations,
                    builder.get_insert_location(),
                )?);

                let args_len = args.len();
                state.popn(args_len)?;
                state.reachable = false;
            }
            Operator::Return => {
                let frame = state.outermost_frame()?;
                let values = state.peekn(frame.phis().len())?;
                builder.create(cf::br(
                    frame.br_dest(),
                    &values,
                    builder.get_insert_location(),
                ));
                state.reachable = false;
            }
            Operator::Call { function_index } => {
                let func_index = FunctionIndex::from_u32(function_index);
                let sigindex = fcx.wasm_module.functions[func_index];
                let wasm_func_type = &fcx.wasm_module.signatures[sigindex];
                let vm_ctx = fcx.ctx.vm_ctx;
                let (FunctionCache { func_type }, func_name) =
                    if let Some(local_func_index) = fcx.wasm_module.local_func_index(func_index) {
                        let func_name = match fcx.wasm_module.function_names.get(&func_index) {
                            Some(name) => name.to_string(),
                            None => fcx
                                .symbol_registry
                                .symbol_to_name(Symbol::LocalFunction(local_func_index)),
                        };
                        (
                            fcx.ctx.local_func(
                                local_func_index,
                                func_index,
                                wasm_func_type,
                                backend.ctx,
                                &backend.intrinsics,
                            )?,
                            func_name,
                        )
                    } else {
                        unimplemented!("wasm imported functions");
                    };
                let args = state.popn_save_extra(wasm_func_type.params().len())?;
                let args = args.iter().map(|p| p.0).collect::<Vec<Value<'_, '_>>>();
                let args = std::iter::once(vm_ctx)
                    .chain(args.iter().copied())
                    .collect::<Vec<Value<'_, '_>>>();
                let result_count = func_type.result_count();
                let result_types = (0..result_count)
                    .map(|i| func_type.result(i).unwrap())
                    .collect::<Vec<_>>();
                let op = builder.create(func::call(
                    &backend.ctx.mlir_context,
                    FlatSymbolRefAttribute::new(&backend.ctx.mlir_context, &func_name),
                    &args,
                    &result_types,
                    builder.get_insert_location(),
                ));
                for i in (0..result_count) {
                    let value = op.result(i)?.to_ctx_value();
                    state.push1(value);
                }
            }
            Operator::Drop => {
                state.pop1()?;
            }
            // `TypedSelect` must be used for extern refs so ref counting should
            // be done with TypedSelect. But otherwise they're the same.
            Operator::TypedSelect { .. } | Operator::Select => {
                let ((v1, i1), (v2, i2), (cond, _)) = state.pop3_extra()?;
                // If the pending bits of v1 and v2 are the same, we can pass
                // them along to the result. Otherwise, apply pending
                // canonicalizations now.
                let (v1, i1, v2, i2) = if i1.has_pending_f32_nan() != i2.has_pending_f32_nan()
                    || i1.has_pending_f64_nan() != i2.has_pending_f64_nan()
                {
                    (v1, i1.strip_pending(), v2, i2.strip_pending())
                } else {
                    (v1, i1, v2, i2)
                };
                let res = builder.make(
                    dora_ir::wasm::select(
                        builder.context(),
                        v1.r#type(),
                        v1,
                        v2,
                        cond,
                        builder.get_insert_location(),
                    )
                    .into(),
                )?;
                let info = {
                    let mut info = i1.strip_pending() & i2.strip_pending();
                    if i1.has_pending_f32_nan() {
                        debug_assert!(i2.has_pending_f32_nan());
                        info |= ExtraInfo::pending_f32_nan();
                    }
                    if i1.has_pending_f64_nan() {
                        debug_assert!(i2.has_pending_f64_nan());
                        info |= ExtraInfo::pending_f64_nan();
                    }
                    info
                };
                state.push1_extra(res.to_ctx_value(), info);
            }
            Operator::LocalGet { local_index } => {
                let (type_value, pointer_value) = fcx.locals[local_index as usize];
                let op = builder.load(pointer_value.to_ctx_value(), type_value);
                let v = builder.make(op)?;
                state.push1(v.to_ctx_value());
            }
            Operator::LocalSet { local_index } => {
                let pointer_value = fcx.locals[local_index as usize].1;
                let v = state.pop1()?;
                builder.create(builder.store(v, pointer_value));
            }
            Operator::LocalTee { local_index } => {
                let pointer_value = fcx.locals[local_index as usize].1;
                let v = state.peek1()?;
                builder.create(builder.store(v, pointer_value));
            }
            Operator::GlobalGet { global_index } => {
                let global_index = GlobalIndex::from_u32(global_index);
                match fcx
                    .ctx
                    .global(global_index, backend.ctx, &backend.intrinsics, block)?
                {
                    GlobalCache::Const { value } => {
                        state.push1(value.to_ctx_value());
                    }
                    GlobalCache::Mut {
                        ptr_to_value,
                        value_type,
                    } => {
                        let builder = OpBuilder::new_with_block(builder.context(), block);
                        let value = builder.make(builder.load(ptr_to_value, value_type))?;
                        state.push1(value.to_ctx_value());
                    }
                }
            }
            Operator::GlobalSet { global_index } => {
                let global_index = GlobalIndex::from_u32(global_index);
                match fcx
                    .ctx
                    .global(global_index, backend.ctx, &backend.intrinsics, block)?
                {
                    GlobalCache::Const { value: _ } => {
                        return Err(CompileError::Codegen(format!(
                            "global.set on immutable global index {}",
                            global_index.as_u32()
                        ))
                        .into())
                    }
                    GlobalCache::Mut { ptr_to_value, .. } => {
                        let value = state.pop1()?;
                        builder.create(builder.store(value, ptr_to_value));
                    }
                }
            }
            Operator::I32Load { memarg } => todo!(),
            Operator::I64Load { memarg } => todo!(),
            Operator::F32Load { memarg } => todo!(),
            Operator::F64Load { memarg } => todo!(),
            Operator::I32Load8S { memarg } => todo!(),
            Operator::I32Load8U { memarg } => todo!(),
            Operator::I32Load16S { memarg } => todo!(),
            Operator::I32Load16U { memarg } => todo!(),
            Operator::I64Load8S { memarg } => todo!(),
            Operator::I64Load8U { memarg } => todo!(),
            Operator::I64Load16S { memarg } => todo!(),
            Operator::I64Load16U { memarg } => todo!(),
            Operator::I64Load32S { memarg } => todo!(),
            Operator::I64Load32U { memarg } => todo!(),
            Operator::I32Store { memarg } => todo!(),
            Operator::I64Store { memarg } => todo!(),
            Operator::F32Store { memarg } => todo!(),
            Operator::F64Store { memarg } => todo!(),
            Operator::I32Store8 { memarg } => todo!(),
            Operator::I32Store16 { memarg } => todo!(),
            Operator::I64Store8 { memarg } => todo!(),
            Operator::I64Store16 { memarg } => todo!(),
            Operator::I64Store32 { memarg } => todo!(),
            Operator::I32Const { value } => {
                let i = builder
                    .create(builder.iconst_32(value))
                    .result(0)?
                    .to_ctx_value();
                let info = if is_f32_arithmetic(value as u32) {
                    ExtraInfo::arithmetic_f32()
                } else {
                    Default::default()
                };
                backend.state.push1_extra(i, info);
            }
            Operator::I64Const { value } => {
                let i = builder
                    .create(builder.iconst_64(value))
                    .result(0)?
                    .to_ctx_value();
                let info = if is_f64_arithmetic(value as u64) {
                    ExtraInfo::arithmetic_f64()
                } else {
                    Default::default()
                };
                backend.state.push1_extra(i, info);
            }
            Operator::F32Const { value } => {
                let bits = value.bits();
                let info = if is_f32_arithmetic(bits) {
                    ExtraInfo::arithmetic_f32()
                } else {
                    Default::default()
                };
                let f = builder
                    .create(builder.fconst_32(bits as f32))
                    .result(0)?
                    .to_ctx_value();
                backend.state.push1_extra(f, info);
            }
            Operator::F64Const { value } => {
                let bits = value.bits();
                let info = if is_f64_arithmetic(bits) {
                    ExtraInfo::arithmetic_f64()
                } else {
                    Default::default()
                };
                let f = builder
                    .create(builder.fconst_64(bits as f64))
                    .result(0)?
                    .to_ctx_value();
                backend.state.push1_extra(f, info);
            }
            /***************************
             * Reference types.
             * https://github.com/WebAssembly/reference-types/blob/master/proposals/reference-types/Overview.md
             ***************************/
            Operator::RefNull { hty } => {
                let ty = wpheaptype_to_type(hty)?;

                let attr = type_to_mlir_zero_attribute(builder.context(), &backend.intrinsics, ty);
                let value = builder
                    .create(arith::constant(
                        builder.context(),
                        attr,
                        builder.unknown_loc(),
                    ))
                    .result(0)?
                    .to_ctx_value();

                backend.state.push1(value);
            }
            Operator::RefIsNull => {
                let value = state.pop1()?;
                backend.state.push1(is_zero(&builder, value)?)
            }
            Operator::RefFunc { function_index } => {
                let index = builder.make(builder.iconst_32(function_index as i32))?;
                let value = builder
                    .make(func::call(
                        &backend.ctx.mlir_context,
                        FlatSymbolRefAttribute::new(
                            &backend.ctx.mlir_context,
                            symbols::wasm::FUNC_REF,
                        ),
                        &[fcx.ctx.vm_ctx, index],
                        &[builder.ptr_ty()],
                        builder.get_insert_location(),
                    ))?
                    .to_ctx_value();
                backend.state.push1(value);
            }
            Operator::I32Eqz => {
                op!(builder, state, eqz, i32);
            }
            Operator::I32Eq => {
                bin_op!(builder, state, eq, i32);
            }
            Operator::I32Ne => {
                bin_op!(builder, state, ne, i32);
            }
            Operator::I32LtS => {
                bin_op!(builder, state, lts, i32);
            }
            Operator::I32LtU => {
                bin_op!(builder, state, ltu, i32);
            }
            Operator::I32GtS => {
                bin_op!(builder, state, gts, i32);
            }
            Operator::I32GtU => {
                bin_op!(builder, state, gtu, i32);
            }
            Operator::I32LeS => {
                bin_op!(builder, state, les, i32);
            }
            Operator::I32LeU => {
                bin_op!(builder, state, leu, i32);
            }
            Operator::I32GeS => {
                bin_op!(builder, state, ges, i32);
            }
            Operator::I32GeU => {
                bin_op!(builder, state, geu, i32);
            }
            Operator::I64Eqz => {
                op!(builder, state, eqz, i64 => i32);
            }
            Operator::I64Eq => {
                bin_op!(builder, state, eq, i64 => i32);
            }
            Operator::I64Ne => {
                bin_op!(builder, state, ne, i64 => i32);
            }
            Operator::I64LtS => {
                bin_op!(builder, state, lts, i64 => i32);
            }
            Operator::I64LtU => {
                bin_op!(builder, state, ltu, i64 => i32);
            }
            Operator::I64GtS => {
                bin_op!(builder, state, gts, i64 => i32);
            }
            Operator::I64GtU => {
                bin_op!(builder, state, gtu, i64 => i32);
            }
            Operator::I64LeS => {
                bin_op!(builder, state, les, i64 => i32);
            }
            Operator::I64LeU => {
                bin_op!(builder, state, leu, i64 => i32);
            }
            Operator::I64GeS => {
                bin_op!(builder, state, ges, i64 => i32);
            }
            Operator::I64GeU => {
                bin_op!(builder, state, geu, i64 => i32);
            }
            Operator::F32Eq => {
                bin_op!(builder, state, eq, f32 => i32);
            }
            Operator::F32Ne => {
                bin_op!(builder, state, ne, f32 => i32);
            }
            Operator::F32Lt => {
                bin_op!(builder, state, lt, f32 => i32);
            }
            Operator::F32Gt => {
                bin_op!(builder, state, gt, f32 => i32);
            }
            Operator::F32Le => {
                bin_op!(builder, state, le, f32 => i32);
            }
            Operator::F32Ge => {
                bin_op!(builder, state, ge, f32 => i32);
            }
            Operator::F64Eq => {
                bin_op!(builder, state, eq, f64 => i32);
            }
            Operator::F64Ne => {
                bin_op!(builder, state, ne, f64 => i32);
            }
            Operator::F64Lt => {
                bin_op!(builder, state, lt, f64 => i32);
            }
            Operator::F64Gt => {
                bin_op!(builder, state, gt, f64 => i32);
            }
            Operator::F64Le => {
                bin_op!(builder, state, le, f64 => i32);
            }
            Operator::F64Ge => {
                bin_op!(builder, state, ge, f64 => i32);
            }
            Operator::I32Clz => {
                op!(builder, state, clz, i32);
            }
            Operator::I32Ctz => {
                op!(builder, state, ctz, i32);
            }
            Operator::I32Popcnt => {
                op!(builder, state, popcnt, i32);
            }
            Operator::I32Add => {
                bin_op!(builder, state, add, i32);
            }
            Operator::I32Sub => {
                bin_op!(builder, state, sub, i32);
            }
            Operator::I32Mul => {
                bin_op!(builder, state, mul, i32);
            }
            Operator::I32DivS => {
                bin_op!(builder, state, divs, i32);
            }
            Operator::I32DivU => {
                bin_op!(builder, state, divu, i32);
            }
            Operator::I32RemS => {
                bin_op!(builder, state, rems, i32);
            }
            Operator::I32RemU => {
                bin_op!(builder, state, remu, i32);
            }
            Operator::I32And => {
                bin_op!(builder, state, and, i32);
            }
            Operator::I32Or => {
                bin_op!(builder, state, or, i32);
            }
            Operator::I32Xor => {
                bin_op!(builder, state, xor, i32);
            }
            Operator::I32Shl => {
                bin_op!(builder, state, shl, i32);
            }
            Operator::I32ShrS => {
                bin_op!(builder, state, shrs, i32);
            }
            Operator::I32ShrU => {
                bin_op!(builder, state, shru, i32);
            }
            Operator::I32Rotl => {
                bin_op!(builder, state, rotl, i32);
            }
            Operator::I32Rotr => {
                bin_op!(builder, state, rotr, i32);
            }
            Operator::I64Clz => {
                op!(builder, state, clz, i64);
            }
            Operator::I64Ctz => {
                op!(builder, state, ctz, i64);
            }
            Operator::I64Popcnt => {
                op!(builder, state, popcnt, i64);
            }
            Operator::I64Add => {
                bin_op!(builder, state, add, i64);
            }
            Operator::I64Sub => {
                bin_op!(builder, state, sub, i64);
            }
            Operator::I64Mul => {
                bin_op!(builder, state, mul, i64);
            }
            Operator::I64DivS => {
                bin_op!(builder, state, divs, i64);
            }
            Operator::I64DivU => {
                bin_op!(builder, state, divu, i64);
            }
            Operator::I64RemS => {
                bin_op!(builder, state, rems, i64);
            }
            Operator::I64RemU => {
                bin_op!(builder, state, remu, i64);
            }
            Operator::I64And => {
                bin_op!(builder, state, and, i64);
            }
            Operator::I64Or => {
                bin_op!(builder, state, or, i64);
            }
            Operator::I64Xor => {
                bin_op!(builder, state, xor, i64);
            }
            Operator::I64Shl => {
                bin_op!(builder, state, shl, i64);
            }
            Operator::I64ShrS => {
                bin_op!(builder, state, shrs, i64);
            }
            Operator::I64ShrU => {
                bin_op!(builder, state, shru, i64);
            }
            Operator::I64Rotl => {
                bin_op!(builder, state, rotl, i64);
            }
            Operator::I64Rotr => {
                bin_op!(builder, state, rotr, i64);
            }
            Operator::F32Abs => {
                op!(builder, state, abs, f32);
            }
            Operator::F32Neg => {
                op!(builder, state, neg, f32);
            }
            Operator::F32Ceil => {
                op!(builder, state, ceil, f32);
            }
            Operator::F32Floor => {
                op!(builder, state, floor, f32);
            }
            Operator::F32Trunc => {
                op!(builder, state, trunc, f32);
            }
            Operator::F32Nearest => {
                op!(builder, state, nearest, f32);
            }
            Operator::F32Sqrt => {
                op!(builder, state, sqrt, f32);
            }
            Operator::F32Add => {
                bin_op!(builder, state, add, f32);
            }
            Operator::F32Sub => {
                bin_op!(builder, state, sub, f32);
            }
            Operator::F32Mul => {
                bin_op!(builder, state, mul, f32);
            }
            Operator::F32Div => {
                bin_op!(builder, state, div, f32);
            }
            Operator::F32Min => {
                bin_op!(builder, state, min, f32);
            }
            Operator::F32Max => {
                bin_op!(builder, state, max, f32);
            }
            Operator::F32Copysign => {
                bin_op!(builder, state, copysign, f32);
            }
            Operator::F64Abs => {
                op!(builder, state, abs, f64);
            }
            Operator::F64Neg => {
                op!(builder, state, neg, f64);
            }
            Operator::F64Ceil => {
                op!(builder, state, ceil, f64);
            }
            Operator::F64Floor => {
                op!(builder, state, floor, f64);
            }
            Operator::F64Trunc => {
                op!(builder, state, trunc, f64);
            }
            Operator::F64Nearest => {
                op!(builder, state, nearest, f64);
            }
            Operator::F64Sqrt => {
                op!(builder, state, sqrt, f64);
            }
            Operator::F64Add => {
                bin_op!(builder, state, add, f64);
            }
            Operator::F64Sub => {
                bin_op!(builder, state, sub, f64);
            }
            Operator::F64Mul => {
                bin_op!(builder, state, mul, f64);
            }
            Operator::F64Div => {
                bin_op!(builder, state, div, f64);
            }
            Operator::F64Min => {
                bin_op!(builder, state, min, f64);
            }
            Operator::F64Max => {
                bin_op!(builder, state, max, f64);
            }
            Operator::F64Copysign => {
                bin_op!(builder, state, copysign, f64);
            }
            /***************************
             * Conversion instructions.
             * https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md#conversion-instructions
             ***************************/
            Operator::I32WrapI64 => {
                op!(builder, state, i_32_wrap_i_64, i32);
            }
            Operator::I32TruncF32S => {
                op!(builder, state, i_32_trunc_f_32_s, i32);
            }
            Operator::I32TruncF32U => {
                op!(builder, state, i_32_trunc_f_32_u, i32);
            }
            Operator::I32TruncF64S => {
                op!(builder, state, i_32_trunc_f_64_s, i32);
            }
            Operator::I32TruncF64U => {
                op!(builder, state, i_32_trunc_f_64_u, i32);
            }
            Operator::I64ExtendI32S => {
                op!(builder, state, i_64_extend_i_32_s, i64);
            }
            Operator::I64ExtendI32U => {
                op!(builder, state, i_64_extend_i_32_u, i64);
            }
            Operator::I64TruncF32S => {
                op!(builder, state, i_64_trunc_f_32_s, i64);
            }
            Operator::I64TruncF32U => {
                op!(builder, state, i_64_trunc_f_32_u, i64);
            }
            Operator::I64TruncF64S => {
                op!(builder, state, i_64_trunc_f_64_s, i64);
            }
            Operator::I64TruncF64U => {
                op!(builder, state, i_64_trunc_f_64_u, i64);
            }
            Operator::F32ConvertI32S => {
                op!(builder, state, f_32_convert_i_32_s, f32);
            }
            Operator::F32ConvertI32U => {
                op!(builder, state, f_32_convert_i_32_u, f32);
            }
            Operator::F32ConvertI64S => {
                op!(builder, state, f_32_convert_i_64_s, f32);
            }
            Operator::F32ConvertI64U => {
                op!(builder, state, f_32_convert_i_64_u, f32);
            }
            Operator::F32DemoteF64 => {
                op!(builder, state, f_32_demote_f_64, f32);
            }
            Operator::F64ConvertI32S => {
                op!(builder, state, f_64_convert_i_32_s, f64);
            }
            Operator::F64ConvertI32U => {
                op!(builder, state, f_64_convert_i_32_u, f64);
            }
            Operator::F64ConvertI64S => {
                op!(builder, state, f_64_convert_i_64_s, f64);
            }
            Operator::F64ConvertI64U => {
                op!(builder, state, f_64_convert_i_64_u, f64);
            }
            Operator::F64PromoteF32 => {
                op!(builder, state, f_64_promote_f_32, f64);
            }
            Operator::I32ReinterpretF32 => {
                op!(builder, state, i_32_reinterpret_f_32, i32);
            }
            Operator::I64ReinterpretF64 => {
                op!(builder, state, i_64_reinterpret_f_64, i64);
            }
            Operator::F32ReinterpretI32 => {
                op!(builder, state, f_32_reinterpret_i_32, f32);
            }
            Operator::F64ReinterpretI64 => {
                op!(builder, state, f_64_reinterpret_i_64, f64);
            }
            Operator::I32Extend8S => {
                op!(builder, state, i_32_extend_8_s, i32);
            }
            Operator::I32Extend16S => {
                op!(builder, state, i_32_extend_16_s, i32);
            }
            Operator::I64Extend8S => {
                op!(builder, state, i_64_extend_8_s, i64);
            }
            Operator::I64Extend16S => {
                op!(builder, state, i_64_extend_16_s, i64);
            }
            Operator::I64Extend32S => {
                op!(builder, state, i_64_extend_32_s, i64);
            }
            Operator::StructNew { struct_type_index } => todo!(),
            Operator::StructNewDefault { struct_type_index } => todo!(),
            Operator::StructGet {
                struct_type_index,
                field_index,
            } => todo!(),
            Operator::StructGetS {
                struct_type_index,
                field_index,
            } => todo!(),
            Operator::StructGetU {
                struct_type_index,
                field_index,
            } => todo!(),
            Operator::StructSet {
                struct_type_index,
                field_index,
            } => todo!(),
            Operator::ArrayNew { array_type_index } => todo!(),
            Operator::ArrayNewDefault { array_type_index } => todo!(),
            Operator::ArrayNewFixed {
                array_type_index,
                array_size,
            } => todo!(),
            Operator::ArrayNewData {
                array_type_index,
                array_data_index,
            } => todo!(),
            Operator::ArrayNewElem {
                array_type_index,
                array_elem_index,
            } => todo!(),
            Operator::ArrayGet { array_type_index } => todo!(),
            Operator::ArrayGetS { array_type_index } => todo!(),
            Operator::ArrayGetU { array_type_index } => todo!(),
            Operator::ArraySet { array_type_index } => todo!(),
            Operator::ArrayLen => todo!(),
            Operator::ArrayFill { array_type_index } => todo!(),
            Operator::ArrayCopy {
                array_type_index_dst,
                array_type_index_src,
            } => todo!(),
            Operator::ArrayInitData {
                array_type_index,
                array_data_index,
            } => todo!(),
            Operator::ArrayInitElem {
                array_type_index,
                array_elem_index,
            } => todo!(),
            Operator::RefTestNonNull { hty } => todo!(),
            Operator::RefTestNullable { hty } => todo!(),
            Operator::RefCastNonNull { hty } => todo!(),
            Operator::RefCastNullable { hty } => todo!(),
            Operator::BrOnCast {
                relative_depth,
                from_ref_type,
                to_ref_type,
            } => todo!(),
            Operator::BrOnCastFail {
                relative_depth,
                from_ref_type,
                to_ref_type,
            } => todo!(),
            Operator::AnyConvertExtern => todo!(),
            Operator::ExternConvertAny => todo!(),
            Operator::RefI31 => todo!(),
            Operator::I31GetS => todo!(),
            Operator::I31GetU => todo!(),
            Operator::I32TruncSatF32S => {
                op!(builder, state, i_32_trunc_sat_f_32_s, i32);
            }
            Operator::I32TruncSatF32U => {
                op!(builder, state, i_32_trunc_sat_f_32_u, i32);
            }
            Operator::I32TruncSatF64S => {
                op!(builder, state, i_32_trunc_sat_f_64_s, i32);
            }
            Operator::I32TruncSatF64U => {
                op!(builder, state, i_32_trunc_sat_f_64_u, i32);
            }
            Operator::I64TruncSatF32S => {
                op!(builder, state, i_64_trunc_sat_f_32_s, i64);
            }
            Operator::I64TruncSatF32U => {
                op!(builder, state, i_64_trunc_sat_f_32_u, i64);
            }
            Operator::I64TruncSatF64S => {
                op!(builder, state, i_64_trunc_sat_f_64_s, i64);
            }
            Operator::I64TruncSatF64U => {
                op!(builder, state, i_64_trunc_sat_f_64_u, i64);
            }
            Operator::MemoryInit { data_index, mem } => {
                let (dest, src, len) = state.pop3()?;
                let mem = builder
                    .create(builder.iconst_32(mem as i32))
                    .result(0)?
                    .into();
                let segment = builder
                    .create(builder.iconst_32(data_index as i32))
                    .result(0)?
                    .into();

                builder.create(
                    dora_ir::wasm::mem_init(
                        builder.context(),
                        mem,
                        segment,
                        dest,
                        src,
                        len,
                        builder.unknown_loc(),
                    )
                    .into(),
                );
            }
            Operator::DataDrop { data_index } => {
                let segment = builder
                    .create(builder.iconst_32(data_index as i32))
                    .result(0)?
                    .into();

                builder.create(
                    dora_ir::wasm::data_drop(builder.context(), segment, builder.unknown_loc())
                        .into(),
                );
            }
            Operator::ElemDrop { elem_index } => {
                let segment = builder
                    .create(builder.iconst_32(elem_index as i32))
                    .result(0)?
                    .into();

                builder.create(
                    dora_ir::wasm::elem_drop(builder.context(), segment, builder.unknown_loc())
                        .into(),
                );
            }
            Operator::MemoryCopy { dst_mem, src_mem } => {
                // ignored until we support multiple memories
                let _dst = dst_mem;
                let (dest_pos, src_pos, len) = state.pop3()?;

                if let Some(local_memory_index) = fcx
                    .wasm_module
                    .local_memory_index(MemoryIndex::from_u32(src_mem))
                {
                    let src_index = builder
                        .create(builder.iconst_32(local_memory_index.as_u32() as i32))
                        .result(0)?
                        .into();
                    builder.create(
                        dora_ir::wasm::mem_copy(
                            builder.context(),
                            src_index,
                            dest_pos,
                            src_pos,
                            len,
                            builder.unknown_loc(),
                        )
                        .into(),
                    );
                } else {
                    let src_index = builder
                        .create(builder.iconst_32(src_mem as i32))
                        .result(0)?
                        .into();
                    builder.create(
                        dora_ir::wasm::imported_mem_copy(
                            builder.context(),
                            src_index,
                            dest_pos,
                            src_pos,
                            len,
                            builder.unknown_loc(),
                        )
                        .into(),
                    );
                };
            }
            Operator::MemoryFill { mem } => {
                let (dst, val, len) = state.pop3()?;

                if let Some(local_memory_index) = fcx
                    .wasm_module
                    .local_memory_index(MemoryIndex::from_u32(mem))
                {
                    let mem_index = builder
                        .create(builder.iconst_32(local_memory_index.as_u32() as i32))
                        .result(0)?
                        .into();
                    builder.create(
                        dora_ir::wasm::mem_copy(
                            builder.context(),
                            mem_index,
                            dst,
                            val,
                            len,
                            builder.unknown_loc(),
                        )
                        .into(),
                    );
                } else {
                    let mem_index = builder
                        .create(builder.iconst_32(mem as i32))
                        .result(0)?
                        .into();
                    builder.create(
                        dora_ir::wasm::mem_copy(
                            builder.context(),
                            mem_index,
                            dst,
                            val,
                            len,
                            builder.unknown_loc(),
                        )
                        .into(),
                    );
                };
            }
            Operator::TableInit { elem_index, table } => {
                let (dst, src, len) = state.pop3()?;
                let segment = builder
                    .create(builder.iconst_32(elem_index as i32))
                    .result(0)?
                    .into();
                let table = builder
                    .create(builder.iconst_32(table as i32))
                    .result(0)?
                    .into();
                builder.create(
                    dora_ir::wasm::table_init(
                        builder.context(),
                        table,
                        segment,
                        dst,
                        src,
                        len,
                        builder.unknown_loc(),
                    )
                    .into(),
                );
            }
            Operator::TableCopy {
                dst_table,
                src_table,
            } => {
                let (dst, src, len) = state.pop3()?;
                let dst_table = builder
                    .create(builder.iconst_32(dst_table as i32))
                    .result(0)?
                    .into();
                let src_table = builder
                    .create(builder.iconst_32(src_table as i32))
                    .result(0)?
                    .into();
                builder.create(
                    dora_ir::wasm::table_copy(
                        builder.context(),
                        dst_table,
                        src_table,
                        dst,
                        src,
                        len,
                        builder.unknown_loc(),
                    )
                    .into(),
                );
            }
            Operator::TableFill { table } => {
                let table = builder
                    .create(builder.iconst_32(table as i32))
                    .result(0)?
                    .into();
                let (start, elem, len) = state.pop3()?;
                builder.create(
                    dora_ir::wasm::table_fill(
                        builder.context(),
                        table,
                        start,
                        elem,
                        len,
                        builder.unknown_loc(),
                    )
                    .into(),
                );
            }
            Operator::TableGet { table } => {
                let table_index = builder
                    .create(builder.iconst_32(table as i32))
                    .result(0)?
                    .into();
                let elem = state.pop1()?;
                let op = if fcx
                    .wasm_module
                    .local_table_index(TableIndex::from_u32(table))
                    .is_some()
                {
                    builder.create(
                        dora_ir::wasm::table_get(
                            builder.context(),
                            table_index,
                            elem,
                            builder.unknown_loc(),
                        )
                        .into(),
                    )
                } else {
                    builder.create(
                        dora_ir::wasm::imported_table_get(
                            builder.context(),
                            table_index,
                            elem,
                            builder.unknown_loc(),
                        )
                        .into(),
                    )
                };
                let value = op.result(0)?.into();
                let op = builder.create(llvm::bitcast(
                    value,
                    type_to_mlir(
                        &backend.intrinsics,
                        &fcx.wasm_module
                            .tables
                            .get(TableIndex::from_u32(table))
                            .unwrap()
                            .ty,
                    ),
                    builder.unknown_loc(),
                ));
                let value = op.result(0)?.to_ctx_value();
                backend.state.push1(value);
            }
            Operator::TableSet { table } => {
                let table_index = builder
                    .create(builder.iconst_32(table as i32))
                    .result(0)?
                    .into();
                let (elem, value) = state.pop2()?;
                if fcx
                    .wasm_module
                    .local_table_index(TableIndex::from_u32(table))
                    .is_some()
                {
                    builder.create(
                        dora_ir::wasm::table_set(
                            builder.context(),
                            table_index,
                            elem,
                            value,
                            builder.unknown_loc(),
                        )
                        .into(),
                    )
                } else {
                    builder.create(
                        dora_ir::wasm::imported_table_set(
                            builder.context(),
                            table_index,
                            elem,
                            value,
                            builder.unknown_loc(),
                        )
                        .into(),
                    )
                };
            }
            Operator::TableGrow { table } => {
                let (elem, delta) = state.pop2()?;
                let op = if let Some(local_table_index) = fcx
                    .wasm_module
                    .local_table_index(TableIndex::from_u32(table))
                {
                    let table_index = builder
                        .create(builder.iconst_32(local_table_index.as_u32() as i32))
                        .result(0)?
                        .into();
                    builder.create(
                        dora_ir::wasm::table_grow(
                            builder.context(),
                            elem,
                            delta,
                            table_index,
                            builder.unknown_loc(),
                        )
                        .into(),
                    )
                } else {
                    let table_index = builder
                        .create(builder.iconst_32(table as i32))
                        .result(0)?
                        .into();
                    builder.create(
                        dora_ir::wasm::imported_table_grow(
                            builder.context(),
                            elem,
                            delta,
                            table_index,
                            builder.unknown_loc(),
                        )
                        .into(),
                    )
                };
                let size = op.result(0)?.to_ctx_value();
                backend.state.push1(size);
            }
            Operator::TableSize { table } => {
                let op = if let Some(local_table_index) = fcx
                    .wasm_module
                    .local_table_index(TableIndex::from_u32(table))
                {
                    let table_index = builder
                        .create(builder.iconst_32(local_table_index.as_u32() as i32))
                        .result(0)?
                        .into();
                    builder.create(
                        dora_ir::wasm::table_size(
                            builder.context(),
                            table_index,
                            builder.unknown_loc(),
                        )
                        .into(),
                    )
                } else {
                    let table_index = builder
                        .create(builder.iconst_32(table as i32))
                        .result(0)?
                        .into();
                    builder.create(
                        dora_ir::wasm::imported_table_size(
                            builder.context(),
                            table_index,
                            builder.unknown_loc(),
                        )
                        .into(),
                    )
                };
                let size = op.result(0)?.to_ctx_value();
                backend.state.push1(size);
            }
            Operator::MemorySize { mem } => {
                let mem = builder
                    .create(builder.iconst_32(mem as i32))
                    .result(0)?
                    .into();
                let op = builder.create(
                    dora_ir::wasm::mem_size(builder.context(), mem, builder.unknown_loc()).into(),
                );
                let size = op.result(0)?.to_ctx_value();
                backend.state.push1(size);
            }
            Operator::MemoryGrow { mem } => {
                let mem = builder
                    .create(builder.iconst_32(mem as i32))
                    .result(0)?
                    .into();
                let delta = state.pop1()?;
                let op = builder.create(
                    dora_ir::wasm::mem_grow(builder.context(), delta, mem, builder.unknown_loc())
                        .into(),
                );
                let size = op.result(0)?.to_ctx_value();
                backend.state.push1(size);
            }
            Operator::MemoryDiscard { mem } => todo!(),
            Operator::MemoryAtomicNotify { memarg } => todo!(),
            Operator::MemoryAtomicWait32 { memarg } => todo!(),
            Operator::MemoryAtomicWait64 { memarg } => todo!(),
            Operator::AtomicFence => {
                // Fence is a nop.
                //
                // Fence was added to preserve information about fences from
                // source languages. If in the future Wasm extends the memory
                // model, and if we hadn't recorded what fences used to be there,
                // it would lead to data races that weren't present in the
                // original source language.
            }
            Operator::I32AtomicLoad { memarg } => todo!(),
            Operator::I64AtomicLoad { memarg } => todo!(),
            Operator::I32AtomicLoad8U { memarg } => todo!(),
            Operator::I32AtomicLoad16U { memarg } => todo!(),
            Operator::I64AtomicLoad8U { memarg } => todo!(),
            Operator::I64AtomicLoad16U { memarg } => todo!(),
            Operator::I64AtomicLoad32U { memarg } => todo!(),
            Operator::I32AtomicStore { memarg } => todo!(),
            Operator::I64AtomicStore { memarg } => todo!(),
            Operator::I32AtomicStore8 { memarg } => todo!(),
            Operator::I32AtomicStore16 { memarg } => todo!(),
            Operator::I64AtomicStore8 { memarg } => todo!(),
            Operator::I64AtomicStore16 { memarg } => todo!(),
            Operator::I64AtomicStore32 { memarg } => todo!(),
            Operator::I32AtomicRmwAdd { memarg } => todo!(),
            Operator::I64AtomicRmwAdd { memarg } => todo!(),
            Operator::I32AtomicRmw8AddU { memarg } => todo!(),
            Operator::I32AtomicRmw16AddU { memarg } => todo!(),
            Operator::I64AtomicRmw8AddU { memarg } => todo!(),
            Operator::I64AtomicRmw16AddU { memarg } => todo!(),
            Operator::I64AtomicRmw32AddU { memarg } => todo!(),
            Operator::I32AtomicRmwSub { memarg } => todo!(),
            Operator::I64AtomicRmwSub { memarg } => todo!(),
            Operator::I32AtomicRmw8SubU { memarg } => todo!(),
            Operator::I32AtomicRmw16SubU { memarg } => todo!(),
            Operator::I64AtomicRmw8SubU { memarg } => todo!(),
            Operator::I64AtomicRmw16SubU { memarg } => todo!(),
            Operator::I64AtomicRmw32SubU { memarg } => todo!(),
            Operator::I32AtomicRmwAnd { memarg } => todo!(),
            Operator::I64AtomicRmwAnd { memarg } => todo!(),
            Operator::I32AtomicRmw8AndU { memarg } => todo!(),
            Operator::I32AtomicRmw16AndU { memarg } => todo!(),
            Operator::I64AtomicRmw8AndU { memarg } => todo!(),
            Operator::I64AtomicRmw16AndU { memarg } => todo!(),
            Operator::I64AtomicRmw32AndU { memarg } => todo!(),
            Operator::I32AtomicRmwOr { memarg } => todo!(),
            Operator::I64AtomicRmwOr { memarg } => todo!(),
            Operator::I32AtomicRmw8OrU { memarg } => todo!(),
            Operator::I32AtomicRmw16OrU { memarg } => todo!(),
            Operator::I64AtomicRmw8OrU { memarg } => todo!(),
            Operator::I64AtomicRmw16OrU { memarg } => todo!(),
            Operator::I64AtomicRmw32OrU { memarg } => todo!(),
            Operator::I32AtomicRmwXor { memarg } => todo!(),
            Operator::I64AtomicRmwXor { memarg } => todo!(),
            Operator::I32AtomicRmw8XorU { memarg } => todo!(),
            Operator::I32AtomicRmw16XorU { memarg } => todo!(),
            Operator::I64AtomicRmw8XorU { memarg } => todo!(),
            Operator::I64AtomicRmw16XorU { memarg } => todo!(),
            Operator::I64AtomicRmw32XorU { memarg } => todo!(),
            Operator::I32AtomicRmwXchg { memarg } => todo!(),
            Operator::I64AtomicRmwXchg { memarg } => todo!(),
            Operator::I32AtomicRmw8XchgU { memarg } => todo!(),
            Operator::I32AtomicRmw16XchgU { memarg } => todo!(),
            Operator::I64AtomicRmw8XchgU { memarg } => todo!(),
            Operator::I64AtomicRmw16XchgU { memarg } => todo!(),
            Operator::I64AtomicRmw32XchgU { memarg } => todo!(),
            Operator::I32AtomicRmwCmpxchg { memarg } => todo!(),
            Operator::I64AtomicRmwCmpxchg { memarg } => todo!(),
            Operator::I32AtomicRmw8CmpxchgU { memarg } => todo!(),
            Operator::I32AtomicRmw16CmpxchgU { memarg } => todo!(),
            Operator::I64AtomicRmw8CmpxchgU { memarg } => todo!(),
            Operator::I64AtomicRmw16CmpxchgU { memarg } => todo!(),
            Operator::I64AtomicRmw32CmpxchgU { memarg } => todo!(),
            Operator::V128Load { memarg } => todo!(),
            Operator::V128Load8x8S { memarg } => todo!(),
            Operator::V128Load8x8U { memarg } => todo!(),
            Operator::V128Load16x4S { memarg } => todo!(),
            Operator::V128Load16x4U { memarg } => todo!(),
            Operator::V128Load32x2S { memarg } => todo!(),
            Operator::V128Load32x2U { memarg } => todo!(),
            Operator::V128Load8Splat { memarg } => todo!(),
            Operator::V128Load16Splat { memarg } => todo!(),
            Operator::V128Load32Splat { memarg } => todo!(),
            Operator::V128Load64Splat { memarg } => todo!(),
            Operator::V128Load32Zero { memarg } => todo!(),
            Operator::V128Load64Zero { memarg } => todo!(),
            Operator::V128Store { memarg } => todo!(),
            Operator::V128Load8Lane { memarg, lane } => todo!(),
            Operator::V128Load16Lane { memarg, lane } => todo!(),
            Operator::V128Load32Lane { memarg, lane } => todo!(),
            Operator::V128Load64Lane { memarg, lane } => todo!(),
            Operator::V128Store8Lane { memarg, lane } => todo!(),
            Operator::V128Store16Lane { memarg, lane } => todo!(),
            Operator::V128Store32Lane { memarg, lane } => todo!(),
            Operator::V128Store64Lane { memarg, lane } => todo!(),
            Operator::V128Const { value } => todo!(),
            Operator::I8x16Shuffle { lanes } => todo!(),
            Operator::I8x16ExtractLaneS { lane } => todo!(),
            Operator::I8x16ExtractLaneU { lane } => todo!(),
            Operator::I8x16ReplaceLane { lane } => todo!(),
            Operator::I16x8ExtractLaneS { lane } => todo!(),
            Operator::I16x8ExtractLaneU { lane } => todo!(),
            Operator::I16x8ReplaceLane { lane } => todo!(),
            Operator::I32x4ExtractLane { lane } => todo!(),
            Operator::I32x4ReplaceLane { lane } => todo!(),
            Operator::I64x2ExtractLane { lane } => todo!(),
            Operator::I64x2ReplaceLane { lane } => todo!(),
            Operator::F32x4ExtractLane { lane } => todo!(),
            Operator::F32x4ReplaceLane { lane } => todo!(),
            Operator::F64x2ExtractLane { lane } => todo!(),
            Operator::F64x2ReplaceLane { lane } => todo!(),
            Operator::I8x16Swizzle => todo!(),
            Operator::I8x16Splat => todo!(),
            Operator::I16x8Splat => todo!(),
            Operator::I32x4Splat => todo!(),
            Operator::I64x2Splat => todo!(),
            Operator::F32x4Splat => todo!(),
            Operator::F64x2Splat => todo!(),
            Operator::I8x16Eq => todo!(),
            Operator::I8x16Ne => todo!(),
            Operator::I8x16LtS => todo!(),
            Operator::I8x16LtU => todo!(),
            Operator::I8x16GtS => todo!(),
            Operator::I8x16GtU => todo!(),
            Operator::I8x16LeS => todo!(),
            Operator::I8x16LeU => todo!(),
            Operator::I8x16GeS => todo!(),
            Operator::I8x16GeU => todo!(),
            Operator::I16x8Eq => todo!(),
            Operator::I16x8Ne => todo!(),
            Operator::I16x8LtS => todo!(),
            Operator::I16x8LtU => todo!(),
            Operator::I16x8GtS => todo!(),
            Operator::I16x8GtU => todo!(),
            Operator::I16x8LeS => todo!(),
            Operator::I16x8LeU => todo!(),
            Operator::I16x8GeS => todo!(),
            Operator::I16x8GeU => todo!(),
            Operator::I32x4Eq => todo!(),
            Operator::I32x4Ne => todo!(),
            Operator::I32x4LtS => todo!(),
            Operator::I32x4LtU => todo!(),
            Operator::I32x4GtS => todo!(),
            Operator::I32x4GtU => todo!(),
            Operator::I32x4LeS => todo!(),
            Operator::I32x4LeU => todo!(),
            Operator::I32x4GeS => todo!(),
            Operator::I32x4GeU => todo!(),
            Operator::I64x2Eq => todo!(),
            Operator::I64x2Ne => todo!(),
            Operator::I64x2LtS => todo!(),
            Operator::I64x2GtS => todo!(),
            Operator::I64x2LeS => todo!(),
            Operator::I64x2GeS => todo!(),
            Operator::F32x4Eq => todo!(),
            Operator::F32x4Ne => todo!(),
            Operator::F32x4Lt => todo!(),
            Operator::F32x4Gt => todo!(),
            Operator::F32x4Le => todo!(),
            Operator::F32x4Ge => todo!(),
            Operator::F64x2Eq => todo!(),
            Operator::F64x2Ne => todo!(),
            Operator::F64x2Lt => todo!(),
            Operator::F64x2Gt => todo!(),
            Operator::F64x2Le => todo!(),
            Operator::F64x2Ge => todo!(),
            Operator::V128Not => todo!(),
            Operator::V128And => todo!(),
            Operator::V128AndNot => todo!(),
            Operator::V128Or => todo!(),
            Operator::V128Xor => todo!(),
            Operator::V128Bitselect => todo!(),
            Operator::V128AnyTrue => todo!(),
            Operator::I8x16Abs => todo!(),
            Operator::I8x16Neg => todo!(),
            Operator::I8x16Popcnt => todo!(),
            Operator::I8x16AllTrue => todo!(),
            Operator::I8x16Bitmask => todo!(),
            Operator::I8x16NarrowI16x8S => todo!(),
            Operator::I8x16NarrowI16x8U => todo!(),
            Operator::I8x16Shl => todo!(),
            Operator::I8x16ShrS => todo!(),
            Operator::I8x16ShrU => todo!(),
            Operator::I8x16Add => todo!(),
            Operator::I8x16AddSatS => todo!(),
            Operator::I8x16AddSatU => todo!(),
            Operator::I8x16Sub => todo!(),
            Operator::I8x16SubSatS => todo!(),
            Operator::I8x16SubSatU => todo!(),
            Operator::I8x16MinS => todo!(),
            Operator::I8x16MinU => todo!(),
            Operator::I8x16MaxS => todo!(),
            Operator::I8x16MaxU => todo!(),
            Operator::I8x16AvgrU => todo!(),
            Operator::I16x8ExtAddPairwiseI8x16S => todo!(),
            Operator::I16x8ExtAddPairwiseI8x16U => todo!(),
            Operator::I16x8Abs => todo!(),
            Operator::I16x8Neg => todo!(),
            Operator::I16x8Q15MulrSatS => todo!(),
            Operator::I16x8AllTrue => todo!(),
            Operator::I16x8Bitmask => todo!(),
            Operator::I16x8NarrowI32x4S => todo!(),
            Operator::I16x8NarrowI32x4U => todo!(),
            Operator::I16x8ExtendLowI8x16S => todo!(),
            Operator::I16x8ExtendHighI8x16S => todo!(),
            Operator::I16x8ExtendLowI8x16U => todo!(),
            Operator::I16x8ExtendHighI8x16U => todo!(),
            Operator::I16x8Shl => todo!(),
            Operator::I16x8ShrS => todo!(),
            Operator::I16x8ShrU => todo!(),
            Operator::I16x8Add => todo!(),
            Operator::I16x8AddSatS => todo!(),
            Operator::I16x8AddSatU => todo!(),
            Operator::I16x8Sub => todo!(),
            Operator::I16x8SubSatS => todo!(),
            Operator::I16x8SubSatU => todo!(),
            Operator::I16x8Mul => todo!(),
            Operator::I16x8MinS => todo!(),
            Operator::I16x8MinU => todo!(),
            Operator::I16x8MaxS => todo!(),
            Operator::I16x8MaxU => todo!(),
            Operator::I16x8AvgrU => todo!(),
            Operator::I16x8ExtMulLowI8x16S => todo!(),
            Operator::I16x8ExtMulHighI8x16S => todo!(),
            Operator::I16x8ExtMulLowI8x16U => todo!(),
            Operator::I16x8ExtMulHighI8x16U => todo!(),
            Operator::I32x4ExtAddPairwiseI16x8S => todo!(),
            Operator::I32x4ExtAddPairwiseI16x8U => todo!(),
            Operator::I32x4Abs => todo!(),
            Operator::I32x4Neg => todo!(),
            Operator::I32x4AllTrue => todo!(),
            Operator::I32x4Bitmask => todo!(),
            Operator::I32x4ExtendLowI16x8S => todo!(),
            Operator::I32x4ExtendHighI16x8S => todo!(),
            Operator::I32x4ExtendLowI16x8U => todo!(),
            Operator::I32x4ExtendHighI16x8U => todo!(),
            Operator::I32x4Shl => todo!(),
            Operator::I32x4ShrS => todo!(),
            Operator::I32x4ShrU => todo!(),
            Operator::I32x4Add => todo!(),
            Operator::I32x4Sub => todo!(),
            Operator::I32x4Mul => todo!(),
            Operator::I32x4MinS => todo!(),
            Operator::I32x4MinU => todo!(),
            Operator::I32x4MaxS => todo!(),
            Operator::I32x4MaxU => todo!(),
            Operator::I32x4DotI16x8S => todo!(),
            Operator::I32x4ExtMulLowI16x8S => todo!(),
            Operator::I32x4ExtMulHighI16x8S => todo!(),
            Operator::I32x4ExtMulLowI16x8U => todo!(),
            Operator::I32x4ExtMulHighI16x8U => todo!(),
            Operator::I64x2Abs => todo!(),
            Operator::I64x2Neg => todo!(),
            Operator::I64x2AllTrue => todo!(),
            Operator::I64x2Bitmask => todo!(),
            Operator::I64x2ExtendLowI32x4S => todo!(),
            Operator::I64x2ExtendHighI32x4S => todo!(),
            Operator::I64x2ExtendLowI32x4U => todo!(),
            Operator::I64x2ExtendHighI32x4U => todo!(),
            Operator::I64x2Shl => todo!(),
            Operator::I64x2ShrS => todo!(),
            Operator::I64x2ShrU => todo!(),
            Operator::I64x2Add => todo!(),
            Operator::I64x2Sub => todo!(),
            Operator::I64x2Mul => todo!(),
            Operator::I64x2ExtMulLowI32x4S => todo!(),
            Operator::I64x2ExtMulHighI32x4S => todo!(),
            Operator::I64x2ExtMulLowI32x4U => todo!(),
            Operator::I64x2ExtMulHighI32x4U => todo!(),
            Operator::F32x4Ceil => todo!(),
            Operator::F32x4Floor => todo!(),
            Operator::F32x4Trunc => todo!(),
            Operator::F32x4Nearest => todo!(),
            Operator::F32x4Abs => todo!(),
            Operator::F32x4Neg => todo!(),
            Operator::F32x4Sqrt => todo!(),
            Operator::F32x4Add => todo!(),
            Operator::F32x4Sub => todo!(),
            Operator::F32x4Mul => todo!(),
            Operator::F32x4Div => todo!(),
            Operator::F32x4Min => todo!(),
            Operator::F32x4Max => todo!(),
            Operator::F32x4PMin => todo!(),
            Operator::F32x4PMax => todo!(),
            Operator::F64x2Ceil => todo!(),
            Operator::F64x2Floor => todo!(),
            Operator::F64x2Trunc => todo!(),
            Operator::F64x2Nearest => todo!(),
            Operator::F64x2Abs => todo!(),
            Operator::F64x2Neg => todo!(),
            Operator::F64x2Sqrt => todo!(),
            Operator::F64x2Add => todo!(),
            Operator::F64x2Sub => todo!(),
            Operator::F64x2Mul => todo!(),
            Operator::F64x2Div => todo!(),
            Operator::F64x2Min => todo!(),
            Operator::F64x2Max => todo!(),
            Operator::F64x2PMin => todo!(),
            Operator::F64x2PMax => todo!(),
            Operator::I32x4TruncSatF32x4S => todo!(),
            Operator::I32x4TruncSatF32x4U => todo!(),
            Operator::F32x4ConvertI32x4S => todo!(),
            Operator::F32x4ConvertI32x4U => todo!(),
            Operator::I32x4TruncSatF64x2SZero => todo!(),
            Operator::I32x4TruncSatF64x2UZero => todo!(),
            Operator::F64x2ConvertLowI32x4S => todo!(),
            Operator::F64x2ConvertLowI32x4U => todo!(),
            Operator::F32x4DemoteF64x2Zero => todo!(),
            Operator::F64x2PromoteLowF32x4 => todo!(),
            Operator::I8x16RelaxedSwizzle => todo!(),
            Operator::I32x4RelaxedTruncF32x4S => todo!(),
            Operator::I32x4RelaxedTruncF32x4U => todo!(),
            Operator::I32x4RelaxedTruncF64x2SZero => todo!(),
            Operator::I32x4RelaxedTruncF64x2UZero => todo!(),
            Operator::F32x4RelaxedMadd => todo!(),
            Operator::F32x4RelaxedNmadd => todo!(),
            Operator::F64x2RelaxedMadd => todo!(),
            Operator::F64x2RelaxedNmadd => todo!(),
            Operator::I8x16RelaxedLaneselect => todo!(),
            Operator::I16x8RelaxedLaneselect => todo!(),
            Operator::I32x4RelaxedLaneselect => todo!(),
            Operator::I64x2RelaxedLaneselect => todo!(),
            Operator::F32x4RelaxedMin => todo!(),
            Operator::F32x4RelaxedMax => todo!(),
            Operator::F64x2RelaxedMin => todo!(),
            Operator::F64x2RelaxedMax => todo!(),
            Operator::I16x8RelaxedQ15mulrS => todo!(),
            Operator::I16x8RelaxedDotI8x16I7x16S => todo!(),
            Operator::I32x4RelaxedDotI8x16I7x16AddS => todo!(),
            Operator::CallRef { type_index } => todo!(),
            Operator::ReturnCallRef { type_index } => todo!(),
            Operator::RefAsNonNull => todo!(),
            Operator::BrOnNull { relative_depth } => todo!(),
            Operator::BrOnNonNull { relative_depth } => todo!(),
            Operator::CallIndirect {
                type_index,
                table_index,
            } => todo!(),
            Operator::GlobalAtomicGet {
                ordering,
                global_index,
            } => todo!(),
            Operator::GlobalAtomicSet {
                ordering,
                global_index,
            } => todo!(),
            Operator::GlobalAtomicRmwAdd {
                ordering,
                global_index,
            } => todo!(),
            Operator::GlobalAtomicRmwSub {
                ordering,
                global_index,
            } => todo!(),
            Operator::GlobalAtomicRmwAnd {
                ordering,
                global_index,
            } => todo!(),
            Operator::GlobalAtomicRmwOr {
                ordering,
                global_index,
            } => todo!(),
            Operator::GlobalAtomicRmwXor {
                ordering,
                global_index,
            } => todo!(),
            Operator::GlobalAtomicRmwXchg {
                ordering,
                global_index,
            } => todo!(),
            Operator::GlobalAtomicRmwCmpxchg {
                ordering,
                global_index,
            } => todo!(),
            Operator::TableAtomicGet {
                ordering,
                table_index,
            } => todo!(),
            Operator::TableAtomicSet {
                ordering,
                table_index,
            } => todo!(),
            Operator::TableAtomicRmwXchg {
                ordering,
                table_index,
            } => todo!(),
            Operator::TableAtomicRmwCmpxchg {
                ordering,
                table_index,
            } => todo!(),
            Operator::StructAtomicGet {
                ordering,
                struct_type_index,
                field_index,
            } => todo!(),
            Operator::StructAtomicGetS {
                ordering,
                struct_type_index,
                field_index,
            } => todo!(),
            Operator::StructAtomicGetU {
                ordering,
                struct_type_index,
                field_index,
            } => todo!(),
            Operator::StructAtomicSet {
                ordering,
                struct_type_index,
                field_index,
            } => todo!(),
            Operator::StructAtomicRmwAdd {
                ordering,
                struct_type_index,
                field_index,
            } => todo!(),
            Operator::StructAtomicRmwSub {
                ordering,
                struct_type_index,
                field_index,
            } => todo!(),
            Operator::StructAtomicRmwAnd {
                ordering,
                struct_type_index,
                field_index,
            } => todo!(),
            Operator::StructAtomicRmwOr {
                ordering,
                struct_type_index,
                field_index,
            } => todo!(),
            Operator::StructAtomicRmwXor {
                ordering,
                struct_type_index,
                field_index,
            } => todo!(),
            Operator::StructAtomicRmwXchg {
                ordering,
                struct_type_index,
                field_index,
            } => todo!(),
            Operator::StructAtomicRmwCmpxchg {
                ordering,
                struct_type_index,
                field_index,
            } => todo!(),
            Operator::ArrayAtomicGet {
                ordering,
                array_type_index,
            } => todo!(),
            Operator::ArrayAtomicGetS {
                ordering,
                array_type_index,
            } => todo!(),
            Operator::ArrayAtomicGetU {
                ordering,
                array_type_index,
            } => todo!(),
            Operator::ArrayAtomicSet {
                ordering,
                array_type_index,
            } => todo!(),
            Operator::ArrayAtomicRmwAdd {
                ordering,
                array_type_index,
            } => todo!(),
            Operator::ArrayAtomicRmwSub {
                ordering,
                array_type_index,
            } => todo!(),
            Operator::ArrayAtomicRmwAnd {
                ordering,
                array_type_index,
            } => todo!(),
            Operator::ArrayAtomicRmwOr {
                ordering,
                array_type_index,
            } => todo!(),
            Operator::ArrayAtomicRmwXor {
                ordering,
                array_type_index,
            } => todo!(),
            Operator::ArrayAtomicRmwXchg {
                ordering,
                array_type_index,
            } => todo!(),
            Operator::ArrayAtomicRmwCmpxchg {
                ordering,
                array_type_index,
            } => todo!(),
            Operator::RefI31Shared => todo!(),
            _ => {
                return Err(
                    CompileError::Codegen(format!("Operator {:?} unimplemented", op)).into(),
                );
            }
        }
        Ok(block)
    }

    /// Finalizes the translation of a WebAssembly function by appending a return operation.
    ///
    /// This function is responsible for handling the final steps in the translation of a WebAssembly
    /// function to MLIR, including retrieving the result values from the backend's state and appending
    /// them as a return operation to the provided block. The function also ensures that the result types
    /// conform to both the WebAssembly function type and the MLIR function type.
    ///
    /// # Parameters
    /// - `backend`: A mutable reference to the `WASMBackend` which manages the context and translation state.
    /// - `block`: A reference to the block where the return operation will be appended.
    /// - `wasm_fn_type`: A reference to the WebAssembly `FunctionType` which provides the expected result types.
    /// - `fn_type`: A reference to the MLIR `FunctionType` which defines the corresponding MLIR-level function signature.
    ///
    /// # Returns
    /// A `Result` indicating whether the operation succeeded. On success, `Ok(())` is returned. On failure, an error is returned.
    ///
    /// # Errors
    /// This function may return an error if there is a failure in retrieving or processing the result values from the backend state.
    pub fn finalize<'c>(
        backend: &mut WASMBackend<'c>,
        block: BlockRef<'c, '_>,
        wasm_fn_type: &FunctionType,
        fn_type: &melior::ir::r#type::FunctionType,
    ) -> Result<()> {
        debug_assert!(block.argument_count() == wasm_fn_type.results().len());
        let mut results = vec![];
        for i in 0..block.argument_count() {
            results.push(block.argument(i)?.into());
        }
        block.append_operation(func::r#return(&results, backend.intrinsics.unknown_loc));
        Ok(())
    }
}
