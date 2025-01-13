use crate::conversion::rewriter::{replace_op, Replacer, Rewriter};
use crate::conversion::walker::walk_operation;
use crate::errors::Result;
use crate::value::IntoContextOperation;
use dora_ir;
use melior::ir::ValueLike;
use melior::{
    dialect::DialectHandle,
    ir::{r#type::TypeId, OperationRef},
    pass::{create_external, ExternalPass, Pass, RunExternalPass},
    Context, ContextRef,
};
use tracing::debug;

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
            if name == dora_ir::wasm::AddOperation::name() {
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
            } else if name == dora_ir::wasm::DivSOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::sdiv(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into(),
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
                    dora_ir::dora::shl(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into(),
                );
            } else if name == dora_ir::wasm::ShrSOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::sar(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into(),
                );
            } else if name == dora_ir::wasm::ShrUOperation::name() {
                let lhs = op.operand(0)?;
                let rhs = op.operand(1)?;
                debug_assert!(lhs.r#type() == rhs.r#type());
                replace_op(
                    op,
                    dora_ir::dora::shr(self.ctx, lhs.r#type(), lhs, rhs, op.location()).into(),
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
            } else if name == dora_ir::wasm::I32Extend8SOperation::name()
                || name == dora_ir::wasm::I32Extend16SOperation::name()
                || name == dora_ir::wasm::I64Extend8SOperation::name()
                || name == dora_ir::wasm::I64Extend16SOperation::name()
                || name == dora_ir::wasm::I64Extend32SOperation::name()
            {
                let value = op.operand(0)?;
                let rewriter = Rewriter::new_with_op(self.ctx, op);
                let byte = rewriter.make(rewriter.iconst(
                    value.r#type(),
                    rewriter.int_ty_width(value.r#type())? as i64 - 1,
                ))?;
                rewriter.replace_op(
                    op,
                    dora_ir::dora::signextend(self.ctx, value.r#type(), byte, value, op.location())
                        .into(),
                )?;
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
