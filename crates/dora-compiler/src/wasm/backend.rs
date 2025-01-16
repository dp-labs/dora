use dora_ir::IRTypes;
use dora_runtime::symbols;
use melior::dialect::{
    arith::{self, CmpfPredicate, CmpiPredicate},
    cf, func,
};
use melior::ir::{
    attribute::FlatSymbolRefAttribute, r#type::IntegerType, BlockRef, OperationRef, Region, Type,
    TypeLike, Value, ValueLike,
};
use wasmer_types::TrapCode;

use crate::backend::IntCC;
use crate::errors::Result;
use crate::value::ToContextValue;
use crate::{backend::TypeMethods, context::Context, conversion::builder::OpBuilder, state::State};

use super::intrinsics::WASMIntrinsics;

/// A builder structure for constructing WebAssembly (WASM) operations and managing
/// the underlying compilation process using an `OpBuilder`. The `WASMBuilder`
/// operates on a `WASMBackend` and provides an interface to emit operations
/// within the context of a given WASM execution model.
///
/// # Fields:
/// - `backend`: A mutable reference to the `WASMBackend`, which contains the overall
///   context and state of the WASM environment being built.
/// - `builder`: An `OpBuilder` instance responsible for generating operations
///   during the construction process.
///
/// # Example Usage:
/// ```no_check
/// let mut wasm_builder = WASMBuilder {
///     backend: &mut wasm_backend,
///     builder: op_builder,
/// };
/// wasm_builder.builder.some_build_method();
/// ```
///
/// # Notes:
/// - The `WASMBuilder` works closely with the `WASMBackend` to manage WebAssembly-specific
///   context and state, allowing for the construction of WASM operations within the MLIR system.
#[derive(Debug)]
pub struct WASMBuilder<'c, 'a> {
    /// A mutable reference to the WASM backend containing the context and state.
    pub backend: &'a mut WASMBackend<'c>,
    /// The builder used to generate operations within the WASM execution context.
    pub builder: OpBuilder<'c, 'a>,
}

impl<'c> std::ops::Deref for WASMBuilder<'c, '_> {
    type Target = WASMBackend<'c>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.backend
    }
}

impl std::ops::DerefMut for WASMBuilder<'_, '_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.backend
    }
}

/// The backend structure for managing the WebAssembly (WASM) execution context and
/// intrinsics. `WASMBackend` encapsulates the relevant MLIR context, intrinsic types,
/// and execution state required for compiling and executing WASM bytecode.
///
/// # Fields:
/// - `ctx`: A reference to the MLIR `Context`, which provides the underlying infrastructure
///   for managing the compilation and execution process.
/// - `intrinsics`: A set of WebAssembly-specific intrinsic types, such as i32, i64, f32, and f64,
///   that are commonly used in WASM operations.
/// - `state`: The current state of the execution, which includes the stack and control
///   structures required for managing the flow of WASM bytecode execution.
///
/// # Example Usage:
/// ```no_check
/// let wasm_backend = WASMBackend {
///     ctx: &mlir_context,
///     intrinsics: wasm_intrinsics,
///     state: wasm_state,
/// };
/// ```
///
/// # Notes:
/// - The `WASMBackend` is responsible for maintaining the state and context necessary
///   for WASM execution and compilation. It interacts with other structures like the
///   `WASMBuilder` to facilitate the construction of operations that adhere to the
///   WebAssembly execution model.
#[derive(Debug)]
pub struct WASMBackend<'c> {
    /// A reference to the MLIR context used in the WASM backend.
    pub ctx: &'c Context,
    /// WebAssembly-specific intrinsic types, such as i32, i64, f32, and f64.
    pub intrinsics: WASMIntrinsics<'c>,
    /// The current state of WASM execution, including the stack and control structures.
    pub state: State<'c, 'c>,
}

impl<'c> WASMBackend<'c> {
    pub fn new(ctx: &'c Context) -> Self {
        Self {
            ctx,
            intrinsics: WASMIntrinsics::declare(ctx),
            state: State::new(),
        }
    }
}

impl<'c> IRTypes for WASMBackend<'c> {
    type Type = Type<'c>;
    type Value = Value<'c, 'c>;
    type Region = Region<'c>;
    type BasicBlock = BlockRef<'c, 'c>;
    type Operation = OperationRef<'c, 'c>;
}

impl TypeMethods for WASMBackend<'_> {
    fn type_ptr(&self) -> Self::Type {
        self.intrinsics.ptr_ty
    }

    fn type_int(&self, bits: u32) -> Self::Type {
        IntegerType::new(&self.ctx.mlir_context, bits).into()
    }
}

pub(crate) fn is_zero<'c, 'a>(
    builder: &OpBuilder<'c, 'a>,
    value: Value<'c, 'a>,
) -> Result<Value<'c, 'a>> {
    let ty = value.r#type();
    if ty.is_integer() {
        Ok(builder
            .create(arith::cmpi(
                builder.context(),
                CmpiPredicate::Eq,
                value,
                if builder.int_ty_width(ty)? == 32 {
                    builder.create(builder.iconst_32(0)).result(0)?.into()
                } else {
                    builder.create(builder.iconst_64(0)).result(0)?.into()
                },
                builder.unknown_loc(),
            ))
            .result(0)?
            .to_ctx_value())
    } else if ty.is_float() {
        Ok(builder
            .create(arith::cmpf(
                builder.context(),
                CmpfPredicate::Oeq,
                value,
                if ty.is_f32() {
                    builder.create(builder.fconst_32(0.0)).result(0)?.into()
                } else {
                    builder.create(builder.fconst_64(0.0)).result(0)?.into()
                },
                builder.unknown_loc(),
            ))
            .result(0)?
            .to_ctx_value())
    } else {
        // WASM pointer type
        let value = builder.make(builder.load(value, builder.i32_ty()))?;
        let result = builder.make(builder.icmp_imm(IntCC::Equal, value, 0)?)?;
        Ok(unsafe { Value::from_raw(result.to_raw()) })
    }
}

pub(crate) fn trap<'c, 'a>(
    builder: &OpBuilder<'c, 'a>,
    code: TrapCode,
    continue_block: BlockRef<'c, 'a>,
) -> Result<()> {
    let ctx = builder.ctx;
    let code = builder.make(builder.iconst_32(code as _))?;
    builder.create(func::call(
        ctx,
        FlatSymbolRefAttribute::new(ctx, symbols::wasm::RAISE_TRAP),
        &[code],
        &[],
        builder.get_insert_location(),
    ));
    builder.create(cf::br(&continue_block, &[], builder.get_insert_location()));
    Ok(())
}
