use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::ops::Deref;

use crate::context::Context;
use crate::conversion::builder::OpBuilder;
use crate::intrinsics::Intrinsics;
use crate::value::ToContextValue;
use anyhow::Result;
use melior::dialect::arith;
use melior::ir::r#type::Type;
use melior::ir::{Value, ValueLike};
use melior::{dialect::llvm::r#type::r#struct, ir::r#type::FunctionType};
use wasmer::{LocalFunctionIndex, Mutability};
use wasmer_types::entity::PrimaryMap;
use wasmer_types::{
    FunctionIndex, GlobalIndex, ImportIndex, ImportKey, MemoryIndex, MemoryStyle, ModuleInfo,
    SignatureIndex, TableIndex, VMOffsets,
};

use super::ty::{func_type_to_mlir, type_to_mlir};

/// WASM magic number `\0asm` in array form.
pub const WASM_MAGIC_BYTES: [u8; 4] = [0x00, 0x61, 0x73, 0x6D];

/// Represents a set of WebAssembly (WASM) intrinsic types and utilities for interacting with
/// WebAssembly-specific constructs within the target intermediate representation (IR).
/// The `WASMIntrinsics` struct provides essential types like function references, external
/// references, and context types, along with common intrinsics inherited from the base `Intrinsics`.
///
/// # Fields:
/// - `ctx_ty`: The `Type` representing the context type in the WebAssembly runtime, typically
///   used to manage the execution environment for WebAssembly functions.
/// - `ctx_ptr_ty`: The pointer type to the context, used for passing the context in calls to
///   WebAssembly functions.
/// - `any_func_ty`: The `Type` representing a generic WebAssembly function type, which can be
///   used when a specific function signature is not known or is dynamic.
/// - `func_ref_ty`: The `Type` representing a WebAssembly function reference, used to refer to
///   functions within the module or imported functions.
/// - `extern_ref_ty`: The `Type` representing an external reference, used for passing references
///   to external objects into WebAssembly functions.
/// - `intrinsics`: A collection of common intrinsics that are inherited from the base `Intrinsics`
///   struct. These include shared low-level types and functions that are not specific to WebAssembly
///   but are still required for efficient code generation.
///
/// # Example Usage:
/// ```no_check
/// let wasm_intrinsics = WASMIntrinsics {
///     ctx_ty: /* WebAssembly context type */,
///     ctx_ptr_ty: /* Pointer to the WebAssembly context */,
///     any_func_ty: /* Type for any WebAssembly function */,
///     func_ref_ty: /* Function reference type */,
///     extern_ref_ty: /* External reference type */,
///     intrinsics: /* Common intrinsics inherited from base */,
/// };
///
/// // Use wasm_intrinsics to reference types in WebAssembly code generation.
/// let context_type = wasm_intrinsics.ctx_ty;
/// ```
///
/// # Notes:
/// - The `WASMIntrinsics` struct is crucial for providing type definitions and low-level operations
///   required when generating or interacting with WebAssembly code in the IR.
/// - The types provided by this struct are essential for modeling WebAssembly-specific behavior,
///   such as calling functions, referencing external entities, and managing the execution context.
/// - All pointers are opaque in LLVM 18.
#[derive(Debug)]
pub struct WASMIntrinsics<'c> {
    /// The type representing the WebAssembly execution context.
    pub ctx_ty: Type<'c>,
    /// The pointer type to the WebAssembly execution context.
    pub ctx_ptr_ty: Type<'c>,
    /// The type for representing any WebAssembly function.
    pub any_func_ty: Type<'c>,
    /// The type representing a reference to a WebAssembly function.
    pub func_ref_ty: Type<'c>,
    /// The type for representing a reference to an external object.
    pub extern_ref_ty: Type<'c>,
    /// Intrinsics inherited from the base `Intrinsics` structure.
    pub(crate) intrinsics: Intrinsics<'c>,
}

impl<'c> WASMIntrinsics<'c> {
    /// Create an [`Intrinsics`] for the given MLIR Builtin [`Context`].
    pub fn declare(context: &'c Context) -> Self {
        let intrinsics = Intrinsics::declare(context);
        let ctx_ty = intrinsics.i8_ty;
        let ctx_ptr_ty = intrinsics.ptr_ty;
        let sig_index_ty = intrinsics.i32_ty;
        let any_func_ty = r#struct(
            &context.mlir_context,
            &[intrinsics.ptr_ty, sig_index_ty, ctx_ptr_ty],
            false,
        );
        let func_ref_ty = intrinsics.ptr_ty;
        let extern_ref_ty = intrinsics.ptr_ty;
        Self {
            ctx_ty,
            ctx_ptr_ty,
            any_func_ty,
            func_ref_ty,
            extern_ref_ty,
            intrinsics,
        }
    }
}

impl<'c> Deref for WASMIntrinsics<'c> {
    type Target = Intrinsics<'c>;

    fn deref(&self) -> &Self::Target {
        &self.intrinsics
    }
}

#[derive(Clone, Copy)]
pub enum MemoryCache<'c, 'a> {
    /// The memory moves around.
    Dynamic {
        ptr_to_base_ptr: Value<'c, 'a>,
        ptr_to_current_length: Value<'c, 'a>,
    },
    /// The memory is always in the same place.
    Static { base_ptr: Value<'c, 'a> },
}

#[derive(Clone, Copy)]
struct TableCache<'c, 'a> {
    ptr_to_base_ptr: Value<'c, 'a>,
    ptr_to_bounds: Value<'c, 'a>,
}

#[derive(Clone, Copy)]
pub enum GlobalCache<'c, 'a> {
    Mut {
        ptr_to_value: Value<'c, 'a>,
        value_type: Type<'c>,
    },
    Const {
        value: Value<'c, 'a>,
    },
}

/// Represents a cache for a WebAssembly function during execution or compilation. The `FunctionCache`
/// struct holds a reference to the actual operation for the function and its associated function type.
///
/// # Fields:
/// - `func`: A reference to the `OperationRef`, representing the underlying operation for the WebAssembly
///   function in the intermediate representation.
/// - `func_type`: The type of the WebAssembly function, which includes its signature (parameter and return types).
///
/// # Example Usage:
/// ```no_check
/// let function_cache = FunctionCache {
///     func: /* OperationRef representing the function */,
///     func_type: /* FunctionType for the function signature */,
/// };
///
/// // Access the cached function's operation and type.
/// let operation_ref = function_cache.func;
/// let function_signature = function_cache.func_type;
/// ```
///
/// # Notes:
/// - The `FunctionCache` is used to optimize repeated access to WebAssembly functions by caching the operations
///   and function types during execution or code generation.
#[derive(Clone)]
pub struct FunctionCache<'c, 'a> {
    /// The type of the WebAssembly function, including its parameter and return types.
    pub func_type: FunctionType<'c>,
    /// The function body pointer value and the vm context value.
    pub func_with_vmctx: Option<(Value<'c, 'a>, Value<'c, 'a>)>,
}

/// Represents a collection of cached entities within a WebAssembly execution context. The `CtxType` struct
/// holds caches for various components such as memories, tables, signatures, globals, and functions, allowing
/// for efficient reuse and access during WebAssembly execution or compilation.
///
/// # Fields:
/// - `wasm_module`: A reference to the `ModuleInfo` that provides metadata and information about the WebAssembly module.
/// - `cached_memories`: A map caching the `MemoryCache` instances associated with the WebAssembly memories, indexed by `MemoryIndex`.
/// - `cached_tables`: A map caching the `TableCache` instances associated with the WebAssembly tables, indexed by `TableIndex`.
/// - `cached_sigindices`: A map caching signature indices as `Value` instances, allowing for efficient signature lookups.
/// - `cached_globals`: A map caching the `GlobalCache` instances associated with the WebAssembly globals, indexed by `GlobalIndex`.
/// - `cached_functions`: A map caching `FunctionCache` instances for WebAssembly functions, allowing efficient access to cached functions.
/// - `cached_memory_grow`: A map caching values for memory growth, indexed by `MemoryIndex`.
/// - `cached_memory_size`: A map caching values for memory size, indexed by `MemoryIndex`.
/// - `offsets`: Contains offsets for the various WebAssembly memory and table elements in the execution environment.
///
/// # Example Usage:
/// ```no_check
/// let ctx_type = CtxType {
///     wasm_module: /* Reference to the ModuleInfo */,
///     cached_memories: HashMap::new(),
///     cached_tables: HashMap::new(),
///     cached_sigindices: HashMap::new(),
///     cached_globals: HashMap::new(),
///     cached_functions: HashMap::new(),
///     cached_memory_grow: HashMap::new(),
///     cached_memory_size: HashMap::new(),
///     offsets: /* VMOffsets for memory and table element offsets */,
/// };
///
/// // Access the cached memory or function within the execution context.
/// let memory_cache = ctx_type.cached_memories.get(&memory_index);
/// let function_cache = ctx_type.cached_functions.get(&function_index);
/// ```
///
/// # Notes:
/// - The `CtxType` struct is essential for efficiently managing the state and resources of a WebAssembly
///   execution environment by caching frequently accessed elements like functions, memories, and tables.
pub struct CtxType<'c, 'a> {
    /// The WASM vm context value used during the execution.
    pub vm_ctx: Value<'c, 'c>,
    /// The builder used to generate cache instructions.
    cache_builder: OpBuilder<'c, 'a>,
    /// A reference to the WebAssembly module information, providing metadata about the module.
    wasm_module: &'a ModuleInfo,
    /// A cache of WebAssembly memories, indexed by `MemoryIndex`.
    cached_memories: HashMap<MemoryIndex, MemoryCache<'c, 'a>>,
    /// A cache of WebAssembly tables, indexed by `TableIndex`.
    cached_tables: HashMap<TableIndex, TableCache<'c, 'a>>,
    /// A cache of signature indices as `Value` instances.
    cached_sigindices: HashMap<SignatureIndex, Value<'c, 'a>>,
    /// A cache of WebAssembly globals, indexed by `GlobalIndex`.
    cached_globals: HashMap<GlobalIndex, GlobalCache<'c, 'a>>,
    /// A cache of WebAssembly functions, indexed by `FunctionIndex`.
    cached_functions: HashMap<FunctionIndex, FunctionCache<'c, 'a>>,
    /// Contains the offsets for memory and table elements within the WebAssembly execution context.
    pub(crate) offsets: VMOffsets,
}

impl<'c, 'a> CtxType<'c, 'a> {
    pub fn new(
        vm_ctx: Value<'c, 'c>,
        wasm_module: &'a ModuleInfo,
        cache_builder: OpBuilder<'c, 'a>,
    ) -> CtxType<'c, 'a> {
        CtxType {
            vm_ctx,
            cache_builder,
            wasm_module,
            cached_memories: HashMap::new(),
            cached_tables: HashMap::new(),
            cached_sigindices: HashMap::new(),
            cached_globals: HashMap::new(),
            cached_functions: HashMap::new(),
            offsets: VMOffsets::new(8, wasm_module),
        }
    }

    pub(crate) fn local_func(
        &mut self,
        _local_function_index: LocalFunctionIndex,
        function_index: FunctionIndex,
        func_type: &wasmer::FunctionType,
        ctx: &'c Context,
        intrinsics: &WASMIntrinsics<'c>,
    ) -> Result<&FunctionCache<'c, 'a>> {
        let cached_functions = &mut self.cached_functions;
        Ok(match cached_functions.entry(function_index) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let func_type = func_type_to_mlir(ctx, intrinsics, func_type);
                entry.insert(FunctionCache {
                    func_type,
                    func_with_vmctx: None,
                })
            }
        })
    }

    pub(crate) fn func(
        &mut self,
        function_index: FunctionIndex,
        func_type: &wasmer::FunctionType,
        ctx: &'c Context,
        intrinsics: &WASMIntrinsics<'c>,
    ) -> Result<&FunctionCache<'c, 'a>> {
        let cached_functions = &mut self.cached_functions;
        Ok(match cached_functions.entry(function_index) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let builder = &self.cache_builder;
                let func_type = func_type_to_mlir(ctx, intrinsics, func_type);
                debug_assert!(self.wasm_module.local_func_index(function_index).is_none());
                let offset = self.offsets.vmctx_vmfunction_import(function_index);
                let vmfunction_import_ptr = builder.make(builder.gep(
                    self.vm_ctx,
                    offset as usize,
                    builder.i8_ty(),
                    builder.ptr_ty(),
                ))?;
                let body_ptr_ptr = builder.make(builder.gep(
                    vmfunction_import_ptr,
                    self.offsets.vmfunction_import_body() as usize,
                    builder.i8_ty(),
                    builder.ptr_ty(),
                ))?;
                let body_ptr = builder.make(builder.load(body_ptr_ptr, builder.ptr_ty()))?;
                let vmctx_ptr_ptr = builder.make(builder.gep(
                    vmfunction_import_ptr,
                    self.offsets.vmfunction_import_vmctx() as usize,
                    builder.i8_ty(),
                    builder.ptr_ty(),
                ))?;
                let vmctx_ptr = builder.make(builder.load(vmctx_ptr_ptr, builder.ptr_ty()))?;

                entry.insert(FunctionCache {
                    func_type,
                    func_with_vmctx: Some(unsafe {
                        (
                            Value::from_raw(body_ptr.to_raw()),
                            Value::from_raw(vmctx_ptr.to_raw()),
                        )
                    }),
                })
            }
        })
    }

    pub(crate) fn table_prepare(
        &mut self,
        table_index: TableIndex,
    ) -> Result<(Value<'c, 'a>, Value<'c, 'a>)> {
        let (cached_tables, wasm_module, offsets) =
            (&mut self.cached_tables, self.wasm_module, &self.offsets);
        let TableCache {
            ptr_to_base_ptr,
            ptr_to_bounds,
        } = match cached_tables.entry(table_index) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let (ptr_to_base_ptr, ptr_to_bounds) =
                    if let Some(local_table_index) = wasm_module.local_table_index(table_index) {
                        let builder = &self.cache_builder;
                        let ptr_to_base_ptr = builder.make(builder.gep(
                            self.vm_ctx,
                            offsets.vmctx_vmtable_definition_base(local_table_index) as usize,
                            builder.i8_ty(),
                            builder.ptr_ty(),
                        ))?;
                        let ptr_to_bounds = builder.make(builder.gep(
                            self.vm_ctx,
                            offsets.vmctx_vmtable_definition_current_elements(local_table_index)
                                as usize,
                            builder.i8_ty(),
                            builder.ptr_ty(),
                        ))?;
                        (ptr_to_base_ptr.to_ctx_value(), ptr_to_bounds.to_ctx_value())
                    } else {
                        let builder = &self.cache_builder;
                        let definition_ptr_ptr = builder
                            .make(builder.gep(
                                self.vm_ctx,
                                offsets.vmctx_vmtable_import_definition(table_index) as usize,
                                builder.i8_ty(),
                                builder.ptr_ty(),
                            ))?
                            .to_ctx_value();
                        let definition_ptr =
                            builder.make(builder.load(definition_ptr_ptr, builder.ptr_ty()))?;
                        let ptr_to_base_ptr = builder.make(builder.gep(
                            definition_ptr.to_ctx_value(),
                            offsets.vmtable_definition_base() as usize,
                            builder.i8_ty(),
                            builder.ptr_ty(),
                        ))?;
                        let ptr_to_bounds = builder.make(builder.gep(
                            definition_ptr.to_ctx_value(),
                            offsets.vmtable_definition_current_elements() as usize,
                            builder.i8_ty(),
                            builder.ptr_ty(),
                        ))?;
                        (ptr_to_base_ptr.to_ctx_value(), ptr_to_bounds.to_ctx_value())
                    };

                let v = TableCache {
                    ptr_to_base_ptr: ptr_to_base_ptr.to_ctx_value(),
                    ptr_to_bounds: ptr_to_bounds.to_ctx_value(),
                };

                entry.insert(v);

                v
            }
        };
        unsafe {
            Ok((
                Value::from_raw(ptr_to_base_ptr.to_raw()),
                Value::from_raw(ptr_to_bounds.to_raw()),
            ))
        }
    }

    pub(crate) fn table(&mut self, index: TableIndex) -> Result<(Value<'c, 'a>, Value<'c, 'a>)> {
        let (ptr_to_base_ptr, ptr_to_bounds) = self.table_prepare(index)?;
        let builder = &self.cache_builder;
        let base_ptr = builder.make(builder.load(ptr_to_base_ptr, builder.ptr_ty()))?;
        let mut bounds = builder.make(builder.load(ptr_to_bounds, builder.isize_ty()))?;
        if builder.int_ty_width(bounds.r#type())? != 32 {
            bounds = builder.make(arith::trunci(
                bounds,
                builder.i32_ty(),
                builder.get_insert_location(),
            ))?;
        }
        unsafe {
            Ok((
                Value::from_raw(base_ptr.to_raw()),
                Value::from_raw(bounds.to_raw()),
            ))
        }
    }

    pub(crate) fn dynamic_sigindex(&mut self, index: SignatureIndex) -> Result<Value<'c, 'a>> {
        let (cached_sigindices, ctx_ptr_value, offsets) =
            (&mut self.cached_sigindices, self.vm_ctx, &self.offsets);

        match cached_sigindices.entry(index) {
            Entry::Occupied(entry) => {
                let value = *entry.get();
                Ok(unsafe { Value::from_raw(value.to_raw()) })
            }
            Entry::Vacant(entry) => {
                let builder = &self.cache_builder;
                let sigindex_ptr = builder
                    .make(builder.gep(
                        ctx_ptr_value,
                        offsets.vmctx_vmshared_signature_id(index) as usize,
                        builder.i8_ty(),
                        builder.ptr_ty(),
                    ))?
                    .to_ctx_value();
                let sigindex = builder.make(builder.load(sigindex_ptr, builder.i32_ty()))?;
                entry.insert(sigindex.to_ctx_value());
                Ok(unsafe { Value::from_raw(sigindex.to_raw()) })
            }
        }
    }

    pub(crate) fn global(
        &mut self,
        index: GlobalIndex,
        intrinsics: &WASMIntrinsics<'c>,
    ) -> Result<GlobalCache<'c, 'a>> {
        let (cached_globals, wasm_module, ctx_ptr_value, offsets) = (
            &mut self.cached_globals,
            self.wasm_module,
            self.vm_ctx,
            &self.offsets,
        );
        if let Entry::Vacant(entry) = cached_globals.entry(index) {
            let global_type = wasm_module.globals[index];
            let global_value_type = global_type.ty;

            let global_mutability = global_type.mutability;
            let offset = if let Some(local_global_index) = wasm_module.local_global_index(index) {
                offsets.vmctx_vmglobal_definition(local_global_index)
            } else {
                offsets.vmctx_vmglobal_import(index)
            };
            let builder = &self.cache_builder;
            let global = {
                let global_ptr = {
                    let global_ptr_ptr = builder
                        .make(builder.gep(
                            ctx_ptr_value,
                            offset as usize,
                            builder.i8_ty(),
                            builder.ptr_ty(),
                        ))?
                        .to_ctx_value();
                    let global_ptr = builder
                        .make(builder.load(global_ptr_ptr, builder.ptr_ty()))?
                        .to_ctx_value();
                    global_ptr
                };
                match global_mutability {
                    Mutability::Const => {
                        let value = builder
                            .make(
                                builder
                                    .load(global_ptr, type_to_mlir(intrinsics, &global_value_type)),
                            )?
                            .to_ctx_value();
                        GlobalCache::Const { value }
                    }
                    Mutability::Var => GlobalCache::Mut {
                        ptr_to_value: global_ptr,
                        value_type: type_to_mlir(intrinsics, &global_value_type),
                    },
                }
            };
            entry.insert(global);
        }
        self.cached_globals
            .get(&index)
            .ok_or_else(|| anyhow::anyhow!("wasm global {} not found", index.as_u32()))
            .cloned()
    }

    pub(crate) fn memory(
        &mut self,
        index: MemoryIndex,
        memory_styles: &PrimaryMap<MemoryIndex, MemoryStyle>,
    ) -> Result<MemoryCache<'c, 'a>> {
        let (cached_memories, wasm_module, ctx_ptr_value, offsets) = (
            &mut self.cached_memories,
            self.wasm_module,
            self.vm_ctx,
            &self.offsets,
        );
        let memory_style = &memory_styles[index];
        match cached_memories.get(&index) {
            Some(r) => Ok(*r),
            None => {
                let builder = &self.cache_builder;
                let memory_definition_ptr =
                    if let Some(local_memory_index) = wasm_module.local_memory_index(index) {
                        let offset = offsets.vmctx_vmmemory_definition(local_memory_index);
                        builder
                            .make(builder.gep(
                                ctx_ptr_value,
                                offset as usize,
                                builder.i8_ty(),
                                builder.ptr_ty(),
                            ))?
                            .to_ctx_value()
                    } else {
                        let offset = offsets.vmctx_vmmemory_import(index);
                        let memory_definition_ptr_ptr = builder
                            .make(builder.gep(
                                ctx_ptr_value,
                                offset as usize,
                                builder.i8_ty(),
                                builder.ptr_ty(),
                            ))?
                            .to_ctx_value();
                        builder
                            .make(builder.load(memory_definition_ptr_ptr, builder.ptr_ty()))?
                            .to_ctx_value()
                    };
                let base_ptr = builder
                    .make(builder.gep(
                        memory_definition_ptr,
                        offsets.vmmemory_definition_base() as usize,
                        builder.i8_ty(),
                        builder.ptr_ty(),
                    ))?
                    .to_ctx_value();
                let value = if let MemoryStyle::Dynamic { .. } = memory_style {
                    let current_length_ptr = builder
                        .make(builder.gep(
                            memory_definition_ptr,
                            offsets.vmmemory_definition_current_length() as usize,
                            builder.i8_ty(),
                            builder.ptr_ty(),
                        ))?
                        .to_ctx_value();
                    MemoryCache::Dynamic {
                        ptr_to_base_ptr: base_ptr,
                        ptr_to_current_length: current_length_ptr,
                    }
                } else {
                    let base_ptr = builder
                        .make(builder.load(base_ptr, builder.ptr_ty()))?
                        .to_ctx_value();
                    MemoryCache::Static { base_ptr }
                };

                self.cached_memories.insert(index, value);
                Ok(*self.cached_memories.get(&index).unwrap())
            }
        }
    }

    pub(crate) fn get_import_function_info(&self, function_index: u32) -> Option<&ImportKey> {
        for (import_key, import_index) in &self.wasm_module.imports {
            if let ImportIndex::Function(index) = import_index {
                if function_index == index.as_u32() {
                    return Some(import_key);
                }
            }
        }
        None
    }
}
