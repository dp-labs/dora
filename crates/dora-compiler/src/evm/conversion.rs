use crate::conversion::rewriter::{self, Rewriter};
use crate::conversion::walker::walk_operation;
use crate::errors::Result;
use crate::value::IntoContextOperation;
use dora_ir;
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

/// The `ConversionPass` struct is responsible for transforming specific EVM (Ethereum Virtual Machine)
/// operations into their Dora (eXtensible Virtual Machine) counterparts. It walks through the MLIR operation
/// tree, identifies certain EVM instructions, and replaces them with Dora operations.
///
/// This transformation is essential in a system where WebAssembly or other intermediate representations
/// are being processed with additional virtual machine layers, providing flexibility and extensibility.
///
/// # Struct Details:
/// The `ConversionPass` struct is typically used as part of a larger pass framework where EVM bytecode
/// or MLIR IR is being converted into an intermediate representation supported by Dora. This pass
/// focuses on replacing arithmetic and logical instructions.
///
/// ## EVM to Dora Operation Transformations like:
/// - `evm.add` → `dora.add`
/// - `evm.sub` → `dora.sub`
/// - `evm.mul` → `dora.mul`
/// - `evm.div` → `dora.div`
///
/// These transformations optimize or translate EVM-specific instructions into a format compatible
/// with Dora or other runtime environments.
///
/// # Errors:
/// This conversion pass returns a `Result<()>`, with possible errors occurring during operand retrieval
/// or operation replacement.
///
/// # Methods:
/// - `run(&mut self, operation: OperationRef<'_, '_>) -> Result<()>`
///   - Walks through the operation tree and applies transformations to recognized EVM operations.
#[derive(Clone, Debug)]
pub struct ConversionPass<'c> {
    /// A reference to the MLIR context, which manages global state and resources required for MLIR operations.
    pub ctx: &'c Context,
}

impl ConversionPass<'_> {
    /// Executes the conversion pass on the provided operation.
    ///
    /// This function transforms specific EVM (Ethereum Virtual Machine) operations into equivalent
    /// Dora (eXtensible Virtual Machine) operations. It walks through the operation, checks for
    /// recognized EVM instructions (such as `evm.add`, `evm.sub`, etc.), and replaces them with their
    /// corresponding Dora counterparts (like `dora.add`, `dora.sub`, etc.).
    ///
    /// # Parameters:
    /// - `operation`: The root `OperationRef` representing the operation to be transformed.
    ///
    /// # Errors:
    /// This function returns an error if there are issues in retrieving operands or replacing
    /// the operation with the new Dora operations.
    pub(crate) fn run(&mut self, operation: OperationRef<'_, '_>) -> Result<()> {
        let mut evm_ops = vec![];
        walk_operation(
            operation,
            Box::new(|op| {
                let name = op.name().as_string_ref().as_str().unwrap().to_string();
                if name.starts_with("evm") {
                    evm_ops.push(op.to_ctx_operation_ref());
                }
                Ok(())
            }),
        )?;
        let rewriter = Rewriter::new(self.ctx);
        for op in evm_ops {
            let name = op.name().as_string_ref().as_str().unwrap().to_string();

            if name == "evm.add" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::add(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.sub" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::sub(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.mul" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::mul(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.div" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::div(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.sdiv" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::sdiv(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.mod" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::r#mod(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.smod" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::smod(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.addmod" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::addmod(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.operand(2)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.mulmod" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::mulmod(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.operand(2)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.exp" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::exp(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.signextend" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::signextend(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.lt" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::lt(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.gt" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::gt(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.slt" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::slt(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.sgt" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::sgt(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.eq" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::eq(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.iszero" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::iszero(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.and" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::and(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.or" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::or(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.xor" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::xor(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.not" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::not(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.byte" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::byte(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.shl" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::shl(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.shr" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::shr(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.sar" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::sar(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.keccak256" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::keccak_256(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.address" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::address(self.ctx, rewriter.intrinsics.i256_ty, op.location())
                        .into(),
                );
            } else if name == "evm.balance" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::balance(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.origin" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::origin(self.ctx, rewriter.intrinsics.i256_ty, op.location())
                        .into(),
                );
            } else if name == "evm.caller" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::caller(self.ctx, rewriter.intrinsics.i256_ty, op.location())
                        .into(),
                );
            } else if name == "evm.callvalue" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::callvalue(self.ctx, rewriter.intrinsics.i256_ty, op.location())
                        .into(),
                );
            } else if name == "evm.calldataload" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::calldataload(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.calldatasize" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::calldatasize(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.calldatacopy" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::calldatacopy(
                        self.ctx,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.operand(2)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.codesize" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::codesize(self.ctx, rewriter.intrinsics.i256_ty, op.location())
                        .into(),
                );
            } else if name == "evm.codecopy" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::codecopy(
                        self.ctx,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.operand(2)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.gasprice" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::gasprice(self.ctx, rewriter.intrinsics.i256_ty, op.location())
                        .into(),
                );
            } else if name == "evm.extcodesize" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::extcodesize(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.extcodecopy" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::extcodecopy(
                        self.ctx,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.operand(2)?,
                        op.operand(3)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.returndatasize" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::returndatasize(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.returndatacopy" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::returndatacopy(
                        self.ctx,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.operand(2)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.extcodehash" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::extcodehash(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.blockhash" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::blockhash(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.coinbase" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::coinbase(self.ctx, rewriter.intrinsics.i256_ty, op.location())
                        .into(),
                );
            } else if name == "evm.timestamp" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::timestamp(self.ctx, rewriter.intrinsics.i256_ty, op.location())
                        .into(),
                );
            } else if name == "evm.number" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::number(self.ctx, rewriter.intrinsics.i256_ty, op.location())
                        .into(),
                );
            } else if name == "evm.prevrandao" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::prevrandao(self.ctx, rewriter.intrinsics.i256_ty, op.location())
                        .into(),
                );
            } else if name == "evm.gaslimit" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::gaslimit(self.ctx, rewriter.intrinsics.i256_ty, op.location())
                        .into(),
                );
            } else if name == "evm.chainid" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::chainid(self.ctx, rewriter.intrinsics.i256_ty, op.location())
                        .into(),
                );
            } else if name == "evm.selfbalance" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::selfbalance(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.basefee" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::basefee(self.ctx, rewriter.intrinsics.i256_ty, op.location())
                        .into(),
                );
            } else if name == "evm.blobhash" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::blobhash(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.blobbasefee" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::blobbasefee(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.mload" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::mload(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.mstore" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::mstore(self.ctx, op.operand(0)?, op.operand(1)?, op.location())
                        .into(),
                );
            } else if name == "evm.mstore8" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::mstore_8(
                        self.ctx,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.sload" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::sload(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.sstore" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::sstore(self.ctx, op.operand(0)?, op.operand(1)?, op.location())
                        .into(),
                );
            } else if name == "evm.msize" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::msize(self.ctx, rewriter.intrinsics.i256_ty, op.location())
                        .into(),
                );
            } else if name == "evm.gas" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::gas(self.ctx, rewriter.intrinsics.i256_ty, op.location()).into(),
                );
            } else if name == "evm.tload" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::tload(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.tstore" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::tstore(self.ctx, op.operand(0)?, op.operand(1)?, op.location())
                        .into(),
                );
            } else if name == "evm.mcopy" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::mcopy(
                        self.ctx,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.operand(2)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.log0" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::log_0(self.ctx, op.operand(0)?, op.operand(1)?, op.location())
                        .into(),
                );
            } else if name == "evm.log1" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::log_1(
                        self.ctx,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.operand(2)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.log2" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::log_2(
                        self.ctx,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.operand(2)?,
                        op.operand(3)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.log3" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::log_3(
                        self.ctx,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.operand(2)?,
                        op.operand(3)?,
                        op.operand(4)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.log4" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::log_4(
                        self.ctx,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.operand(2)?,
                        op.operand(3)?,
                        op.operand(4)?,
                        op.operand(5)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.create" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::create(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.operand(2)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.create2" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::create_2(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.operand(2)?,
                        op.operand(3)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.call" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::call(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.operand(2)?,
                        op.operand(3)?,
                        op.operand(4)?,
                        op.operand(5)?,
                        op.operand(6)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.callcode" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::callcode(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.operand(2)?,
                        op.operand(3)?,
                        op.operand(4)?,
                        op.operand(5)?,
                        op.operand(6)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.delegatecall" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::delegatecall(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.operand(2)?,
                        op.operand(3)?,
                        op.operand(4)?,
                        op.operand(5)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.staticcall" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::staticcall(
                        self.ctx,
                        rewriter.intrinsics.i256_ty,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.operand(2)?,
                        op.operand(3)?,
                        op.operand(4)?,
                        op.operand(5)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.return" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::r#return(
                        self.ctx,
                        op.operand(0)?,
                        op.operand(1)?,
                        op.location(),
                    )
                    .into(),
                );
            } else if name == "evm.revert" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::revert(self.ctx, op.operand(0)?, op.operand(1)?, op.location())
                        .into(),
                );
            } else if name == "evm.invalid" {
                rewriter::replace_op(op, dora_ir::dora::invalid(self.ctx, op.location()).into());
            } else if name == "evm.stop" {
                rewriter::replace_op(op, dora_ir::dora::stop(self.ctx, op.location()).into());
            } else if name == "evm.selfdestruct" {
                rewriter::replace_op(
                    op,
                    dora_ir::dora::selfdestruct(self.ctx, op.operand(0)?, op.location()).into(),
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
            debug!("run evm conversion pass error: {}", err);
            pass.signal_failure();
        }
    }
}

impl ConversionPass<'_> {
    pub fn into_pass(self) -> Pass {
        create_external(
            self,
            TypeId::create(&CONVERSION_PASS),
            "EVM conversion pass",
            "EVM conversion argument",
            "a EVM conversion pass",
            "",
            &[DialectHandle::func()],
        )
    }
}
