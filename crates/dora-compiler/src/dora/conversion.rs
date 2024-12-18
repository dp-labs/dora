use crate::{conversion::walker::walk_operation, errors::Result, value::IntoContextOperation};
use dora_primitives::SpecId;
use dora_runtime::constants::CallType;
use melior::{
    dialect::DialectHandle,
    ir::{r#type::TypeId, OperationRef},
    pass::{create_external, ExternalPass, Pass, RunExternalPass},
    Context, ContextRef,
};
use tracing::debug;

#[repr(align(8))]
struct PassId;
static PASS: PassId = PassId;

/// Represents a conversion pass that processes a program's operations, specifically targeting
/// Dora IR (Intermediate Representation) operations and transforming them into equivalent LLVM/MLIR instructions.
///
/// # Fields:
/// - `program_code_size`: The size of the program's bytecode, typically measured in bytes.
///
/// # Example Usage:
/// ```no_check
/// let mut conversion_pass = ConversionPass { program_code_size: 512 };
/// conversion_pass.run(operation_ref).expect("Conversion failed");
/// ```
///
/// # Notes:
/// - The `ConversionPass` struct is primarily used in translating Dora IR instructions such as `dora.add`,
///   `dora.sub`, `dora.mul`, and more into LLVM/MLIR. It processes operations through a walking mechanism,
///   identifies specific Dora IR operations, and performs appropriate transformations.
/// - This pass is essential for optimizing and preparing Dora bytecode for execution in LLVM/MLIR-based environments.
#[derive(Clone, Debug)]
pub struct ConversionPass<'c> {
    /// A reference to the MLIR context, which manages global state and resources required for MLIR operations.
    pub ctx: &'c Context,
    /// The size of the program's bytecode (in bytes).
    pub program_code_size: u32,
    pub spec_id: SpecId,
    pub limit_contract_code_size: Option<usize>,
}

impl ConversionPass<'_> {
    /// Runs the conversion pass on a given operation, walking through the operations in the program
    /// and transforming specific Dora IR operations to their LLVM/MLIR equivalents.
    ///
    /// # Parameters:
    /// - `operation`: A reference to the operation that will be processed.
    ///
    /// # Returns:
    /// - `Result<()>`: Returns an Ok result if the conversion was successful, or an error if it failed.
    ///
    /// # Example:
    /// ```no_check
    /// conversion_pass.run(operation_ref).expect("Conversion failed");
    /// ```
    ///
    /// # Notes:
    /// - This method identifies and converts operations like `dora.add`, `dora.sub`, `dora.mul`, `dora.div`,
    ///   and more. It walks through the operations in the program, collects the applicable ones, and
    ///   transforms them accordingly.
    pub fn run(&mut self, operation: OperationRef<'_, '_>) -> Result<()> {
        let mut dora_ops = vec![];
        let context = self.ctx;
        walk_operation(
            operation,
            Box::new(|op| {
                let name = op.name().as_string_ref().as_str().unwrap().to_string();
                if dora_ir::Operation::try_from(name.as_str()).is_ok() {
                    dora_ops.push(op.to_ctx_operation_ref());
                }
                Ok(())
            }),
        )?;
        for op in &dora_ops {
            let name = op.name().as_string_ref().as_str().unwrap().to_string();

            if name == "dora.add" {
                Self::add(context, op)?
            } else if name == "dora.sub" {
                Self::sub(context, op)?
            } else if name == "dora.mul" {
                Self::mul(context, op)?
            } else if name == "dora.div" {
                Self::udiv(context, op)?
            } else if name == "dora.sdiv" {
                Self::sdiv(context, op)?
            } else if name == "dora.mod" {
                Self::umod(context, op)?
            } else if name == "dora.smod" {
                Self::smod(context, op)?
            } else if name == "dora.addmod" {
                Self::addmod(context, op)?
            } else if name == "dora.mulmod" {
                Self::mulmod(context, op)?
            } else if name == "dora.exp" {
                Self::exp(context, op, &self.spec_id)?
            } else if name == "dora.signextend" {
                Self::signextend(context, op)?
            } else if name == "dora.lt" {
                Self::lt(context, op)?
            } else if name == "dora.gt" {
                Self::gt(context, op)?
            } else if name == "dora.slt" {
                Self::slt(context, op)?
            } else if name == "dora.sgt" {
                Self::sgt(context, op)?
            } else if name == "dora.eq" {
                Self::eq(context, op)?
            } else if name == "dora.iszero" {
                Self::iszero(context, op)?
            } else if name == "dora.and" {
                Self::and(context, op)?
            } else if name == "dora.or" {
                Self::or(context, op)?
            } else if name == "dora.xor" {
                Self::xor(context, op)?
            } else if name == "dora.not" {
                Self::not(context, op)?
            } else if name == "dora.byte" {
                Self::byte(context, op)?
            } else if name == "dora.shl" {
                Self::shl(context, op)?
            } else if name == "dora.shr" {
                Self::shr(context, op)?
            } else if name == "dora.sar" {
                Self::sar(context, op)?
            } else if name == "dora.keccak256" {
                Self::keccak256(context, op)?
            } else if name == "dora.address" {
                Self::address(context, op)?
            } else if name == "dora.balance" {
                Self::balance(context, op)?
            } else if name == "dora.origin" {
                Self::origin(context, op)?
            } else if name == "dora.caller" {
                Self::caller(context, op)?
            } else if name == "dora.callvalue" {
                Self::callvalue(context, op)?
            } else if name == "dora.calldataload" {
                Self::calldataload(context, op)?
            } else if name == "dora.calldatasize" {
                Self::calldatasize(context, op)?
            } else if name == "dora.calldatacopy" {
                Self::calldatacopy(context, op)?
            } else if name == "dora.codesize" {
                Self::codesize(context, op, self.program_code_size)?
            } else if name == "dora.codecopy" {
                Self::codecopy(context, op)?
            } else if name == "dora.gasprice" {
                Self::gasprice(context, op)?
            } else if name == "dora.extcodesize" {
                Self::extcodesize(context, op)?
            } else if name == "dora.extcodecopy" {
                Self::extcodecopy(context, op)?
            } else if name == "dora.returndatasize" {
                Self::returndatasize(context, op)?
            } else if name == "dora.returndatacopy" {
                Self::returndatacopy(context, op)?
            } else if name == "dora.extcodehash" {
                Self::extcodehash(context, op)?
            } else if name == "dora.blockhash" {
                Self::blockhash(context, op)?
            } else if name == "dora.coinbase" {
                Self::coinbase(context, op)?
            } else if name == "dora.timestamp" {
                Self::timestamp(context, op)?
            } else if name == "dora.number" {
                Self::number(context, op)?
            } else if name == "dora.prevrandao" {
                Self::prevrandao(context, op)?
            } else if name == "dora.gaslimit" {
                Self::gaslimit(context, op)?
            } else if name == "dora.chainid" {
                Self::chainid(context, op)?
            } else if name == "dora.selfbalance" {
                Self::selfbalance(context, op)?;
            } else if name == "dora.basefee" {
                Self::basefee(context, op)?;
            } else if name == "dora.blobhash" {
                Self::blobhash(context, op)?;
            } else if name == "dora.blobbasefee" {
                Self::blobbasefee(context, op)?;
            } else if name == "dora.mload" {
                Self::mload(context, op)?;
            } else if name == "dora.mstore" {
                Self::mstore(context, op, 32)?;
            } else if name == "dora.mstore8" {
                Self::mstore(context, op, 1)?;
            } else if name == "dora.sload" {
                Self::sload(context, op)?;
            } else if name == "dora.sstore" {
                Self::sstore(context, op)?;
            } else if name == "dora.msize" {
                Self::msize(context, op)?;
            } else if name == "dora.gas" {
                Self::gas(context, op)?;
            } else if name == "dora.tload" {
                Self::tload(context, op)?;
            } else if name == "dora.tstore" {
                Self::tstore(context, op)?;
            } else if name == "dora.mcopy" {
                Self::mcopy(context, op)?;
            } else if name == "dora.log0" {
                Self::log(context, op, 0)?;
            } else if name == "dora.log1" {
                Self::log(context, op, 1)?;
            } else if name == "dora.log2" {
                Self::log(context, op, 2)?;
            } else if name == "dora.log3" {
                Self::log(context, op, 3)?;
            } else if name == "dora.log4" {
                Self::log(context, op, 4)?;
            } else if name == "dora.dataload" || name == "dora.dataloadn" {
                // Optimize code by merging the same two LLVM/MLIR codes
                Self::dataload(context, op)?;
            } else if name == "dora.datasize" {
                Self::datasize(context, op)?;
            } else if name == "dora.datacopy" {
                Self::datacopy(context, op)?;
            } else if name == "dora.create" {
                Self::create(
                    context,
                    op,
                    false,
                    self.spec_id,
                    self.limit_contract_code_size,
                )?;
            } else if name == "dora.create2" {
                Self::create(
                    context,
                    op,
                    true,
                    self.spec_id,
                    self.limit_contract_code_size,
                )?;
            } else if name == "dora.call" {
                Self::call(context, op, CallType::Call)?;
            } else if name == "dora.callcode" {
                Self::call(context, op, CallType::Callcode)?;
            } else if name == "dora.return" {
                Self::creturn(context, op)?;
            } else if name == "dora.delegatecall" {
                Self::call(context, op, CallType::Delegatecall)?;
            } else if name == "dora.staticcall" {
                Self::call(context, op, CallType::Staticcall)?;
            } else if name == "dora.revert" {
                Self::revert(context, op)?;
            } else if name == "dora.invalid" {
                Self::invalid(context, op)?;
            } else if name == "dora.stop" {
                Self::stop(context, op)?
            } else if name == "dora.selfdestruct" {
                Self::selfdestruct(context, op)?;
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
            debug!("run dora conversion pass error: {}", err);
            pass.signal_failure();
        }
    }
}

impl ConversionPass<'_> {
    pub fn into_pass(self) -> Pass {
        create_external(
            self,
            TypeId::create(&PASS),
            "Dora conversion pass",
            "Dora conversion argument",
            "a Dora conversion pass",
            "",
            &[DialectHandle::func()],
        )
    }
}
