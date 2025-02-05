use crate::conversion::rewriter::{replace_op, Replacer, Rewriter};
use crate::conversion::walker::walk_operation;
use crate::errors::Result;
use crate::value::IntoContextOperation;
use dora_ir;
use dora_runtime::constants::{
    GEF32_LEQ_I32_MAX, GEF32_LEQ_I64_MAX, GEF32_LEQ_U32_MAX, GEF32_LEQ_U64_MAX, GEF64_LEQ_I32_MAX,
    GEF64_LEQ_I64_MAX, GEF64_LEQ_U32_MAX, GEF64_LEQ_U64_MAX, LEF32_GEQ_I32_MIN, LEF32_GEQ_I64_MIN,
    LEF32_GEQ_U32_MIN, LEF32_GEQ_U64_MIN, LEF64_GEQ_I32_MIN, LEF64_GEQ_I64_MIN, LEF64_GEQ_U32_MIN,
    LEF64_GEQ_U64_MIN,
};
use melior::dialect::{arith, llvm, ods::math};
use melior::ir::{operation::OperationBuilder, Operation, TypeLike, ValueLike};
use melior::{
    dialect::DialectHandle,
    ir::{r#type::TypeId, OperationRef},
    pass::{create_external, ExternalPass, Pass, RunExternalPass},
    Context, ContextRef,
};
use tracing::debug;

use super::backend::trunc_sat_scalar;

#[repr(align(8))]
struct PassId;
static CONVERSION_PASS: PassId = PassId;

/// The `ConversionPass` struct is responsible for transforming specific WASM
/// operations into their Dora counterparts. It walks through the MLIR operation
/// tree, identifies certain WASM instructions, and replaces them with Dora operations.
///
/// This transformation is essential in a system where WebAssembly or other intermediate representations
/// are being processed with additional virtual machine layers, providing flexibility and extensibility.
///
/// # Struct Details:
/// The `ConversionPass` struct is typically used as part of a larger pass framework where WASM bytecode
/// or MLIR IR is being converted into an intermediate representation supported by Dora. This pass
/// focuses on replacing arithmetic and logical instructions.
///
/// ## WASM to Dora Operation Transformations like:
/// - `wasm.add` → `dora.add`
/// - `wasm.sub` → `dora.sub`
/// - `wasm.mul` → `dora.mul`
/// - `wasm.div_s` → `dora.sdiv`
/// - `wasm.div_u` → `dora.div`
///
/// These transformations optimize or translate WASM-specific instructions into a format compatible
/// with Dora or other runtime environments.
///
/// # Errors:
/// This conversion pass returns a `Result<()>`, with possible errors occurring during operand retrieval
/// or operation replacement.
///
/// # Methods:
/// - `run(&mut self, operation: OperationRef<'_, '_>) -> Result<()>`
///   - Walks through the operation tree and applies transformations to recognized WASM operators.
#[derive(Clone, Debug)]
pub struct ConversionPass<'c> {
    /// A reference to the MLIR context, which manages global state and resources required for MLIR operations.
    pub ctx: &'c Context,
}

impl ConversionPass<'_> {
    /// Executes the conversion pass on the provided operation.
    ///
    /// This function transforms specific WASM operators into equivalent
    /// Dora operations. It walks through the operation, checks for
    /// recognized WASM instructions (such as `wasm.add`, `wasm.sub`, etc.), and replaces them with their
    /// corresponding Dora counterparts (like `dora.add`, `dora.sub`, etc.).
    ///
    /// # Parameters:
    /// - `operation`: The root `OperationRef` representing the operation to be transformed.
    ///
    /// # Errors:
    /// This function returns an error if there are issues in retrieving operands or replacing
    /// the operation with the new Dora operations.
    pub(crate) fn run(&mut self, operation: OperationRef<'_, '_>) -> Result<()> {
        let mut wasm_ops = vec![];
        walk_operation(
            operation,
            Box::new(|op| {
                let name = op.name().as_string_ref().as_str().unwrap().to_string();
                if name.starts_with("wasm") {
                    wasm_ops.push(op.to_ctx_operation_ref());
                }
                Ok(())
            }),
        )?;
        for op in wasm_ops {
            let name = op.name().as_string_ref().as_str().unwrap().to_string();
            let rewriter = Rewriter::new_with_op(self.ctx, op);
            if name == dora_ir::wasm::UnreachableOperation::name() {
                replace_op(op, llvm::unreachable(op.location()));
            } else if name == dora_ir::wasm::AddOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::add(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into(),
                );
            } else if name == dora_ir::wasm::SubOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::sub(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into(),
                );
            } else if name == dora_ir::wasm::MulOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::mul(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into(),
                );
            } else if name == dora_ir::wasm::DivSOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::sdiv(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into(),
                );
            } else if name == dora_ir::wasm::DivUOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::div(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into(),
                );
            } else if name == dora_ir::wasm::RemUOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::r#mod(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into(),
                );
            } else if name == dora_ir::wasm::RemSOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::smod(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into(),
                );
            } else if name == dora_ir::wasm::AndOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::and(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into(),
                );
            } else if name == dora_ir::wasm::OrOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::or(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into(),
                );
            } else if name == dora_ir::wasm::XOrOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::xor(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into(),
                );
            } else if name == dora_ir::wasm::ShlOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::shl(self.ctx, lhs.r#type(), rhs, lhs, op.location()).into(),
                );
            } else if name == dora_ir::wasm::ShrSOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::sar(self.ctx, lhs.r#type(), rhs, lhs, op.location()).into(),
                );
            } else if name == dora_ir::wasm::ShrUOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::shr(self.ctx, lhs.r#type(), rhs, lhs, op.location()).into(),
                );
            } else if name == dora_ir::wasm::RotlOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::rotl(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into(),
                );
            } else if name == dora_ir::wasm::RotrOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::rotr(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into(),
                );
            } else if name == dora_ir::wasm::ClzOperation::name() {
                let value = op.operand(0)?;
                replace_op(
                    op,
                    dora_ir::dora::clz(self.ctx, value.r#type(), value, op.location()).into(),
                );
            } else if name == dora_ir::wasm::CtzOperation::name() {
                let value = op.operand(0)?;
                replace_op(
                    op,
                    dora_ir::dora::ctz(self.ctx, value.r#type(), value, op.location()).into(),
                );
            } else if name == dora_ir::wasm::PopcntOperation::name() {
                let value = op.operand(0)?;
                replace_op(
                    op,
                    dora_ir::dora::popcnt(self.ctx, value.r#type(), value, op.location()).into(),
                );
            } else if name == dora_ir::wasm::I32WrapI64Operation::name() {
                let value = op.operand(0)?;
                replace_op(op, arith::trunci(value, rewriter.i32_ty(), op.location()));
            } else if name == dora_ir::wasm::I32TruncF32SOperation::name()
                || name == dora_ir::wasm::I32TruncF64SOperation::name()
            {
                let value = op.operand(0)?;
                replace_op(op, arith::fptosi(value, rewriter.i32_ty(), op.location()));
            } else if name == dora_ir::wasm::I32TruncF32UOperation::name()
                || name == dora_ir::wasm::I32TruncF64UOperation::name()
            {
                let value = op.operand(0)?;
                replace_op(op, arith::fptoui(value, rewriter.i32_ty(), op.location()));
            } else if name == dora_ir::wasm::I64TruncF32SOperation::name()
                || name == dora_ir::wasm::I64TruncF64SOperation::name()
            {
                let value = op.operand(0)?;
                replace_op(op, arith::fptosi(value, rewriter.i64_ty(), op.location()));
            } else if name == dora_ir::wasm::I64TruncF32UOperation::name()
                || name == dora_ir::wasm::I64TruncF64UOperation::name()
            {
                let value = op.operand(0)?;
                replace_op(op, arith::fptoui(value, rewriter.i64_ty(), op.location()));
            } else if name == dora_ir::wasm::F32ConvertI64UOperation::name()
                || name == dora_ir::wasm::F32ConvertI32UOperation::name()
            {
                let value = op.operand(0)?;
                replace_op(op, arith::uitofp(value, rewriter.f32_ty(), op.location()));
            } else if name == dora_ir::wasm::F64ConvertI64UOperation::name()
                || name == dora_ir::wasm::F64ConvertI32UOperation::name()
            {
                let value = op.operand(0)?;
                replace_op(op, arith::uitofp(value, rewriter.f64_ty(), op.location()));
            } else if name == dora_ir::wasm::F32ConvertI64SOperation::name()
                || name == dora_ir::wasm::F32ConvertI32SOperation::name()
            {
                let value = op.operand(0)?;
                replace_op(op, arith::sitofp(value, rewriter.f32_ty(), op.location()));
            } else if name == dora_ir::wasm::F32DemoteF64Operation::name() {
                let value = op.operand(0)?;
                let new_op = OperationBuilder::new("arith.truncf", op.location())
                    .add_operands(&[value])
                    .add_results(&[op.result(0)?.r#type()])
                    .build()?;
                replace_op(op, new_op);
            } else if name == dora_ir::wasm::F64ConvertI64SOperation::name()
                || name == dora_ir::wasm::F64ConvertI32SOperation::name()
            {
                let value = op.operand(0)?;
                replace_op(op, arith::sitofp(value, rewriter.f64_ty(), op.location()));
            } else if name == dora_ir::wasm::F64PromoteF32Operation::name() {
                let value = op.operand(0)?;
                replace_op(op, arith::extf(value, rewriter.f64_ty(), op.location()));
            } else if name == dora_ir::wasm::I32ReinterpretF32Operation::name() {
                let value = op.operand(0)?;
                replace_op(op, arith::bitcast(value, rewriter.i32_ty(), op.location()));
            } else if name == dora_ir::wasm::I64ReinterpretF64Operation::name() {
                let value = op.operand(0)?;
                replace_op(op, arith::bitcast(value, rewriter.i64_ty(), op.location()));
            } else if name == dora_ir::wasm::I32ReinterpretF32Operation::name() {
                let value = op.operand(0)?;
                replace_op(op, arith::bitcast(value, rewriter.i32_ty(), op.location()));
            } else if name == dora_ir::wasm::F32ReinterpretI32Operation::name() {
                let value = op.operand(0)?;
                replace_op(op, arith::bitcast(value, rewriter.f32_ty(), op.location()));
            } else if name == dora_ir::wasm::F64ReinterpretI64Operation::name() {
                let value = op.operand(0)?;
                replace_op(op, arith::bitcast(value, rewriter.f64_ty(), op.location()));
            } else if name == dora_ir::wasm::I64ExtendI32UOperation::name() {
                let value = op.operand(0)?;
                replace_op(op, arith::extui(value, rewriter.i64_ty(), op.location()));
            } else if name == dora_ir::wasm::I64ExtendI32SOperation::name() {
                let value = op.operand(0)?;
                replace_op(op, arith::extsi(value, rewriter.i64_ty(), op.location()));
            } else if name == dora_ir::wasm::I32Extend8SOperation::name() {
                let value = op.operand(0)?;
                let value = rewriter.make(arith::trunci(value, rewriter.i8_ty(), op.location()))?;
                replace_op(op, arith::extsi(value, rewriter.i32_ty(), op.location()));
            } else if name == dora_ir::wasm::I32Extend16SOperation::name() {
                let value = op.operand(0)?;
                let value =
                    rewriter.make(arith::trunci(value, rewriter.i16_ty(), op.location()))?;
                replace_op(op, arith::extsi(value, rewriter.i32_ty(), op.location()));
            } else if name == dora_ir::wasm::I64Extend8SOperation::name() {
                let value = op.operand(0)?;
                let value = rewriter.make(arith::trunci(value, rewriter.i8_ty(), op.location()))?;
                replace_op(op, arith::extsi(value, rewriter.i64_ty(), op.location()));
            } else if name == dora_ir::wasm::I64Extend16SOperation::name() {
                let value = op.operand(0)?;
                let value =
                    rewriter.make(arith::trunci(value, rewriter.i16_ty(), op.location()))?;
                replace_op(op, arith::extsi(value, rewriter.i64_ty(), op.location()));
            } else if name == dora_ir::wasm::I64Extend32SOperation::name() {
                let value = op.operand(0)?;
                let value =
                    rewriter.make(arith::trunci(value, rewriter.i32_ty(), op.location()))?;
                replace_op(op, arith::extsi(value, rewriter.i64_ty(), op.location()));
            } else if name == dora_ir::wasm::I32TruncSatF32SOperation::name() {
                let value = op.operand(0)?;
                let value = trunc_sat_scalar(
                    &rewriter,
                    rewriter.i32_ty(),
                    LEF32_GEQ_I32_MIN,
                    GEF32_LEQ_I32_MAX,
                    i32::MIN as u32 as u64,
                    i32::MAX as u32 as u64,
                    value,
                )?;
                rewriter.replace_op(
                    op,
                    arith::bitcast(value, rewriter.i32_ty(), rewriter.get_insert_location()),
                )?;
            } else if name == dora_ir::wasm::I32TruncSatF32UOperation::name() {
                let value = op.operand(0)?;
                let value = trunc_sat_scalar(
                    &rewriter,
                    rewriter.i32_ty(),
                    LEF32_GEQ_U32_MIN,
                    GEF32_LEQ_U32_MAX,
                    u32::MIN as u64,
                    u32::MAX as u64,
                    value,
                )?;
                rewriter.replace_op(
                    op,
                    arith::bitcast(value, rewriter.i32_ty(), rewriter.get_insert_location()),
                )?;
            } else if name == dora_ir::wasm::I64TruncSatF32SOperation::name() {
                let value = op.operand(0)?;
                let value = trunc_sat_scalar(
                    &rewriter,
                    rewriter.i64_ty(),
                    LEF32_GEQ_I64_MIN,
                    GEF32_LEQ_I64_MAX,
                    i64::MIN as u64,
                    i64::MAX as u64,
                    value,
                )?;
                rewriter.replace_op(
                    op,
                    arith::bitcast(value, rewriter.i64_ty(), rewriter.get_insert_location()),
                )?;
            } else if name == dora_ir::wasm::I32TruncSatF64SOperation::name() {
                let value = op.operand(0)?;
                let value = trunc_sat_scalar(
                    &rewriter,
                    rewriter.i32_ty(),
                    LEF64_GEQ_I32_MIN,
                    GEF64_LEQ_I32_MAX,
                    i32::MIN as u64,
                    i32::MAX as u64,
                    value,
                )?;
                rewriter.replace_op(
                    op,
                    arith::bitcast(value, rewriter.i32_ty(), rewriter.get_insert_location()),
                )?;
            } else if name == dora_ir::wasm::I64TruncSatF32UOperation::name() {
                let value = op.operand(0)?;
                let value = trunc_sat_scalar(
                    &rewriter,
                    rewriter.i64_ty(),
                    LEF32_GEQ_U64_MIN,
                    GEF32_LEQ_U64_MAX,
                    u64::MIN,
                    u64::MAX,
                    value,
                )?;
                rewriter.replace_op(
                    op,
                    arith::bitcast(value, rewriter.i64_ty(), rewriter.get_insert_location()),
                )?;
            } else if name == dora_ir::wasm::I64TruncSatF64SOperation::name() {
                let value = op.operand(0)?;
                let value = trunc_sat_scalar(
                    &rewriter,
                    rewriter.i64_ty(),
                    LEF64_GEQ_I64_MIN,
                    GEF64_LEQ_I64_MAX,
                    i64::MIN as u64,
                    i64::MAX as u64,
                    value,
                )?;
                rewriter.replace_op(
                    op,
                    arith::bitcast(value, rewriter.i64_ty(), rewriter.get_insert_location()),
                )?;
            } else if name == dora_ir::wasm::I32TruncSatF64UOperation::name() {
                let value = op.operand(0)?;
                let value = trunc_sat_scalar(
                    &rewriter,
                    rewriter.i32_ty(),
                    LEF64_GEQ_U32_MIN,
                    GEF64_LEQ_U32_MAX,
                    u32::MIN as u64,
                    u32::MAX as u64,
                    value,
                )?;
                rewriter.replace_op(
                    op,
                    arith::bitcast(value, rewriter.i32_ty(), rewriter.get_insert_location()),
                )?;
            } else if name == dora_ir::wasm::I64TruncSatF64UOperation::name() {
                let value = op.operand(0)?;
                let value = trunc_sat_scalar(
                    &rewriter,
                    rewriter.i64_ty(),
                    LEF64_GEQ_U64_MIN,
                    GEF64_LEQ_U64_MAX,
                    u64::MIN,
                    u64::MAX,
                    value,
                )?;
                rewriter.replace_op(
                    op,
                    arith::bitcast(value, rewriter.i64_ty(), rewriter.get_insert_location()),
                )?;
            } else if name == dora_ir::wasm::EqzOperation::name() {
                let value = op.operand(0)?;
                let is_zero_op =
                    dora_ir::dora::iszero(self.ctx, op.result(0)?.r#type(), value, op.location())
                        .into();
                // Note: the wasm eqz operation always return i32 value.
                replace_op(op, is_zero_op);
            } else if name == dora_ir::wasm::EqOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                let ty = op.result(0)?.r#type();
                let eq_op = dora_ir::dora::eq(self.ctx, ty, lhs, rhs, op.location()).into();
                // Note: the wasm eq operation always return i32 value.
                replace_op(op, eq_op);
            } else if name == dora_ir::wasm::NeOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                let eq = rewriter.make(
                    dora_ir::dora::eq(self.ctx, op.result(0)?.r#type(), lhs, rhs, op.location())
                        .into(),
                )?;
                let not_op: Operation =
                    dora_ir::dora::not(self.ctx, eq.r#type(), eq, op.location()).into();
                let ty = not_op.result(0)?.r#type();
                let is_i32 = ty.is_integer() && rewriter.int_ty_width(ty)? == 32;
                // Note: the wasm ne operation always return i32 value.
                if is_i32 {
                    replace_op(op, not_op);
                } else {
                    let not_value = rewriter.make(not_op)?;
                    replace_op(
                        op,
                        arith::trunci(not_value, rewriter.i32_ty(), rewriter.get_insert_location()),
                    );
                }
            } else if name == dora_ir::wasm::FltOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::slt(self.ctx, rewriter.i32_ty(), lhs, rhs, op.location()).into(),
                );
            } else if name == dora_ir::wasm::FgtOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::sgt(self.ctx, rewriter.i32_ty(), lhs, rhs, op.location()).into(),
                );
            } else if name == dora_ir::wasm::FleOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::slte(self.ctx, rewriter.i32_ty(), lhs, rhs, op.location())
                        .into(),
                );
            } else if name == dora_ir::wasm::FgeOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::sgte(self.ctx, rewriter.i32_ty(), lhs, rhs, op.location())
                        .into(),
                );
            } else if name == dora_ir::wasm::LeuOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                let val_op =
                    dora_ir::dora::lte(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into();
                let is_i32 =
                    lhs.r#type().is_integer() && rewriter.int_ty_width(lhs.r#type())? == 32;
                // Note: the wasm ne operation always return i32 value.
                if is_i32 {
                    replace_op(op, val_op);
                } else {
                    let lte_value = rewriter.make(val_op)?;
                    replace_op(
                        op,
                        arith::trunci(lte_value, rewriter.i32_ty(), rewriter.get_insert_location()),
                    );
                }
            } else if name == dora_ir::wasm::LesOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                let val_op =
                    dora_ir::dora::slte(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into();
                let is_i32 =
                    lhs.r#type().is_integer() && rewriter.int_ty_width(lhs.r#type())? == 32;
                // Note: the wasm ne operation always return i32 value.
                if is_i32 {
                    replace_op(op, val_op);
                } else {
                    let lte_value = rewriter.make(val_op)?;
                    replace_op(
                        op,
                        arith::trunci(lte_value, rewriter.i32_ty(), rewriter.get_insert_location()),
                    );
                }
            } else if name == dora_ir::wasm::LtsOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                let val_op =
                    dora_ir::dora::slt(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into();
                let is_i32 =
                    lhs.r#type().is_integer() && rewriter.int_ty_width(lhs.r#type())? == 32;
                // Note: the wasm ne operation always return i32 value.
                if is_i32 {
                    replace_op(op, val_op);
                } else {
                    let lte_value = rewriter.make(val_op)?;
                    replace_op(
                        op,
                        arith::trunci(lte_value, rewriter.i32_ty(), rewriter.get_insert_location()),
                    );
                }
            } else if name == dora_ir::wasm::LtuOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                let val_op =
                    dora_ir::dora::lt(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into();
                let is_i32 =
                    lhs.r#type().is_integer() && rewriter.int_ty_width(lhs.r#type())? == 32;
                // Note: the wasm ne operation always return i32 value.
                if is_i32 {
                    replace_op(op, val_op);
                } else {
                    let lte_value = rewriter.make(val_op)?;
                    replace_op(
                        op,
                        arith::trunci(lte_value, rewriter.i32_ty(), rewriter.get_insert_location()),
                    );
                }
            } else if name == dora_ir::wasm::GeuOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                let val_op =
                    dora_ir::dora::gte(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into();
                let is_i32 =
                    lhs.r#type().is_integer() && rewriter.int_ty_width(lhs.r#type())? == 32;
                // Note: the wasm ne operation always return i32 value.
                if is_i32 {
                    replace_op(op, val_op);
                } else {
                    let lte_value = rewriter.make(val_op)?;
                    replace_op(
                        op,
                        arith::trunci(lte_value, rewriter.i32_ty(), rewriter.get_insert_location()),
                    );
                }
            } else if name == dora_ir::wasm::GesOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                let val_op =
                    dora_ir::dora::sgte(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into();
                let is_i32 =
                    lhs.r#type().is_integer() && rewriter.int_ty_width(lhs.r#type())? == 32;
                // Note: the wasm ne operation always return i32 value.
                if is_i32 {
                    replace_op(op, val_op);
                } else {
                    let lte_value = rewriter.make(val_op)?;
                    replace_op(
                        op,
                        arith::trunci(lte_value, rewriter.i32_ty(), rewriter.get_insert_location()),
                    );
                }
            } else if name == dora_ir::wasm::GtsOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                let val_op =
                    dora_ir::dora::sgt(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into();
                let is_i32 =
                    lhs.r#type().is_integer() && rewriter.int_ty_width(lhs.r#type())? == 32;
                // Note: the wasm ne operation always return i32 value.
                if is_i32 {
                    replace_op(op, val_op);
                } else {
                    let lte_value = rewriter.make(val_op)?;
                    replace_op(
                        op,
                        arith::trunci(lte_value, rewriter.i32_ty(), rewriter.get_insert_location()),
                    );
                }
            } else if name == dora_ir::wasm::GtuOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                let val_op =
                    dora_ir::dora::gt(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into();
                let is_i32 =
                    lhs.r#type().is_integer() && rewriter.int_ty_width(lhs.r#type())? == 32;
                // Note: the wasm ne operation always return i32 value.
                if is_i32 {
                    replace_op(op, val_op);
                } else {
                    let lte_value = rewriter.make(val_op)?;
                    replace_op(
                        op,
                        arith::trunci(lte_value, rewriter.i32_ty(), rewriter.get_insert_location()),
                    );
                }
            } else if name == dora_ir::wasm::SelectOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                let cond = op.operand(2)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::select(
                        self.ctx,
                        op.result(0)?.r#type(),
                        lhs,
                        rhs,
                        cond,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == dora_ir::wasm::SqrtOperation::name() {
                let value = op.operand(0)?;
                replace_op(op, math::sqrt(self.ctx, value, op.location()).into());
            }
        }
        Ok(())
    }
}

impl<'c> RunExternalPass<'c> for ConversionPass<'c> {
    fn construct(&mut self) {}

    fn destruct(&mut self) {}

    fn initialize(&mut self, _context: ContextRef<'c>) {}

    fn run(&mut self, operation: OperationRef<'c, '_>, pass: ExternalPass<'_>) {
        if let Err(err) = self.run(operation) {
            debug!("run wasm conversion pass error: {}", err);
            pass.signal_failure();
        }
    }
}

impl ConversionPass<'_> {
    pub fn into_pass(self) -> Pass {
        create_external(
            self,
            TypeId::create(&CONVERSION_PASS),
            "WASM conversion pass",
            "WASM conversion argument",
            "a WASM conversion pass",
            "",
            &[DialectHandle::func()],
        )
    }
}
