use dora_primitives::SpecId;
use dora_runtime::constants::env::DORA_TRACING;
use dora_runtime::ExitStatusCode;
use dora_runtime::{
    constants::{MAIN_ENTRYPOINT, MAX_STACK_SIZE},
    symbols,
};
use melior::dialect::llvm::AllocaOptions;
use melior::{
    dialect::{
        arith, cf, func,
        llvm::{self, LoadStoreOptions},
    },
    ir::{
        attribute::{FlatSymbolRefAttribute, IntegerAttribute, StringAttribute, TypeAttribute},
        operation::OperationBuilder,
        r#type::{FunctionType, IntegerType},
        Attribute, Block, BlockRef, Identifier, Location, Module, Region, Value,
    },
    Context as MLIRContext,
};
use num_bigint::BigUint;
use program::{stack_io, stack_section_input};
use revmc::{op_info_map, OpcodeInfo};
use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;
use std::collections::BTreeMap;

use crate::backend::IntCC;
use crate::context::Context;
use crate::conversion::builder::OpBuilder;
use crate::errors::{Error as CompileError, Result};
use crate::evm::program::Operation;
use crate::intrinsics::Intrinsics;
use crate::module::Module as MLIRModule;
use crate::symbols as symbols_ctx;
use crate::value::ToContextValue;
use crate::Compiler;
pub mod backend;
pub(crate) mod conversion;
pub(crate) mod instructions;
pub mod pass;
pub mod program;
pub use conversion::ConversionPass;
pub use program::Program;

#[cfg(test)]
mod tests;

/// The `EVMCompiler` struct provides a compiler for translating EVM (Ethereum Virtual Machine) bytecode
/// into MLIR (Multi-Level Intermediate Representation) operations. It encapsulates the MLIR context and
/// EVM-specific intrinsic types required during compilation.
///
/// # Fields:
/// - `ctx`: A reference to the `Context` in which the MLIR operations will be generated. This context manages
///   the state and lifetime of MLIR constructs, including types, operations, and modules. The lifetime `'c`
///   is tied to the compiler's context.
/// - `intrinsics`: A set of EVM intrinsic types (e.g., integer and floating-point types) that are used during
///   the compilation process. These types are necessary to generate correct MLIR operations for EVM instructions.
///
/// # Purpose:
/// The `EVMCompiler` serves as the main entry point for compiling EVM bytecode into MLIR-based representations.
/// It relies on the provided context (`ctx`) to manage the lifetime and validity of MLIR operations, and uses
/// intrinsic types (`intrinsics`) to map EVM constructs into the appropriate MLIR types. This struct simplifies
/// the process of working with MLIR when targeting EVM execution models.
///
/// # Example Usage:
/// ```no_check
/// let evm_compiler = EVMCompiler {
///     ctx: &mlir_context,
///     intrinsics: Intrinsics {
///         i1_ty: mlir_i1_type,
///         i32_ty: mlir_i32_type,
///         // Other intrinsic types...
///         unknown_loc: mlir_unknown_location,
///     },
/// };
/// // Use the `evm_compiler` to translate EVM bytecode into MLIR operations.
/// ```
///
/// # Notes:
/// - The `ctx` field provides access to the MLIR context, which is crucial for managing the state of the
///   intermediate representation during compilation.
/// - The `intrinsics` field provides easy access to the basic types (e.g., integer, float, and index types)
///   necessary for compiling EVM operations.
/// - The `EVMCompiler` simplifies the process of generating MLIR operations from EVM bytecode by abstracting
///   away the details of managing the context and intrinsic types.
pub struct EVMCompiler<'c> {
    /// The MLIR context used for generating operations and managing their lifetime. It encapsulates the state
    /// of the MLIR infrastructure, including types, modules, and operations. This context is tied to the
    /// lifetime `'c` of the EVMCompiler.
    pub ctx: &'c Context,

    /// The intrinsic types specific to the EVM execution model, such as integer and floating-point types.
    /// These are used to translate EVM operations into the corresponding MLIR types during compilation.
    pub intrinsics: Intrinsics<'c>,

    /// Check for stack overflow or underflow errors. Note that there is no need to check for EOF Bytecode,
    /// as stack operations are statically determined at compile time.
    pub stack_bound_checks: bool,
}

impl<'c> Compiler for EVMCompiler<'c> {
    type Module = Program;
    type Target = ();
    type Options = CompileOptions;
    type Compilation = MLIRModule<'c>;
    type CompileError = CompileError;

    fn name(&self) -> &str {
        "evm-bytecode-to-mlir-evm-dialect-compiler"
    }

    fn compile(
        &self,
        module: &Self::Module,
        _target: &Self::Target,
        _options: &Self::Options,
    ) -> std::result::Result<Self::Compilation, Self::CompileError> {
        let context = &self.ctx.mlir_context;

        // Build a module with a single function
        let module_region = Region::new();
        let module_block = Block::new(&[]);

        module_region.append_block(module_block);
        // build main module
        let op = OperationBuilder::new("builtin.module", Location::unknown(context))
            .add_regions([module_region])
            .build()?;

        let mlir_module = Module::from_operation(op).expect("module failed to create");

        self.compile_module(&mlir_module, module, _options)?;

        Ok(MLIRModule::new(mlir_module))
    }
}

impl<'c> EVMCompiler<'c> {
    /// Creates a new instance of `EVMCompiler`.
    ///
    /// # Parameters
    /// * `ctx` - A reference to the context in which the compiler operates.
    ///
    /// # Returns
    /// A new instance of `EVMCompiler`.
    pub fn new(ctx: &'c Context) -> Self {
        Self {
            intrinsics: Intrinsics::declare(ctx),
            ctx,
            stack_bound_checks: true,
        }
    }

    /// Generates operation functions for the EVM compiler.
    ///
    /// This function iterates over the operations in the given program and creates a new MLIR function
    /// for each unique opcode that is not a jump instruction (Jump, JumpI, JumpF). The generated functions
    /// are stored in a hash map with the opcode as the key.
    ///
    /// The generated function has the following signature:
    /// - Parameters: RuntimeContext*, Gas counter ptr, Stack pointer, Stack size pointer, Stack top pointer
    /// - Return type: i8
    ///
    /// The function body is generated by the `EVMCompiler::generate_code_for_op` method, which translates
    /// the EVM operation into MLIR operations.
    ///
    /// # Arguments
    ///
    /// * `context` - A reference to the MLIR context.
    /// * `intrinsics` - A reference to the intrinsics used by the compiler.
    /// * `program` - A reference to the EVM program being compiled.
    /// * `options` - A reference to the compiler options.
    ///
    /// # Returns
    ///
    /// A `Result` containing a hash map of operation functions, where each key is an opcode and the value
    /// is the corresponding MLIR function. If an error occurs during function generation, the `Err` variant
    /// is returned.
    pub fn generate_op_functions(
        context: &'c Context,
        intrinsics: &'c Intrinsics,
        program: &Program,
        options: &<EVMCompiler<'c> as Compiler>::Options,
    ) -> Result<FxHashMap<usize, melior::ir::Operation<'c>>> {
        let location = intrinsics.unknown_loc;
        let mut op_funcs: FxHashMap<usize, melior::ir::Operation> = FxHashMap::default();
        for op in &program.operations {
            if matches!(
                op,
                Operation::Jump
                    | Operation::JumpI
                    | Operation::JumpF(_)
                    | Operation::Push(_)
                    | Operation::PC { .. }
            ) {
                continue;
            }
            let opcode = op.opcode();
            if let Entry::Vacant(e) = op_funcs.entry(opcode) {
                let op_symbol = format!("op{}", opcode);
                let op_func = func::func(
                    &context.mlir_context,
                    StringAttribute::new(&context.mlir_context, &op_symbol),
                    TypeAttribute::new(
                        FunctionType::new(
                            &context.mlir_context,
                            &[
                                // RuntimeContext
                                intrinsics.ptr_ty,
                                // Gas counter ptr
                                intrinsics.ptr_ty,
                                // Stack pointer
                                intrinsics.ptr_ty,
                                // Stack size pointer
                                intrinsics.ptr_ty,
                                // Stack top pointer
                                intrinsics.ptr_ty,
                            ],
                            &[intrinsics.i8_ty],
                        )
                        .into(),
                    ),
                    Region::new(),
                    &[(
                        Identifier::new(&context.mlir_context, "sym_visibility"),
                        StringAttribute::new(&context.mlir_context, "private").into(),
                    )],
                    location,
                );
                let op_func_region = op_func.region(0)?;
                let setup_block = op_func_region.append_block(Block::new(&[]));
                let mut ctx = LegacyCtxType::new_op_func_ctx(
                    context,
                    &op_func_region,
                    &setup_block,
                    program,
                )?;
                let (block_start, block_end) =
                    EVMCompiler::generate_code_for_op(&mut ctx, &op_func_region, op, options)?;
                setup_block.append_operation(cf::br(&block_start, &[], location));
                let return_block = op_func_region.append_block(Block::new(&[]));
                block_end.append_operation(cf::br(&return_block, &[], location));
                let reason = return_block
                    .append_operation(arith::constant(
                        &context.mlir_context,
                        IntegerAttribute::new(
                            intrinsics.i8_ty,
                            ExitStatusCode::Continue.to_u8() as i64,
                        )
                        .into(),
                        location,
                    ))
                    .result(0)?
                    .into();
                return_block.append_operation(func::r#return(&[reason], location));
                e.insert(op_func);
            }
        }
        Ok(op_funcs)
    }

    /// Generates blocks for the specified target [`Operation`].
    ///
    /// This method translates an operation into a sequence of blocks, returning
    /// both the starting block and the unterminated last block of the generated code.
    ///
    /// # Parameters
    /// * `ctx` - A mutable reference to the context type used for code generation.
    /// * `region` - A reference to the region where the code will be generated.
    /// * `op` - The operation to generate code for.
    ///
    /// # Returns
    /// A `Result` containing a tuple of `BlockRef` representing the starting and last blocks.
    pub fn generate_code_for_op(
        ctx: &mut dyn CtxType<'c>,
        region: &'c Region<'c>,
        op: &Operation,
        options: &<EVMCompiler<'c> as Compiler>::Options,
    ) -> Result<(BlockRef<'c, 'c>, BlockRef<'c, 'c>)> {
        let op_infos = op_info_map(unsafe {
            std::mem::transmute::<dora_primitives::SpecId, revmc::primitives::SpecId>(
                options.spec_id,
            )
        });
        let op_info = op_infos[op.opcode()];

        // Note: make opcode not found as the runtime halt error,
        // because normal opcodes still consumes GAS during runtime.
        if op_info.is_unknown() || op_info.is_disabled() {
            return Self::invalid_with_error_code(ctx, region, ExitStatusCode::OpcodeNotFound);
        }

        let (mut block_start, block_end) = match &op {
            // Arithmetic instructions
            Operation::Add => EVMCompiler::add(ctx, region),
            Operation::Mul => EVMCompiler::mul(ctx, region),
            Operation::Sub => EVMCompiler::sub(ctx, region),
            Operation::Div => EVMCompiler::udiv(ctx, region),
            Operation::SDiv => EVMCompiler::sdiv(ctx, region),
            Operation::Mod => EVMCompiler::umod(ctx, region),
            Operation::SMod => EVMCompiler::smod(ctx, region),
            Operation::AddMod => EVMCompiler::addmod(ctx, region),
            Operation::MulMod => EVMCompiler::mulmod(ctx, region),
            Operation::Exp => EVMCompiler::exp(ctx, region),
            Operation::SignExtend => EVMCompiler::signextend(ctx, region),
            // Bitwise instructions
            Operation::Lt => EVMCompiler::lt(ctx, region),
            Operation::Gt => EVMCompiler::gt(ctx, region),
            Operation::Slt => EVMCompiler::slt(ctx, region),
            Operation::Sgt => EVMCompiler::sgt(ctx, region),
            Operation::Eq => EVMCompiler::eq(ctx, region),
            Operation::IsZero => EVMCompiler::iszero(ctx, region),
            Operation::And => EVMCompiler::and(ctx, region),
            Operation::Or => EVMCompiler::or(ctx, region),
            Operation::Xor => EVMCompiler::xor(ctx, region),
            Operation::Not => EVMCompiler::not(ctx, region),
            Operation::Byte => EVMCompiler::byte(ctx, region),
            Operation::Shl => EVMCompiler::shl(ctx, region),
            Operation::Shr => EVMCompiler::shr(ctx, region),
            Operation::Sar => EVMCompiler::sar(ctx, region),
            // System instructions
            Operation::Keccak256 => EVMCompiler::keccak256(ctx, region),
            Operation::Address => EVMCompiler::address(ctx, region),
            Operation::Caller => EVMCompiler::caller(ctx, region),
            Operation::CallValue => EVMCompiler::callvalue(ctx, region),
            Operation::CalldataLoad => EVMCompiler::calldataload(ctx, region),
            Operation::CalldataSize => EVMCompiler::calldatasize(ctx, region),
            Operation::CalldataCopy => EVMCompiler::calldatacopy(ctx, region),
            Operation::DataLoad => EVMCompiler::dataload(ctx, region),
            Operation::DataLoadN(x) => EVMCompiler::dataloadn(ctx, region, *x),
            Operation::DataSize => EVMCompiler::datasize(ctx, region),
            Operation::DataCopy => EVMCompiler::datacopy(ctx, region),
            Operation::CodeSize => EVMCompiler::codesize(ctx, region),
            Operation::CodeCopy => EVMCompiler::codecopy(ctx, region),
            Operation::ExtCodeCopy => EVMCompiler::extcodecopy(ctx, region),
            Operation::ReturndataLoad => EVMCompiler::returndataload(ctx, region),
            Operation::ReturndataSize => EVMCompiler::returndatasize(ctx, region),
            Operation::ReturndataCopy => EVMCompiler::returndatacopy(ctx, region),
            Operation::Gas => EVMCompiler::gas(ctx, region),
            // Host env instructions
            Operation::GasPrice => EVMCompiler::gasprice(ctx, region),
            Operation::Coinbase => EVMCompiler::coinbase(ctx, region),
            Operation::Origin => EVMCompiler::origin(ctx, region),
            Operation::Timestamp => EVMCompiler::timestamp(ctx, region),
            Operation::Number => EVMCompiler::number(ctx, region),
            Operation::Prevrandao => EVMCompiler::prevrandao(ctx, region),
            Operation::GasLimit => EVMCompiler::gaslimit(ctx, region),
            Operation::Chainid => EVMCompiler::chainid(ctx, region),
            Operation::BaseFee => EVMCompiler::basefee(ctx, region),
            Operation::BlobBaseFee => EVMCompiler::blobbasefee(ctx, region),
            Operation::BlobHash => EVMCompiler::blobhash(ctx, region),
            // Host instructions
            Operation::Balance => EVMCompiler::balance(ctx, region),
            Operation::SelfBalance => EVMCompiler::selfbalance(ctx, region),
            Operation::ExtCodeSize => EVMCompiler::extcodesize(ctx, region),
            Operation::ExtCodeHash => EVMCompiler::extcodehash(ctx, region),
            Operation::BlockHash => EVMCompiler::blockhash(ctx, region),
            Operation::SLoad => EVMCompiler::sload(ctx, region),
            Operation::SStore => EVMCompiler::sstore(ctx, region),
            Operation::TLoad => EVMCompiler::tload(ctx, region),
            Operation::TStore => EVMCompiler::tstore(ctx, region),
            Operation::Log(x) => EVMCompiler::log(ctx, region, *x),
            Operation::Selfdestruct => EVMCompiler::selfdestruct(ctx, region),
            // Stack instructions
            Operation::Push0 => EVMCompiler::push(ctx, region, BigUint::ZERO),
            Operation::Push((_, x)) => EVMCompiler::push(ctx, region, (*x).clone()),
            Operation::Pop => EVMCompiler::pop(ctx, region),
            Operation::Dup(n) => EVMCompiler::dup(ctx, region, (*n).into()),
            Operation::DupN(x) => EVMCompiler::dupn(ctx, region, *x),
            Operation::Swap(n) => EVMCompiler::swap(ctx, region, (*n).into()),
            Operation::SwapN(x) => EVMCompiler::swapn(ctx, region, *x),
            Operation::Exchange(x) => EVMCompiler::exchange(ctx, region, *x),
            // Control instructions
            Operation::Jump => EVMCompiler::jump(ctx, region),
            Operation::JumpI => EVMCompiler::jumpi(ctx, region),
            Operation::RJump(x) => EVMCompiler::rjump(ctx, region, *x),
            Operation::RJumpI(x) => EVMCompiler::rjumpi(ctx, region, *x),
            Operation::RJumpV((x1, x2)) => EVMCompiler::rjumpv(ctx, region, *x1, (*x2).clone()),
            Operation::CallF(x) => EVMCompiler::callf(ctx, region, *x),
            Operation::RetF => EVMCompiler::retf(ctx, region),
            Operation::JumpF(x) => EVMCompiler::jumpf(ctx, region, *x),
            Operation::PC { pc } => EVMCompiler::pc(ctx, region, *pc),
            Operation::Jumpdest { pc } => EVMCompiler::jumpdest(ctx, region, *pc),
            Operation::Revert => EVMCompiler::revert(ctx, region),
            Operation::Stop => EVMCompiler::stop(ctx, region),
            Operation::Invalid => EVMCompiler::invalid(ctx, region),
            // Memory instructions
            Operation::MLoad => EVMCompiler::mload(ctx, region),
            Operation::MStore => EVMCompiler::mstore(ctx, region),
            Operation::MStore8 => EVMCompiler::mstore8(ctx, region),
            Operation::MSize => EVMCompiler::msize(ctx, region),
            Operation::MCopy => EVMCompiler::mcopy(ctx, region),
            // Contract instructions
            Operation::Create => EVMCompiler::create(ctx, region),
            Operation::Create2 => EVMCompiler::create2(ctx, region),
            Operation::EofCreate(x) => EVMCompiler::eofcreate(ctx, region, *x),
            Operation::ReturnContract(x) => EVMCompiler::returncontract(ctx, region, *x),
            Operation::Call => EVMCompiler::call(ctx, region),
            Operation::Callcode => EVMCompiler::callcode(ctx, region),
            Operation::Delegatecall => EVMCompiler::delegatecall(ctx, region),
            Operation::Staticcall => EVMCompiler::staticcall(ctx, region),
            Operation::Return => EVMCompiler::creturn(ctx, region),
            Operation::ExtCall => EVMCompiler::extcall(ctx, region),
            Operation::ExtDelegatecall => EVMCompiler::extdelegatecall(ctx, region),
            Operation::ExtStaticcall => EVMCompiler::extstaticcall(ctx, region),
        }?;

        // FIXME : alter below hardcoded line with eof checks in Program in the future
        let is_eof = false;
        // Stack overflow/underflow check.
        if !is_eof && options.stack_bound_checks {
            block_start = Self::stack_bound_checks_block(ctx, region, block_start, op)?;
        }
        // Static gas metering needs to be done before stack checking.
        if options.gas_metering {
            block_start = Self::gas_metering_block(ctx, region, block_start, op, &op_info)?;
        }
        Ok((block_start, block_end))
    }

    fn stack_bound_checks_block<'r>(
        ctx: &mut dyn CtxType<'c>,
        region: &'r Region<'c>,
        block_start: BlockRef<'r, 'c>,
        op: &Operation,
    ) -> Result<BlockRef<'r, 'c>> {
        let (i, o) = stack_io(op);
        let section_input = stack_section_input(op);
        let diff = o as i64 - i as i64;
        let may_underflow = section_input > 0;
        let may_overflow = diff > 0;
        let stack_check_block = region.append_block(Block::new(&[]));
        let builder = OpBuilder::new_with_block(ctx.context(), stack_check_block);
        let uint64 = builder.uint64_ty();
        let location = builder.get_insert_location();
        let stack_max_size = builder.make(builder.iconst_64(MAX_STACK_SIZE as i64))?;
        let size_before = builder.make(builder.load(ctx.values().stack_size_ptr, uint64))?;
        let size_after = builder.make(arith::addi(
            size_before,
            builder.make(builder.iconst_64(diff))?,
            location,
        ))?;
        // Check potential underflow/overflow
        if may_underflow && may_overflow {
            // Update the stack length for this operation.
            builder.create(builder.store(size_after, ctx.values().stack_size_ptr));
            // Check underflow
            let i = builder.make(builder.iconst_64(section_input as i64))?;
            let underflow = builder.make(builder.icmp(IntCC::UnsignedLessThan, size_before, i))?;
            // Check overflow
            let overflow =
                builder.make(builder.icmp(IntCC::UnsignedLessThan, stack_max_size, size_after))?;
            // Whether revert
            let revert = builder.make(arith::xori(underflow, overflow, location))?;
            let code =
                builder.make(builder.iconst_8(ExitStatusCode::StackOverflow.to_u8() as i8))?;
            builder.create(cf::cond_br(
                ctx.context(),
                revert,
                &ctx.revert_block(),
                &block_start,
                &[],
                &[code],
                location,
            ));
        } else if may_underflow {
            // Update the stack length for this operation.
            builder.create(builder.store(size_after, ctx.values().stack_size_ptr));
            // Whether revert (or underflow)
            let i = builder.make(builder.iconst_64(section_input as i64))?;

            let revert = builder.make(builder.icmp(IntCC::UnsignedLessThan, size_before, i))?;

            let code =
                builder.make(builder.iconst_8(ExitStatusCode::StackUnderflow.to_u8() as i8))?;
            builder.create(cf::cond_br(
                ctx.context(),
                revert,
                &ctx.revert_block(),
                &block_start,
                &[],
                &[code],
                location,
            ));
        } else if may_overflow {
            // Update the stack length for this operation.
            builder.create(builder.store(size_after, ctx.values().stack_size_ptr));
            // Whether revert (or overflow)
            let revert =
                builder.make(builder.icmp(IntCC::UnsignedLessThan, stack_max_size, size_after))?;
            let code =
                builder.make(builder.iconst_8(ExitStatusCode::StackOverflow.to_u8() as i8))?;
            builder.create(cf::cond_br(
                ctx.context(),
                revert,
                &ctx.revert_block(),
                &block_start,
                &[],
                &[code],
                location,
            ));
        } else {
            // Update the stack length for this operation.
            builder.create(builder.store(size_after, ctx.values().stack_size_ptr));
            builder.create(cf::br(&block_start, &[], location));
        }
        Ok(stack_check_block)
    }

    fn gas_metering_block<'r>(
        ctx: &mut dyn CtxType,
        region: &'r Region<'c>,
        block_start: BlockRef<'r, 'c>,
        op: &Operation,
        op_info: &OpcodeInfo,
    ) -> Result<BlockRef<'r, 'c>> {
        let base_gas = op_info.base_gas();
        let gas_check_block = region.append_block(Block::new(&[]));
        let update_gas_remaining_block = region.append_block(Block::new(&[]));
        let builder = OpBuilder::new_with_block(ctx.context(), gas_check_block);
        let uint8 = builder.uint8_ty();
        let uint64 = builder.uint64_ty();
        let location = builder.get_insert_location();
        // Get address of gas counter global
        let gas_counter = builder.make(builder.load(ctx.values().gas_counter_ptr, uint64))?;
        let gas_value = builder.make(builder.iconst_64(base_gas as i64))?;
        if std::env::var(DORA_TRACING).is_ok() {
            let opcode = builder
                .create(builder.iconst(uint8, op.opcode() as i64))
                .result(0)?
                .into();
            builder.create(func::call(
                builder.context(),
                FlatSymbolRefAttribute::new(builder.context(), symbols::TRACING),
                &[
                    ctx.values().syscall_ctx,
                    opcode,
                    gas_counter,
                    ctx.values().stack_ptr,
                    ctx.values().stack_size_ptr,
                ],
                &[],
                builder.get_insert_location(),
            ));
        } else {
            // FIXME: insert an empty FFI interface to prevent inline optimization of gas registers.
            builder.create(func::call(
                builder.context(),
                FlatSymbolRefAttribute::new(builder.context(), symbols::NOP),
                &[],
                &[],
                builder.get_insert_location(),
            ));
        }
        let flag = builder.make(arith::cmpi(
            builder.context(),
            arith::CmpiPredicate::Uge,
            gas_counter,
            gas_value,
            location,
        ))?;
        builder.create(cf::cond_br(
            builder.context(),
            flag,
            &update_gas_remaining_block,
            &ctx.revert_block(),
            &[],
            &[builder.make(builder.iconst_8(ExitStatusCode::OutOfGas.to_u8() as i8))?],
            location,
        ));
        let builder = OpBuilder::new_with_block(ctx.context(), update_gas_remaining_block);
        let new_gas_counter = builder.make(arith::subi(gas_counter, gas_value, location))?;
        builder.create(llvm::store(
            builder.context(),
            new_gas_counter,
            ctx.values().gas_counter_ptr,
            location,
            LoadStoreOptions::default(),
        ));
        builder.create(cf::br(&block_start, &[], location));
        Ok(gas_check_block)
    }

    fn compile_module(
        &self,
        module: &Module,
        program: &Program,
        options: &<EVMCompiler<'c> as Compiler>::Options,
    ) -> Result<()> {
        let context = &self.ctx.mlir_context;
        let uint8 = self.intrinsics.i8_ty;
        let ptr_type = self.intrinsics.ptr_ty;
        let location = self.intrinsics.unknown_loc;
        // Build the main function
        let main_func = func::func(
            context,
            StringAttribute::new(context, MAIN_ENTRYPOINT),
            TypeAttribute::new(
                FunctionType::new(
                    context,
                    &[
                        // RuntimeContext
                        ptr_type, // Gas counter ptr
                        ptr_type, // Stack pointer
                        ptr_type, // Stack size pointer
                        ptr_type,
                    ],
                    &[uint8],
                )
                .into(),
            ),
            Region::new(),
            &[
                (
                    Identifier::new(context, "sym_visibility"),
                    StringAttribute::new(context, "public").into(),
                ),
                (
                    Identifier::new(context, "llvm.emit_c_interface"),
                    Attribute::unit(context),
                ),
            ],
            location,
        );

        let main_region = main_func.region(0)?;
        let setup_block = main_region.append_block(Block::new(&[]));

        let mut ctx = LegacyCtxType::new_main_func_ctx(
            self.ctx,
            module,
            &main_region,
            &setup_block,
            program,
        )?;
        let mut last_block = setup_block;
        // Generate all opcode with the inline mode.
        if options.inline {
            // Generate code for the program
            for op in &ctx.program.operations {
                let (block_start, block_end) =
                    EVMCompiler::generate_code_for_op(&mut ctx, &main_region, op, options)?;
                // Register the jump dest block.
                if let Operation::Jumpdest { pc } = op {
                    ctx.register_jump_destination(*pc, block_start);
                }
                last_block.append_operation(cf::br(&block_start, &[], location));
                last_block = block_end;
            }
            let return_block = main_region.append_block(Block::new(&[]));
            EVMCompiler::return_empty_result(&ctx, return_block, ExitStatusCode::Stop)?;
            last_block.append_operation(cf::br(&return_block, &[], location));
        } else {
            // Generate opcode functions for the program
            let op_funcs =
                EVMCompiler::generate_op_functions(self.ctx, &self.intrinsics, program, options)?;
            let mut result = last_block
                .append_operation(arith::constant(
                    context,
                    IntegerAttribute::new(uint8, ExitStatusCode::Continue.to_u8() as i64).into(),
                    location,
                ))
                .result(0)?
                .into();
            let continue_code: Value<'_, '_> = last_block
                .append_operation(arith::constant(
                    context,
                    IntegerAttribute::new(uint8, ExitStatusCode::Continue.to_u8() as i64).into(),
                    location,
                ))
                .result(0)?
                .into();
            let continue_code = continue_code.to_ctx_value();
            // Generate code for the program
            for op in &ctx.program.operations {
                let op_symbol = format!("op{}", op.opcode());
                if matches!(
                    op,
                    Operation::Jump
                        | Operation::JumpI
                        | Operation::JumpF(_)
                        | Operation::Push(_)
                        | Operation::PC { .. }
                ) {
                    let (block_start, block_end) =
                        EVMCompiler::generate_code_for_op(&mut ctx, &main_region, op, options)?;
                    let is_stop = last_block
                        .append_operation(arith::cmpi(
                            context,
                            arith::CmpiPredicate::Ne,
                            result,
                            continue_code,
                            location,
                        ))
                        .result(0)?
                        .into();
                    last_block.append_operation(cf::cond_br(
                        context,
                        is_stop,
                        &ctx.revert_block,
                        &block_start,
                        &[result],
                        &[],
                        location,
                    ));
                    let builder = OpBuilder::new_with_block(context, block_end);
                    result = builder
                        .make(builder.iconst_8(ExitStatusCode::Continue.to_u8() as i8))?
                        .to_ctx_value();
                    last_block = block_end;
                } else {
                    let block_start = main_region.append_block(Block::new(&[]));
                    let builder = OpBuilder::new_with_block(context, block_start);
                    let is_stop = last_block
                        .append_operation(arith::cmpi(
                            context,
                            arith::CmpiPredicate::Ne,
                            result,
                            continue_code,
                            location,
                        ))
                        .result(0)?
                        .into();
                    last_block.append_operation(cf::cond_br(
                        context,
                        is_stop,
                        &ctx.stop_block,
                        &block_start,
                        &[result],
                        &[],
                        location,
                    ));
                    result = builder
                        .make(func::call(
                            context,
                            FlatSymbolRefAttribute::new(context, &op_symbol),
                            &[
                                ctx.values.syscall_ctx,
                                ctx.values.gas_counter_ptr,
                                ctx.values.stack_ptr,
                                ctx.values.stack_size_ptr,
                                ctx.values.stack_top_ptr,
                            ],
                            &[uint8],
                            location,
                        ))?
                        .to_ctx_value();
                    last_block = block_start;
                    // Register the jump dest block.
                    if let Operation::Jumpdest { pc } = op {
                        ctx.register_jump_destination(*pc, block_start);
                    }
                }
            }
            let return_block = main_region.append_block(Block::new(&[]));
            EVMCompiler::return_empty_result(&ctx, return_block, ExitStatusCode::Stop)?;
            let is_stop = last_block
                .append_operation(arith::cmpi(
                    context,
                    arith::CmpiPredicate::Ne,
                    result,
                    continue_code,
                    location,
                ))
                .result(0)?
                .into();
            last_block.append_operation(cf::cond_br(
                context,
                is_stop,
                &ctx.stop_block,
                &return_block,
                &[result],
                &[],
                location,
            ));
            for (_, op_func) in op_funcs {
                module.body().append_operation(op_func);
            }
        }
        // Deal jump operations
        ctx.populate_jumptable()?;
        module.body().append_operation(main_func);
        Ok(())
    }

    fn return_empty_result(
        ctx: &dyn CtxType,
        block: BlockRef<'_, '_>,
        code: ExitStatusCode,
    ) -> Result<()> {
        let builder = OpBuilder::new_with_block(ctx.context(), block);
        let uint8 = builder.uint8_ty();
        let uint64 = builder.uint64_ty();
        let zero = builder.create(builder.iconst_64(0)).result(0)?.into();
        let reason = builder
            .create(builder.iconst(uint8, code.to_u8() as i64))
            .result(0)?
            .into();

        let gas_counter = builder.make(builder.load(ctx.values().gas_counter_ptr, uint64))?;

        builder.create(func::call(
            builder.context(),
            FlatSymbolRefAttribute::new(builder.context(), symbols::WRITE_RESULT),
            &[ctx.values().syscall_ctx, zero, zero, gas_counter, reason],
            &[],
            builder.get_insert_location(),
        ));
        builder.create(func::r#return(&[reason], builder.get_insert_location()));
        Ok(())
    }
}

/// Represents the options used during the compilation process.
/// This struct encapsulates various settings that can be adjusted to customize the compilation behavior.
#[derive(Debug)]
pub struct CompileOptions {
    /// Specification IDs and their activation block.
    ///
    /// Information was obtained from the [Ethereum Execution Specifications](https://github.com/ethereum/execution-specs)
    pub spec_id: SpecId,
    /// A flag indicating whether to do validation for possible eof bytecode before compilation.
    pub validate_eof: bool,
    /// A flag indicating whether to perform gas metering during compilation.
    pub gas_metering: bool,
    /// Check for stack overflow or underflow errors. Note that there is no need to check for EOF Bytecode,
    /// as stack operations are statically determined at compile time.
    pub stack_bound_checks: bool,
    /// Use common op functions instead of inlining everything
    pub inline: bool,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            spec_id: SpecId::CANCUN,
            validate_eof: true,
            gas_metering: true,
            stack_bound_checks: true,
            inline: false,
        }
    }
}

/// The `CtxValues` struct encapsulates values specific to the EVM context, such as those used for
/// system calls and gas management during execution.
///
/// # Fields:
/// - `syscall_ctx`: A value representing the context for system calls, used to interact with
///   external environments during EVM execution.
/// - `remaining_gas`: A value representing the remaining gas for the current execution, which
///   controls the computational cost within the EVM execution model.
///
/// # Purpose:
/// `CtxValues` simplifies the management of context-specific values during the execution of EVM bytecode.
/// It abstracts the remaining gas and system call context, making it easier to access and manage these
/// values within the MLIR execution framework.
///
/// # Example Usage:
/// ```no_check
/// let ctx_values = CtxValues {
///     syscall_ctx: syscall_value,
///     remaining_gas: gas_value,
/// };
/// ```
///
/// # Notes:
/// - These values are integral to tracking the state of system calls and remaining gas during execution.
#[derive(Debug)]
pub struct CtxValues<'c> {
    /// The system call context value used during EVM execution.
    pub syscall_ctx: Value<'c, 'c>,
    /// The gas counter pointer used during EVM execution and it's type is a `*mut u64`.
    pub gas_counter_ptr: Value<'c, 'c>,
    /// The address of the global stack pointer and it's type is a `*mut Stack`
    pub stack_ptr: Value<'c, 'c>,
    /// The address of the global stack top pointer and it's type is a `*mut *mut Stack`
    pub stack_top_ptr: Value<'c, 'c>,
    /// The address of the global stack size and it's type is a `*mut u64`.
    pub stack_size_ptr: Value<'c, 'c>,
}

pub trait CtxType<'c> {
    /// The MLIR context used for managing types, operations, and modules.
    fn context(&self) -> &MLIRContext;
    /// The program being executed or compiled in this context.
    fn program(&self) -> &Program;
    /// A set of values relevant to the context of EVM execution, such as the remaining gas and syscall context.
    fn values(&self) -> &CtxValues;
    /// The block used to handle reverts (errors or exceptions) in the EVM execution model.
    fn revert_block(&self) -> BlockRef;
    /// The block used to handle stop logic with a return code in the EVM execution model.
    fn stop_block(&self) -> BlockRef;
    /// The block used to handle jump table operations in the EVM bytecode.
    fn jumptable_block(&self) -> BlockRef;
    /// A map from jump destination indices in EVM to their corresponding blocks in MLIR.
    fn jumpdest_blocks(&self) -> &BTreeMap<usize, BlockRef>;

    fn new_main_func_ctx(
        context: &'c Context,
        module: &'c Module,
        region: &'c Region,
        block: &'c Block<'c>,
        program: &'c Program,
    ) -> Result<Self>
    where
        Self: Sized;

    fn new_op_func_ctx(
        context: &'c Context,
        region: &'c Region,
        block: &'c Block<'c>,
        program: &'c Program,
    ) -> Result<Self>
    where
        Self: Sized;

    fn populate_jumptable(&self) -> Result<()>;

    fn register_jump_destination(&mut self, pc: usize, block: BlockRef<'c, 'c>);

    fn add_jump_op(&mut self, block: BlockRef<'c, 'c>, pc_to_jump_to: Value, location: Location);
}

/// The [`LegacyCtxType`] struct holds the necessary context and data structures for managing
/// the execution environment when compiling or interpreting EVM (Ethereum Virtual Machine)
/// bytecode using MLIR. It encapsulates references to essential components like the MLIR context,
/// the program being executed, values, and blocks for control flow management.
///
/// # Fields:
/// - `context`: A reference to the `MLIRContext` that manages the lifetime and state of the
///   intermediate representation.
/// - `program`: A reference to the current `Program` being executed or compiled.
/// - `values`: A set of context-specific values, such as the remaining gas and syscall context,
///   which are crucial during the execution of EVM bytecode.
/// - `revert_block`: A reference to a block used for handling reverts (exceptions or errors) in
///   the EVM execution model.
/// - `jumptable_block`: A reference to a block that handles jump table operations for dynamic
///   control flow in EVM bytecode.
/// - `jumpdest_blocks`: A map that associates EVM jump destination indices with their corresponding
///   blocks in MLIR.
///
/// # Purpose:
/// The [`LegacyCtxType`] struct acts as the primary context for managing the execution of EVM bytecode.
/// It provides a unified structure that encapsulates control flow blocks and essential values,
/// facilitating smooth execution and handling of jumps, exceptions, and system calls.
///
/// # Example Usage:
/// ```no_check
/// let ctx_type = LegacyCtxType {
///     context: &mlir_context,
///     program: &evm_program,
///     values: CtxValues {
///         syscall_ctx: syscall_value,
///         remaining_gas: gas_value,
///     },
///     revert_block: revert_block_ref,
///     jumptable_block: jumptable_block_ref,
///     jumpdest_blocks: jumpdest_map,
/// };
/// ```
///
/// # Notes:
/// - `LegacyCtxType` is integral for managing the flow of execution in the context of EVM bytecode,
///   particularly handling jumps, system calls, and reverts efficiently within the MLIR infrastructure.
#[derive(Debug)]
pub struct LegacyCtxType<'c> {
    context: &'c MLIRContext,

    program: &'c Program,

    values: CtxValues<'c>,

    revert_block: BlockRef<'c, 'c>,

    stop_block: BlockRef<'c, 'c>,

    jumptable_block: BlockRef<'c, 'c>,

    jumpdest_blocks: BTreeMap<usize, BlockRef<'c, 'c>>,
}

// impl<'c> LegacyCtxType<'c> {
impl<'c> CtxType<'c> for LegacyCtxType<'c> {
    fn context(&self) -> &MLIRContext {
        &self.context
    }

    fn program(&self) -> &Program {
        &self.program
    }

    fn values(&self) -> &CtxValues {
        &self.values
    }

    fn revert_block(&self) -> BlockRef {
        self.revert_block
    }

    fn stop_block(&self) -> BlockRef {
        self.stop_block
    }

    fn jumptable_block(&self) -> BlockRef {
        self.jumptable_block
    }

    fn jumpdest_blocks(&self) -> &BTreeMap<usize, BlockRef> {
        &self.jumpdest_blocks
    }

    /// Creates a new instance of `CtxType` with the specified parameters.
    ///
    /// This constructor initializes the context for code generation, setting up
    /// the necessary components such as syscall context, initial gas, and symbol
    /// declarations. It also prepares the revert and jump table blocks.
    ///
    /// # Parameters
    /// * `context` - A reference to the MLIR context.
    /// * `module` - A reference to the MLIR module.
    /// * `region` - A reference to the region in which to create blocks.
    /// * `block` - A reference to the block in which operations will be generated.
    /// * `program` - A reference to the program being compiled.
    ///
    /// # Returns
    /// A `Result<Self>` containing the new `CtxType` instance on success, or an
    /// error if the initialization fails.
    fn new_main_func_ctx(
        context: &'c Context,
        module: &'c Module,
        region: &'c Region,
        block: &'c Block<'c>,
        program: &'c Program,
    ) -> Result<Self> {
        let intrinsics = Intrinsics::declare(context);
        let builder = OpBuilder::new(&context.mlir_context);

        let uint256 = builder.uint256_ty();
        let ptr_type = builder.ptr_ty();
        let location = builder.unknown_loc();

        let syscall_ctx = block.add_argument(intrinsics.ptr_ty, location);
        let gas_counter_ptr = block.add_argument(intrinsics.ptr_ty, location);
        let stack_ptr = block.add_argument(intrinsics.ptr_ty, location);
        let stack_size_ptr = block.add_argument(intrinsics.ptr_ty, location);

        SetupBuilder::new(&context.mlir_context, module).declare_symbols()?;

        let array_size = block
            .append_operation(builder.iconst_64(0))
            .result(0)?
            .into();
        let stack_top_ptr = block
            .append_operation(llvm::alloca(
                &context.mlir_context,
                array_size,
                builder.ptr_ty(),
                location,
                AllocaOptions::new().elem_type(Some(TypeAttribute::new(ptr_type))),
            ))
            .result(0)?
            .into();
        block.append_operation(builder.store(stack_ptr, stack_top_ptr));

        let revert_block = region.append_block(revert_block(
            &context.mlir_context,
            syscall_ctx,
            gas_counter_ptr,
        )?);
        let jumptable_block = region.append_block(Block::new(&[(uint256, location)]));
        let return_block = region.append_block(return_block(&context.mlir_context)?);
        Ok(LegacyCtxType {
            context: &context.mlir_context,
            program,
            values: CtxValues {
                syscall_ctx,
                gas_counter_ptr,
                stack_ptr,
                stack_top_ptr,
                stack_size_ptr,
            },
            revert_block,
            stop_block: return_block,
            jumptable_block,
            jumpdest_blocks: Default::default(),
        })
    }

    /// Creates a new instance of `CtxType` with the specified parameters for the op function
    fn new_op_func_ctx(
        context: &'c Context,
        region: &'c Region,
        block: &'c Block<'c>,
        program: &'c Program,
    ) -> Result<Self> {
        let intrinsics = Intrinsics::declare(context);
        let location = Location::unknown(&context.mlir_context);
        let syscall_ctx = block.add_argument(intrinsics.ptr_ty, location);
        let gas_counter_ptr = block.add_argument(intrinsics.ptr_ty, location);
        let stack_ptr = block.add_argument(intrinsics.ptr_ty, location);
        let stack_size_ptr = block.add_argument(intrinsics.ptr_ty, location);
        let stack_top_ptr = block.add_argument(intrinsics.ptr_ty, location);
        let revert_block = region.append_block(revert_block(
            &context.mlir_context,
            syscall_ctx,
            gas_counter_ptr,
        )?);
        let return_block = region.append_block(return_block(&context.mlir_context)?);
        Ok(LegacyCtxType {
            context: &context.mlir_context,
            program,
            values: CtxValues {
                syscall_ctx,
                gas_counter_ptr,
                stack_ptr,
                stack_top_ptr,
                stack_size_ptr,
            },
            revert_block,
            stop_block: return_block,
            // Note that we perform jump processing at all calls to the op function rather than within
            // the op function itself, so this is an impossible situation.
            jumptable_block: revert_block,
            jumpdest_blocks: Default::default(),
        })
    }

    /// Populates the jump table block with the jump destinations.
    ///
    /// This function iterates through the operations in the program to find
    /// jump destination operations, creating a switch operation in the jump
    /// table block. It also verifies the created operation.
    ///
    /// # Returns
    /// A `Result<()>` indicating success or failure of the operation.
    fn populate_jumptable(&self) -> Result<()> {
        let context = self.context;
        let program = self.program;
        let block = self.jumptable_block;

        let location = Location::unknown(context);
        let uint256 = IntegerType::new(context, 256);
        let uint8 = IntegerType::new(context, 8);

        let jumpdest_pcs: Vec<i64> = program
            .operations
            .iter()
            .filter_map(|op| match op {
                Operation::Jumpdest { pc } => Some(*pc as i64),
                _ => None,
            })
            .collect();

        let arg = block.argument(0)?;

        let case_destinations: Vec<_> = self
            .jumpdest_blocks
            .values()
            .map(|b| {
                let x: (&Block, &[Value]) = (b, &[]);
                x
            })
            .collect();

        let code = block
            .append_operation(arith::constant(
                context,
                IntegerAttribute::new(uint8.into(), ExitStatusCode::InvalidJump.to_u8() as i64)
                    .into(),
                location,
            ))
            .result(0)?;

        block.append_operation(cf::switch(
            context,
            &jumpdest_pcs,
            arg.into(),
            uint256.into(),
            (&self.revert_block, &[code.into()]),
            &case_destinations,
            location,
        )?);

        Ok(())
    }

    /// Registers a jump destination block for a given program counter.
    ///
    /// This function adds the specified block as a jump destination for the
    /// given program counter. The registered blocks will be used during code
    /// generation for jump operations.
    ///
    /// # Parameters
    /// * `pc` - The program counter associated with the jump destination.
    /// * `block` - A reference to the block that acts as the jump destination.
    #[inline]
    fn register_jump_destination(&mut self, pc: usize, block: BlockRef<'c, 'c>) {
        self.jumpdest_blocks.insert(pc, block);
    }

    /// Adds a jump operation to the specified block.
    ///
    /// This function appends a branch operation to the specified block that
    /// directs control flow to the jump table block based on the provided
    /// program counter value.
    ///
    /// # Parameters
    /// * `block` - A reference to the block to which the jump operation will be added.
    /// * `pc_to_jump_to` - The program counter value to jump to.
    /// * `location` - The location context for the operation.
    #[inline]
    fn add_jump_op(&mut self, block: BlockRef<'c, 'c>, pc_to_jump_to: Value, location: Location) {
        block.append_operation(cf::br(&self.jumptable_block, &[pc_to_jump_to], location));
    }
}

/// The [`EofCtxType`] struct holds the necessary context and data structures for managing
/// the execution environment when compiling or interpreting EVM (Ethereum Virtual Machine)
/// bytecode using MLIR. It encapsulates references to essential components like the MLIR context,
/// the program being executed, values, and blocks for control flow management.
///
/// # Fields:
/// - `context`: A reference to the `MLIRContext` that manages the lifetime and state of the
///   intermediate representation.
/// - `program`: A reference to the current `Program` being executed or compiled.
/// - `values`: A set of context-specific values, such as the remaining gas and syscall context,
///   which are crucial during the execution of EVM bytecode.
/// - `revert_block`: A reference to a block used for handling reverts (exceptions or errors) in
///   the EVM execution model.
/// - `jumptable_block`: A reference to a block that handles jump table operations for dynamic
///   control flow in EVM bytecode.
/// - `jumpdest_blocks`: A map that associates EVM jump destination indices with their corresponding
///   blocks in MLIR.
///
/// # Purpose:
/// The [`EofCtxType`] struct acts as the primary context for managing the execution of EVM bytecode.
/// It provides a unified structure that encapsulates control flow blocks and essential values,
/// facilitating smooth execution and handling of jumps, exceptions, and system calls.
///
/// # Example Usage:
/// ```no_check
/// let ctx_type = EofCtxType {
///     context: &mlir_context,
///     program: &evm_program,
///     values: CtxValues {
///         syscall_ctx: syscall_value,
///         remaining_gas: gas_value,
///     },
///     revert_block: revert_block_ref,
///     jumptable_block: jumptable_block_ref,
///     jumpdest_blocks: jumpdest_map,
/// };
/// ```
///
/// # Notes:
/// - `EofCtxType` is integral for managing the flow of execution in the context of EVM bytecode,
///   particularly handling jumps, system calls, and reverts efficiently within the MLIR infrastructure.
#[derive(Debug)]
pub struct EofCtxType<'c> {
    context: &'c MLIRContext,

    program: &'c Program,

    values: CtxValues<'c>,

    revert_block: BlockRef<'c, 'c>,

    stop_block: BlockRef<'c, 'c>,

    jumptable_block: BlockRef<'c, 'c>,

    jumpdest_blocks: BTreeMap<usize, BlockRef<'c, 'c>>,
}

impl<'c> CtxType<'c> for EofCtxType<'c> {
    fn context(&self) -> &MLIRContext {
        &self.context
    }

    fn program(&self) -> &Program {
        &self.program
    }

    fn values(&self) -> &CtxValues {
        &self.values
    }

    fn revert_block(&self) -> BlockRef {
        self.revert_block
    }

    fn stop_block(&self) -> BlockRef {
        self.stop_block
    }

    fn jumptable_block(&self) -> BlockRef {
        self.jumptable_block
    }

    fn jumpdest_blocks(&self) -> &BTreeMap<usize, BlockRef> {
        &self.jumpdest_blocks
    }

    /// Creates a new instance of `CtxType` with the specified parameters.
    ///
    /// This constructor initializes the context for code generation, setting up
    /// the necessary components such as syscall context, initial gas, and symbol
    /// declarations. It also prepares the revert and jump table blocks.
    ///
    /// # Parameters
    /// * `context` - A reference to the MLIR context.
    /// * `module` - A reference to the MLIR module.
    /// * `region` - A reference to the region in which to create blocks.
    /// * `block` - A reference to the block in which operations will be generated.
    /// * `program` - A reference to the program being compiled.
    ///
    /// # Returns
    /// A `Result<Self>` containing the new `CtxType` instance on success, or an
    /// error if the initialization fails.
    fn new_main_func_ctx(
        context: &'c Context,
        module: &'c Module,
        region: &'c Region,
        block: &'c Block<'c>,
        program: &'c Program,
    ) -> Result<Self> {
        let intrinsics = Intrinsics::declare(context);
        let builder = OpBuilder::new(&context.mlir_context);

        let uint256 = builder.uint256_ty();
        let ptr_type = builder.ptr_ty();
        let location = builder.unknown_loc();

        let syscall_ctx = block.add_argument(intrinsics.ptr_ty, location);
        let gas_counter_ptr = block.add_argument(intrinsics.ptr_ty, location);
        let stack_ptr = block.add_argument(intrinsics.ptr_ty, location);
        let stack_size_ptr = block.add_argument(intrinsics.ptr_ty, location);

        SetupBuilder::new(&context.mlir_context, module).declare_symbols()?;

        let array_size = block
            .append_operation(builder.iconst_64(0))
            .result(0)?
            .into();
        let stack_top_ptr = block
            .append_operation(llvm::alloca(
                &context.mlir_context,
                array_size,
                builder.ptr_ty(),
                location,
                AllocaOptions::new().elem_type(Some(TypeAttribute::new(ptr_type))),
            ))
            .result(0)?
            .into();
        block.append_operation(builder.store(stack_ptr, stack_top_ptr));

        let revert_block = region.append_block(revert_block(
            &context.mlir_context,
            syscall_ctx,
            gas_counter_ptr,
        )?);
        let jumptable_block = region.append_block(Block::new(&[(uint256, location)]));
        let return_block = region.append_block(return_block(&context.mlir_context)?);
        Ok(EofCtxType {
            context: &context.mlir_context,
            program,
            values: CtxValues {
                syscall_ctx,
                gas_counter_ptr,
                stack_ptr,
                stack_top_ptr,
                stack_size_ptr,
            },
            revert_block,
            stop_block: return_block,
            jumptable_block,
            jumpdest_blocks: Default::default(),
        })
    }

    /// Creates a new instance of `CtxType` with the specified parameters for the op function
    fn new_op_func_ctx(
        context: &'c Context,
        region: &'c Region,
        block: &'c Block<'c>,
        program: &'c Program,
    ) -> Result<Self> {
        let intrinsics = Intrinsics::declare(context);
        let location = Location::unknown(&context.mlir_context);
        let syscall_ctx = block.add_argument(intrinsics.ptr_ty, location);
        let gas_counter_ptr = block.add_argument(intrinsics.ptr_ty, location);
        let stack_ptr = block.add_argument(intrinsics.ptr_ty, location);
        let stack_size_ptr = block.add_argument(intrinsics.ptr_ty, location);
        let stack_top_ptr = block.add_argument(intrinsics.ptr_ty, location);
        let revert_block = region.append_block(revert_block(
            &context.mlir_context,
            syscall_ctx,
            gas_counter_ptr,
        )?);
        let return_block = region.append_block(return_block(&context.mlir_context)?);
        Ok(EofCtxType {
            context: &context.mlir_context,
            program,
            values: CtxValues {
                syscall_ctx,
                gas_counter_ptr,
                stack_ptr,
                stack_top_ptr,
                stack_size_ptr,
            },
            revert_block,
            stop_block: return_block,
            // Note that we perform jump processing at all calls to the op function rather than within
            // the op function itself, so this is an impossible situation.
            jumptable_block: revert_block,
            jumpdest_blocks: Default::default(),
        })
    }

    /// Populates the jump table block with the jump destinations.
    ///
    /// This function iterates through the operations in the program to find
    /// jump destination operations, creating a switch operation in the jump
    /// table block. It also verifies the created operation.
    ///
    /// # Returns
    /// A `Result<()>` indicating success or failure of the operation.
    fn populate_jumptable(&self) -> Result<()> {
        let context = self.context;
        let program = self.program;
        let block = self.jumptable_block;

        let location = Location::unknown(context);
        let uint256 = IntegerType::new(context, 256);
        let uint8 = IntegerType::new(context, 8);

        let jumpdest_pcs: Vec<i64> = program
            .operations
            .iter()
            .filter_map(|op| match op {
                Operation::Jumpdest { pc } => Some(*pc as i64),
                _ => None,
            })
            .collect();

        let arg = block.argument(0)?;

        let case_destinations: Vec<_> = self
            .jumpdest_blocks
            .values()
            .map(|b| {
                let x: (&Block, &[Value]) = (b, &[]);
                x
            })
            .collect();

        let code = block
            .append_operation(arith::constant(
                context,
                IntegerAttribute::new(uint8.into(), ExitStatusCode::InvalidJump.to_u8() as i64)
                    .into(),
                location,
            ))
            .result(0)?;

        block.append_operation(cf::switch(
            context,
            &jumpdest_pcs,
            arg.into(),
            uint256.into(),
            (&self.revert_block, &[code.into()]),
            &case_destinations,
            location,
        )?);

        Ok(())
    }

    /// Registers a jump destination block for a given program counter.
    ///
    /// This function adds the specified block as a jump destination for the
    /// given program counter. The registered blocks will be used during code
    /// generation for jump operations.
    ///
    /// # Parameters
    /// * `pc` - The program counter associated with the jump destination.
    /// * `block` - A reference to the block that acts as the jump destination.
    #[inline]
    fn register_jump_destination(&mut self, pc: usize, block: BlockRef<'c, 'c>) {
        self.jumpdest_blocks.insert(pc, block);
    }

    /// Adds a jump operation to the specified block.
    ///
    /// This function appends a branch operation to the specified block that
    /// directs control flow to the jump table block based on the provided
    /// program counter value.
    ///
    /// # Parameters
    /// * `block` - A reference to the block to which the jump operation will be added.
    /// * `pc_to_jump_to` - The program counter value to jump to.
    /// * `location` - The location context for the operation.
    #[inline]
    fn add_jump_op(&mut self, block: BlockRef<'c, 'c>, pc_to_jump_to: Value, location: Location) {
        block.append_operation(cf::br(&self.jumptable_block, &[pc_to_jump_to], location));
    }
}

/// Creates a revert block that handles error conditions.
///
/// This function constructs a block that represents a revert operation in
/// the EVM context. It sets up the necessary operations to indicate an
/// error condition and returns control to the calling context with a specified
/// reason.
///
/// # Parameters
/// * `context` - A reference to the MLIR context used for operation creation.
/// * `syscall_ctx` - The syscall context value used for interfacing with the system.
/// * `_remaining_gas` - The remaining gas value (currently not checked).
///
/// # Returns
/// A `Result<Block<'c>>` containing the newly created revert block on success,
/// or an error if block creation fails.
///
/// # Operations
/// The created block will include:
/// - Initialization of zero constants for 32-bit and 64-bit integers.
/// - Creation of a reason constant based on the exit status code for errors.
/// - A call to `WRITE_RESULT` with the syscall context and error information.
/// - A return operation that provides the reason for the revert.
pub fn revert_block<'c>(
    context: &'c MLIRContext,
    syscall_ctx: Value<'c, 'c>,
    gas_counter_ptr: Value<'c, 'c>,
) -> Result<Block<'c>> {
    let builder = OpBuilder::new(context);
    let uint8 = builder.uint8_ty();
    let uint64 = builder.uint64_ty();
    let block = Block::new(&[(uint8, builder.unknown_loc())]);
    let location = builder.unknown_loc();

    let zero = block
        .append_operation(builder.iconst_64(0))
        .result(0)?
        .into();
    let reason = block.argument(0)?.into();

    let gas_counter = block
        .append_operation(builder.load(gas_counter_ptr, uint64))
        .result(0)?
        .into();

    block.append_operation(func::call(
        context,
        FlatSymbolRefAttribute::new(context, symbols::WRITE_RESULT),
        &[syscall_ctx, zero, zero, gas_counter, reason],
        &[],
        location,
    ));
    block.append_operation(func::r#return(&[reason], location));
    Ok(block)
}

/// Creates a return block that handles return codes.
pub fn return_block(context: &MLIRContext) -> Result<Block<'_>> {
    let builder = OpBuilder::new(context);
    let uint8 = builder.uint8_ty();
    let block = Block::new(&[(uint8, builder.unknown_loc())]);
    let location = builder.unknown_loc();
    let reason = block.argument(0)?.into();
    block.append_operation(func::r#return(&[reason], location));
    Ok(block)
}

/// The `SetupBuilder` struct is used to initialize and set up the execution environment within MLIR.
/// It encapsulates the MLIR context, module, block, and an operation builder used to generate
/// the necessary operations for the execution of EVM bytecode.
///
/// # Fields:
/// - `context`: A reference to the `MLIRContext` that manages types, operations, and modules.
/// - `module`: A reference to the `Module` that contains the operations generated during execution.
/// - `block`: A reference to the `Block` in which operations are being generated.
/// - `builder`: A reference to the `OpBuilder` that creates MLIR operations.
/// - `location`: The location information used for debugging purposes when generating MLIR operations.
///
/// # Purpose:
/// The `SetupBuilder` is responsible for setting up the initial environment and generating the operations
/// necessary for the execution of EVM bytecode within MLIR. It provides a convenient way to manage
/// the context, module, and builder required for operation generation.
///
/// # Example Usage:
/// ```no_check
/// let setup_builder = SetupBuilder {
///     context: &mlir_context,
///     module: &module,
///     block: &entry_block,
///     builder: &op_builder,
///     location: location,
/// };
/// ```
///
/// # Notes:
/// - The `SetupBuilder` simplifies the process of setting up and managing the necessary components
///   for generating and executing MLIR operations.
pub struct SetupBuilder<'c> {
    /// The MLIR context used for managing types, operations, and modules.
    context: &'c MLIRContext,
    /// The module that contains the operations generated during execution.
    module: &'c Module<'c>,
}

impl<'c> SetupBuilder<'c> {
    /// Creates a new instance of `SetupBuilder`.
    ///
    /// # Parameters
    /// * `context` - A reference to the MLIR context for operation creation.
    /// * `module` - A reference to the MLIR module where operations will be added.
    /// * `block` - A reference to the MLIR block where operations will be inserted.
    /// * `op_builder` - A reference to the operation builder used for creating operations.
    ///
    /// # Returns
    /// A new instance of `SetupBuilder`.
    pub fn new(context: &'c MLIRContext, module: &'c Module<'c>) -> Self {
        Self { context, module }
    }

    /// Declares the necessary symbols within the module.
    ///
    /// This method sets up the symbol context for further operations.
    ///
    /// # Returns
    /// A reference to `self` for method chaining.
    ///
    /// # Errors
    /// Returns an error if symbol declaration fails.
    #[inline]
    pub fn declare_symbols(&self) -> Result<&Self> {
        symbols_ctx::declare_symbols(self.context, self.module);
        Ok(self)
    }
}
