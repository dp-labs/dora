use super::memory;
use crate::{
    arith_constant,
    conversion::{rewriter::Rewriter, walker::walk_operation},
    create_var,
    dora::utils,
    errors::Result,
    load_by_addr,
    value::IntoContextOperation,
};
use cost::get_static_cost_from_op;
use dora_primitives::spec::SpecId;
use dora_runtime::{
    constants::{
        self,
        gas_cost::{self, INIT_WORD_COST, MAX_INITCODE_SIZE},
    },
    ExitStatusCode,
};
use melior::{
    dialect::{
        arith::{self, CmpiPredicate},
        cf,
        llvm::{self, AllocaOptions, LoadStoreOptions},
        scf,
    },
    ir::{
        attribute::{IntegerAttribute, TypeAttribute},
        OperationRef, *,
    },
    Context,
};
use revmc::{op_info_map, OpcodeInfo};

/// Represents a pass that processes gas metering and tracks Dora IR operations in a program.
/// The `GasPass` is used to ensure that gas calculations are correctly handled for specific
/// operations that may affect resource consumption during execution.
///
/// # Example Usage:
/// ```no_check
/// let mut gas_pass = GasPass {};
/// gas_pass.run(operation_ref).expect("Gas metering failed");
/// ```
///
/// # Notes:
/// - The `GasPass` struct processes a series of Dora IR operations, especially arithmetic, logical,
///   and memory-related operations, to account for their gas cost within a Wasm execution context.
/// - It iterates through operations, checking for specific Dora IR operations like `dora.add`,
///   `dora.sub`, `dora.mul`, and others, and integrates gas metering logic.
///
/// # See Also:
/// - The list of supported Dora IR operations includes arithmetic operations (`Add`, `Sub`, `Mul`, etc.),
///   comparison operations (`Lt`, `Gt`, `Eq`, etc.), and memory-related operations (`Address`, `Caller`, `Balance`, etc.).
#[derive(Clone, Debug)]
pub struct GasPass<'c> {
    /// A reference to the MLIR context, which manages global state and resources required for MLIR operations.
    pub ctx: &'c Context,
    pub options: GasOptions,
}

#[derive(Clone, Debug)]
pub struct GasOptions {
    pub spec_id: SpecId,
    pub limit_contract_code_size: Option<usize>,
}

impl<'c> GasPass<'c> {
    /// Runs the gas metering pass on a given operation, processing the Dora IR operations in the program
    /// and integrating the necessary gas calculations.
    ///
    /// # Parameters:
    /// - `operation`: A reference to the operation that will be processed.
    ///
    /// # Returns:
    /// - `Result<()>`: Returns `Ok` if the gas metering pass was successful, or an error if it failed.
    ///
    /// # Example:
    /// ```no_check
    /// gas_pass.run(operation_ref).expect("Gas metering failed");
    /// ```
    ///
    /// # Notes:
    /// - This method walks through the operations in the program, identifies Dora IR operations that
    ///   require gas metering (e.g., arithmetic operations like `dora.add`, `dora.sub`, and `dora.mul`),
    ///   and integrates logic to track gas consumption.
    pub fn run(&mut self, operation: OperationRef<'_, '_>) -> Result<()> {
        let mut dora_ops = vec![];
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

        // module
        // - setup_block(syscall_ctx, init_gas)
        // - revert_block
        // - dora operation blocks

        let spec_id = self.options.spec_id;
        let op_infos = op_info_map(spec_id);
        for op in &dora_ops {
            if let Some(block) = op.block() {
                if let Some(region) = block.parent_region() {
                    if let Some(setup_block) = region.first_block() {
                        if let Some(revert_block) = setup_block.next_in_region() {
                            let name = op.name().as_string_ref().as_str().unwrap().to_string();
                            if let Ok(dora_op) = dora_ir::Operation::try_from(name.as_str()) {
                                let info: OpcodeInfo = op_infos[dora_op.clone() as usize];

                                if info.is_unknown() || info.is_disabled() {
                                    continue;
                                }

                                if info.is_dynamic() {
                                    match dora_op {
                                        dora_ir::Operation::Balance => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                &op.next_in_block().unwrap(),
                                                block,
                                                revert_block,
                                                |_rewriter| Ok(op.result(0)?.into()),
                                            )?;
                                        }
                                        dora_ir::Operation::Exp => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                |rewriter| {
                                                    let exponent = op.operand(1)?;
                                                    let location = rewriter.get_insert_location();
                                                    let zero =
                                                    rewriter.make(rewriter.iconst_256_from_u64(0)?)?;
                                                    let is_exponent_zero =
                                                        rewriter.make(arith::cmpi(
                                                            rewriter.context(),
                                                            CmpiPredicate::Eq,
                                                            exponent,
                                                            zero,
                                                            location,
                                                        ))?;

                                                    let total_gas_cost =
                                                        rewriter.make(scf::r#if(
                                                            is_exponent_zero,
                                                            &[rewriter.intrinsics.i64_ty],
                                                            {
                                                                let region = Region::new();
                                                                let block = region
                                                                    .append_block(Block::new(&[]));
                                                                let rewriter =
                                                                    Rewriter::new_with_block(
                                                                        rewriter.context(),
                                                                        block,
                                                                    );
                                                                rewriter.create(scf::r#yield(
                                                                    &[rewriter.make(rewriter.iconst_64(0))?],
                                                                    location,
                                                                ));
                                                                region
                                                            },
                                                            {
                                                                let region = Region::new();
                                                                let block = region
                                                                    .append_block(Block::new(&[]));
                                                                let rewriter =
                                                                    Rewriter::new_with_block(
                                                                        rewriter.context(),
                                                                        block,
                                                                    );
                                                                let leading_zeros = rewriter.make(
                                                                    llvm::intr_ctlz(
                                                                        rewriter.context(),
                                                                        exponent,
                                                                        false,
                                                                        rewriter.intrinsics.i256_ty,
                                                                        location,
                                                                    ),
                                                                )?;
                                                                let number_of_bits =
                                                                rewriter.make(arith::subi(
                                                                    rewriter.make(
                                                                        rewriter
                                                                            .iconst_256_from_u64(
                                                                                256,
                                                                            )?,
                                                                    )?,
                                                                    leading_zeros,
                                                                    location,
                                                                ))?;
                                                                let bits_with_offset =
                                                                rewriter.make(arith::addi(
                                                                    number_of_bits,
                                                                    rewriter.make(
                                                                        rewriter
                                                                            .iconst_256_from_u64(
                                                                                7,
                                                                            )?,
                                                                    )?,
                                                                    location,
                                                                ))?;
                                                                let number_of_bytes =
                                                                rewriter.make(arith::divui(
                                                                    bits_with_offset,
                                                                    rewriter.make(
                                                                        rewriter
                                                                            .iconst_256_from_u64(
                                                                                8,
                                                                            )?,
                                                                    )?,
                                                                    location,
                                                                ))?;
                                                                let dynamic_gas_cost =
                                                                rewriter.make(arith::muli(
                                                                    number_of_bytes,
                                                                    rewriter.make(
                                                                        rewriter.iconst_256_from_u64(
                                                                            if spec_id.is_enabled_in(
                                                                                SpecId::SPURIOUS_DRAGON,
                                                                            ) {
                                                                                50
                                                                            } else {
                                                                                10
                                                                            },
                                                                        )?,
                                                                    )?,
                                                                    location,
                                                                ))?;

                                                                let total_gas_cost =
                                                                rewriter.make(arith::addi(
                                                                    rewriter.make(
                                                                        rewriter
                                                                            .iconst_256_from_u64(
                                                                                gas_cost::EXP
                                                                                    as u64,
                                                                            )?,
                                                                    )?,
                                                                    dynamic_gas_cost,
                                                                    location,
                                                                ))?;

                                                                let total_gas_cost = rewriter
                                                                    .make(arith::trunci(
                                                                        total_gas_cost,
                                                                        rewriter.intrinsics.i64_ty,
                                                                        location,
                                                                    ))?;
                                                                rewriter.create(scf::r#yield(
                                                                    &[total_gas_cost],
                                                                    location,
                                                                ));
                                                                region
                                                            },
                                                            location,
                                                        ))?;

                                                    Ok(total_gas_cost)
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::Keccak256 => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                |rewriter| {
                                                    let size = op.operand(1)?;
                                                    let location = rewriter.get_insert_location();
                                                    let size = rewriter.make(arith::trunci(
                                                        size,
                                                        rewriter.intrinsics.i64_ty,
                                                        location,
                                                    ))?;
                                                    let dynamic_gas_cost =
                                                        compute_copy_cost(rewriter, size)?;
                                                    let constant_2 =
                                                        rewriter.make(rewriter.iconst_64(2))?;
                                                    let total_gas_cost =
                                                        rewriter.make(arith::muli(
                                                            dynamic_gas_cost,
                                                            constant_2,
                                                            location,
                                                        ))?;
                                                    Ok(total_gas_cost)
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::CallDataCopy => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
                                                    let dest_offset = op.operand(0)?;
                                                    let dest_offset =
                                                        rewriter.make(arith::trunci(
                                                            dest_offset,
                                                            rewriter.intrinsics.i64_ty,
                                                            location,
                                                        ))?;
                                                    let size = op.operand(2)?;
                                                    let size = rewriter.make(arith::trunci(
                                                        size,
                                                        rewriter.intrinsics.i64_ty,
                                                        location,
                                                    ))?;
                                                    let required_size = rewriter.make(
                                                        arith::addi(dest_offset, size, location),
                                                    )?;
                                                    let total_gas_cost =
                                                        memory::resize_memory_with_gas_cost(
                                                            rewriter,
                                                            required_size,
                                                            gas_cost::CALLDATACOPY,
                                                        )?;
                                                    Ok(total_gas_cost)
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::CodeCopy => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
                                                    let dest_offset = op.operand(0)?;
                                                    let dest_offset =
                                                        rewriter.make(arith::trunci(
                                                            dest_offset,
                                                            rewriter.intrinsics.i64_ty,
                                                            location,
                                                        ))?;
                                                    let size = op.operand(2)?;
                                                    let size = rewriter.make(arith::trunci(
                                                        size,
                                                        rewriter.intrinsics.i64_ty,
                                                        location,
                                                    ))?;
                                                    let required_size = rewriter.make(
                                                        arith::addi(dest_offset, size, location),
                                                    )?;
                                                    let total_gas_cost =
                                                        memory::resize_memory_with_gas_cost(
                                                            rewriter,
                                                            required_size,
                                                            gas_cost::CODECOPY,
                                                        )?;
                                                    let dynamic_gas_cost =
                                                        compute_copy_cost(rewriter, size)?;
                                                    let total_gas_cost =
                                                        rewriter.make(arith::addi(
                                                            total_gas_cost,
                                                            dynamic_gas_cost,
                                                            location,
                                                        ))?;
                                                    Ok(total_gas_cost)
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::ExtCodeSize => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                &op.next_in_block().unwrap(),
                                                block,
                                                revert_block,
                                                |_rewriter| Ok(op.result(0)?.into()),
                                            )?;
                                            self.insert_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                gas_cost::EXTCODESIZE_WARM,
                                            )?;
                                        }
                                        dora_ir::Operation::ExtCodeCopy => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
                                                    let dest_offset = op.operand(1)?;
                                                    let dest_offset =
                                                        rewriter.make(arith::trunci(
                                                            dest_offset,
                                                            rewriter.intrinsics.i64_ty,
                                                            location,
                                                        ))?;
                                                    let size = op.operand(3)?;
                                                    let size = rewriter.make(arith::trunci(
                                                        size,
                                                        rewriter.intrinsics.i64_ty,
                                                        location,
                                                    ))?;
                                                    let required_size = rewriter.make(
                                                        arith::addi(dest_offset, size, location),
                                                    )?;
                                                    let total_gas_cost =
                                                        memory::resize_memory_with_gas_cost(
                                                            rewriter,
                                                            required_size,
                                                            gas_cost::EXTCODECOPY_WARM,
                                                        )?;
                                                    let dynamic_gas_cost =
                                                        compute_copy_cost(rewriter, size)?;
                                                    let total_gas_cost =
                                                        rewriter.make(arith::addi(
                                                            total_gas_cost,
                                                            dynamic_gas_cost,
                                                            location,
                                                        ))?;
                                                    Ok(total_gas_cost)
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::ReturnDataCopy => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
                                                    let size = op.operand(3)?;
                                                    let size = rewriter.make(arith::trunci(
                                                        size,
                                                        rewriter.intrinsics.i64_ty,
                                                        location,
                                                    ))?;
                                                    let dynamic_gas_cost =
                                                        compute_copy_cost(rewriter, size)?;
                                                    Ok(dynamic_gas_cost)
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::ReturnDataLoad => todo!(),
                                        dora_ir::Operation::ExtCodeHash => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                &op.next_in_block().unwrap(),
                                                block,
                                                revert_block,
                                                |_rewriter| Ok(op.result(0)?.into()),
                                            )?;
                                        }
                                        dora_ir::Operation::MLoad => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
                                                    let dest_offset = op.operand(0)?;
                                                    let dest_offset =
                                                        rewriter.make(arith::trunci(
                                                            dest_offset,
                                                            rewriter.intrinsics.i64_ty,
                                                            location,
                                                        ))?;
                                                    let size =
                                                        rewriter.make(rewriter.iconst_32(32))?;
                                                    let required_size = rewriter.make(
                                                        arith::addi(dest_offset, size, location),
                                                    )?;
                                                    let total_gas_cost =
                                                        memory::resize_memory_with_gas_cost(
                                                            rewriter,
                                                            required_size,
                                                            gas_cost::MLOAD,
                                                        )?;
                                                    Ok(total_gas_cost)
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::MStore => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
                                                    let dest_offset = op.operand(0)?;
                                                    let dest_offset =
                                                        rewriter.make(arith::trunci(
                                                            dest_offset,
                                                            rewriter.intrinsics.i64_ty,
                                                            location,
                                                        ))?;
                                                    let size =
                                                        rewriter.make(rewriter.iconst_32(32))?;
                                                    let required_size = rewriter.make(
                                                        arith::addi(dest_offset, size, location),
                                                    )?;
                                                    let total_gas_cost =
                                                        memory::resize_memory_with_gas_cost(
                                                            rewriter,
                                                            required_size,
                                                            gas_cost::MSTORE,
                                                        )?;
                                                    Ok(total_gas_cost)
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::MStore8 => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
                                                    let dest_offset = op.operand(0)?;
                                                    let dest_offset =
                                                        rewriter.make(arith::trunci(
                                                            dest_offset,
                                                            rewriter.intrinsics.i64_ty,
                                                            location,
                                                        ))?;
                                                    let size =
                                                        rewriter.make(rewriter.iconst_32(8))?;
                                                    let required_size = rewriter.make(
                                                        arith::addi(dest_offset, size, location),
                                                    )?;
                                                    let total_gas_cost =
                                                        memory::resize_memory_with_gas_cost(
                                                            rewriter,
                                                            required_size,
                                                            gas_cost::MSTORE8,
                                                        )?;
                                                    Ok(total_gas_cost)
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::SStore => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                |_rewriter| {
                                                    let dynamic_gas_cost = op.result(0)?.into();
                                                    Ok(dynamic_gas_cost)
                                                },
                                            )?;
                                            // TODO: Check that (gas_counter - needed_gas) >= SSTORE_MIN_REMAINING_GAS
                                        }
                                        dora_ir::Operation::MCopy => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
                                                    let dest_offset = op.operand(0)?;
                                                    let dest_offset =
                                                        rewriter.make(arith::trunci(
                                                            dest_offset,
                                                            rewriter.intrinsics.i64_ty,
                                                            location,
                                                        ))?;
                                                    let offset = op.operand(1)?;
                                                    let offset = rewriter.make(arith::trunci(
                                                        offset,
                                                        rewriter.intrinsics.i64_ty,
                                                        location,
                                                    ))?;
                                                    let size = op.operand(2)?;
                                                    let size = rewriter.make(arith::trunci(
                                                        size,
                                                        rewriter.intrinsics.i64_ty,
                                                        location,
                                                    ))?;
                                                    // required_size = offset + size
                                                    let src_required_size = rewriter.make(
                                                        arith::addi(offset, size, location),
                                                    )?;
                                                    // dest_required_size = dest_offset + size
                                                    let dest_required_size = rewriter.make(
                                                        arith::addi(dest_offset, size, location),
                                                    )?;
                                                    let required_size =
                                                        rewriter.make(arith::maxui(
                                                            src_required_size,
                                                            dest_required_size,
                                                            location,
                                                        ))?;
                                                    let total_gas_cost =
                                                        memory::resize_memory_with_gas_cost(
                                                            rewriter,
                                                            required_size,
                                                            gas_cost::MCOPY,
                                                        )?;
                                                    let dynamic_gas_cost =
                                                        compute_copy_cost(rewriter, size)?;
                                                    let total_gas_cost =
                                                        rewriter.make(arith::addi(
                                                            total_gas_cost,
                                                            dynamic_gas_cost,
                                                            location,
                                                        ))?;
                                                    Ok(total_gas_cost)
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::Log0 => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
                                                    let dest_offset = op.operand(0)?;
                                                    let dest_offset =
                                                        rewriter.make(arith::trunci(
                                                            dest_offset,
                                                            rewriter.intrinsics.i64_ty,
                                                            location,
                                                        ))?;
                                                    let size = op.operand(1)?;
                                                    let size = rewriter.make(arith::trunci(
                                                        size,
                                                        rewriter.intrinsics.i64_ty,
                                                        location,
                                                    ))?;
                                                    let required_size = rewriter.make(
                                                        arith::addi(dest_offset, size, location),
                                                    )?;
                                                    let total_gas_cost =
                                                        memory::resize_memory_with_gas_cost(
                                                            rewriter,
                                                            required_size,
                                                            gas_cost::LOG0,
                                                        )?;
                                                    let dynamic_gas_cost =
                                                        compute_log_dynamic_gas(rewriter, 0, size)?;
                                                    let total_gas_cost =
                                                        rewriter.make(arith::addi(
                                                            total_gas_cost,
                                                            dynamic_gas_cost,
                                                            location,
                                                        ))?;
                                                    Ok(total_gas_cost)
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::Log1 => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
                                                    let dest_offset = op.operand(0)?;
                                                    let dest_offset =
                                                        rewriter.make(arith::trunci(
                                                            dest_offset,
                                                            rewriter.intrinsics.i64_ty,
                                                            location,
                                                        ))?;
                                                    let size = op.operand(1)?;
                                                    let size = rewriter.make(arith::trunci(
                                                        size,
                                                        rewriter.intrinsics.i64_ty,
                                                        location,
                                                    ))?;
                                                    let required_size = rewriter.make(
                                                        arith::addi(dest_offset, size, location),
                                                    )?;
                                                    let total_gas_cost =
                                                        memory::resize_memory_with_gas_cost(
                                                            rewriter,
                                                            required_size,
                                                            gas_cost::LOG1,
                                                        )?;
                                                    let dynamic_gas_cost =
                                                        compute_log_dynamic_gas(rewriter, 1, size)?;
                                                    let total_gas_cost =
                                                        rewriter.make(arith::addi(
                                                            total_gas_cost,
                                                            dynamic_gas_cost,
                                                            location,
                                                        ))?;
                                                    Ok(total_gas_cost)
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::Log2 => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
                                                    let dest_offset = op.operand(0)?;
                                                    let dest_offset =
                                                        rewriter.make(arith::trunci(
                                                            dest_offset,
                                                            rewriter.intrinsics.i64_ty,
                                                            location,
                                                        ))?;
                                                    let size = op.operand(1)?;
                                                    let size = rewriter.make(arith::trunci(
                                                        size,
                                                        rewriter.intrinsics.i64_ty,
                                                        location,
                                                    ))?;
                                                    let required_size = rewriter.make(
                                                        arith::addi(dest_offset, size, location),
                                                    )?;
                                                    let total_gas_cost =
                                                        memory::resize_memory_with_gas_cost(
                                                            rewriter,
                                                            required_size,
                                                            gas_cost::LOG2,
                                                        )?;
                                                    let dynamic_gas_cost =
                                                        compute_log_dynamic_gas(rewriter, 2, size)?;
                                                    let total_gas_cost =
                                                        rewriter.make(arith::addi(
                                                            total_gas_cost,
                                                            dynamic_gas_cost,
                                                            location,
                                                        ))?;
                                                    Ok(total_gas_cost)
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::Log3 => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
                                                    let dest_offset = op.operand(0)?;
                                                    let dest_offset =
                                                        rewriter.make(arith::trunci(
                                                            dest_offset,
                                                            rewriter.intrinsics.i64_ty,
                                                            location,
                                                        ))?;
                                                    let size = op.operand(1)?;
                                                    let size = rewriter.make(arith::trunci(
                                                        size,
                                                        rewriter.intrinsics.i64_ty,
                                                        location,
                                                    ))?;
                                                    let required_size = rewriter.make(
                                                        arith::addi(dest_offset, size, location),
                                                    )?;
                                                    let total_gas_cost =
                                                        memory::resize_memory_with_gas_cost(
                                                            rewriter,
                                                            required_size,
                                                            gas_cost::LOG3,
                                                        )?;
                                                    let dynamic_gas_cost =
                                                        compute_log_dynamic_gas(rewriter, 3, size)?;
                                                    let total_gas_cost =
                                                        rewriter.make(arith::addi(
                                                            total_gas_cost,
                                                            dynamic_gas_cost,
                                                            location,
                                                        ))?;
                                                    Ok(total_gas_cost)
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::Log4 => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
                                                    let dest_offset = op.operand(0)?;
                                                    let dest_offset =
                                                        rewriter.make(arith::trunci(
                                                            dest_offset,
                                                            rewriter.intrinsics.i64_ty,
                                                            location,
                                                        ))?;
                                                    let size = op.operand(1)?;
                                                    let size = rewriter.make(arith::trunci(
                                                        size,
                                                        rewriter.intrinsics.i64_ty,
                                                        location,
                                                    ))?;
                                                    let required_size = rewriter.make(
                                                        arith::addi(dest_offset, size, location),
                                                    )?;
                                                    let total_gas_cost =
                                                        memory::resize_memory_with_gas_cost(
                                                            rewriter,
                                                            required_size,
                                                            gas_cost::LOG4,
                                                        )?;
                                                    let dynamic_gas_cost =
                                                        compute_log_dynamic_gas(rewriter, 4, size)?;
                                                    let total_gas_cost =
                                                        rewriter.make(arith::addi(
                                                            total_gas_cost,
                                                            dynamic_gas_cost,
                                                            location,
                                                        ))?;
                                                    Ok(total_gas_cost)
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::Create => {
                                            let _max_initcode_size = self
                                                .options
                                                .limit_contract_code_size
                                                .map(|limit| limit.saturating_mul(2))
                                                .unwrap_or(MAX_INITCODE_SIZE);

                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
                                                    let size = op.operand(2)?;
                                                    let size = rewriter.make(arith::trunci(
                                                        size,
                                                        rewriter.intrinsics.i64_ty,
                                                        location,
                                                    ))?;

                                                    let zero = rewriter
                                                        .make(rewriter.iconst_256_from_u64(0)?)?;
                                                    let is_size_zero =
                                                        rewriter.make(arith::cmpi(
                                                            rewriter.context(),
                                                            CmpiPredicate::Eq,
                                                            size,
                                                            zero,
                                                            location,
                                                        ))?;

                                                    let total_gas_cost =
                                                        rewriter.make(scf::r#if(
                                                            is_size_zero,
                                                            &[rewriter.intrinsics.i64_ty],
                                                            {
                                                                let region = Region::new();
                                                                let block = region
                                                                    .append_block(Block::new(&[]));
                                                                let rewriter =
                                                                    Rewriter::new_with_block(
                                                                        rewriter.context(),
                                                                        block,
                                                                    );
                                                                rewriter.create(scf::r#yield(
                                                                    &[rewriter.make(
                                                                        rewriter.iconst_64(0),
                                                                    )?],
                                                                    location,
                                                                ));
                                                                region
                                                            },
                                                            {
                                                                let region = Region::new();
                                                                let block = region
                                                                    .append_block(Block::new(&[]));
                                                                let rewriter =
                                                                    Rewriter::new_with_block(
                                                                        rewriter.context(),
                                                                        block,
                                                                    );

                                                                let gas_cost = if spec_id
                                                                    .is_enabled_in(SpecId::SHANGHAI)
                                                                {
                                                                    // todo: check size with max_initcode_size

                                                                    let rounded_size =
                                                                        utils::round_up_32(
                                                                            size,
                                                                            rewriter.context(),
                                                                            &rewriter,
                                                                            location,
                                                                        )?;
                                                                    rewriter.make(arith::muli(
                                                                        rounded_size,
                                                                        rewriter.make(
                                                                            rewriter.iconst_64(
                                                                                INIT_WORD_COST,
                                                                            ),
                                                                        )?,
                                                                        location,
                                                                    ))?
                                                                } else {
                                                                    rewriter.make(
                                                                        rewriter.iconst_64(0),
                                                                    )?
                                                                };

                                                                rewriter.create(scf::r#yield(
                                                                    &[gas_cost],
                                                                    location,
                                                                ));
                                                                region
                                                            },
                                                            location,
                                                        ))?;
                                                    Ok(total_gas_cost)
                                                },
                                            )?;
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
                                                    let dest_offset = op.operand(1)?;
                                                    let size = op.operand(2)?;
                                                    let size = rewriter.make(arith::trunci(
                                                        size,
                                                        rewriter.intrinsics.i64_ty,
                                                        location,
                                                    ))?;

                                                    let zero = rewriter
                                                        .make(rewriter.iconst_256_from_u64(0)?)?;
                                                    let is_size_zero =
                                                        rewriter.make(arith::cmpi(
                                                            rewriter.context(),
                                                            CmpiPredicate::Eq,
                                                            size,
                                                            zero,
                                                            location,
                                                        ))?;

                                                    let total_gas_cost =
                                                        rewriter.make(scf::r#if(
                                                            is_size_zero,
                                                            &[rewriter.intrinsics.i64_ty],
                                                            {
                                                                let region = Region::new();
                                                                let block = region
                                                                    .append_block(Block::new(&[]));
                                                                let rewriter =
                                                                    Rewriter::new_with_block(
                                                                        rewriter.context(),
                                                                        block,
                                                                    );
                                                                rewriter.create(scf::r#yield(
                                                                    &[rewriter.make(
                                                                        rewriter.iconst_64(0),
                                                                    )?],
                                                                    location,
                                                                ));
                                                                region
                                                            },
                                                            {
                                                                let region = Region::new();
                                                                let block = region
                                                                    .append_block(Block::new(&[]));
                                                                let rewriter =
                                                                    Rewriter::new_with_block(
                                                                        rewriter.context(),
                                                                        block,
                                                                    );

                                                                    let dest_offset =
                                                                    rewriter.make(arith::trunci(
                                                                        dest_offset,
                                                                        rewriter.intrinsics.i64_ty,
                                                                        location,
                                                                    ))?;

                                                                let required_size = rewriter.make(
                                                                    arith::addi(dest_offset, size, location),
                                                                )?;
                                                                let gas_cost =
                                                                    memory::resize_memory_with_gas_cost(
                                                                        &rewriter,
                                                                        required_size,
                                                                        gas_cost::CREATE,
                                                                    )?;

                                                                rewriter.create(scf::r#yield(
                                                                    &[gas_cost],
                                                                    location,
                                                                ));
                                                                region
                                                            },
                                                            location,
                                                        ))?;
                                                    Ok(total_gas_cost)
                                                },
                                            )?;
                                            if spec_id.is_enabled_in(SpecId::TANGERINE) {
                                                self.insert_dynamic_gas_check_block_before_op_block(
                                                    op,
                                                    block,
                                                    revert_block,
                                                    |rewriter| {
                                                        let location = rewriter.get_insert_location();
                                                        let remaining_gas =
                                                            self::get_gas_counter(rewriter)?;
                                                        rewriter.make(arith::divui(
                                                            remaining_gas,
                                                            rewriter
                                                                .make(rewriter.iconst_64(64))?,
                                                            location,
                                                        ))
                                                    },
                                                )?;
                                            }
                                            // TODO: calculate dynamic gas cost from the system call
                                        }
                                        dora_ir::Operation::Create2 => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
                                                    let dest_offset = op.operand(1)?;
                                                    let dest_offset =
                                                        rewriter.make(arith::trunci(
                                                            dest_offset,
                                                            rewriter.intrinsics.i64_ty,
                                                            location,
                                                        ))?;
                                                    let size = op.operand(2)?;
                                                    let size = rewriter.make(arith::trunci(
                                                        size,
                                                        rewriter.intrinsics.i64_ty,
                                                        location,
                                                    ))?;
                                                    let required_size = rewriter.make(
                                                        arith::addi(dest_offset, size, location),
                                                    )?;
                                                    let total_gas_cost =
                                                        memory::resize_memory_with_gas_cost(
                                                            rewriter,
                                                            required_size,
                                                            gas_cost::CREATE2,
                                                        )?;
                                                    Ok(total_gas_cost)
                                                },
                                            )?;
                                            // TODO: calculate dynamic gas cost from the system call
                                        }
                                        dora_ir::Operation::Call
                                        | dora_ir::Operation::DelegateCall
                                        | dora_ir::Operation::StaticCall
                                        | dora_ir::Operation::CallCode => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
                                                    let args_offset = op.operand(1)?;
                                                    let args_offset =
                                                        rewriter.make(arith::trunci(
                                                            args_offset,
                                                            rewriter.intrinsics.i64_ty,
                                                            location,
                                                        ))?;
                                                    let args_size = op.operand(2)?;
                                                    let args_size =
                                                        rewriter.make(arith::trunci(
                                                            args_size,
                                                            rewriter.intrinsics.i64_ty,
                                                            location,
                                                        ))?;

                                                    let ret_offset = op.operand(3)?;
                                                    let ret_offset =
                                                        rewriter.make(arith::trunci(
                                                            ret_offset,
                                                            rewriter.intrinsics.i64_ty,
                                                            location,
                                                        ))?;
                                                    let ret_size = op.operand(4)?;
                                                    let ret_size = rewriter.make(arith::trunci(
                                                        ret_size,
                                                        rewriter.intrinsics.i64_ty,
                                                        location,
                                                    ))?;
                                                    let req_arg_mem_size =
                                                        rewriter.make(arith::addi(
                                                            args_offset,
                                                            args_size,
                                                            location,
                                                        ))?;
                                                    let req_ret_mem_size = rewriter.make(
                                                        arith::addi(ret_offset, ret_size, location),
                                                    )?;
                                                    let required_size =
                                                        rewriter.make(arith::maxui(
                                                            req_arg_mem_size,
                                                            req_ret_mem_size,
                                                            location,
                                                        ))?;
                                                    let total_gas_cost =
                                                        memory::resize_memory_with_gas_cost(
                                                            rewriter,
                                                            required_size,
                                                            gas_cost::CALL,
                                                        )?;
                                                    Ok(total_gas_cost)
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::Return => {
                                            // No gas cost
                                        }
                                        dora_ir::Operation::Revert => {
                                            // No gas cost
                                        }
                                        dora_ir::Operation::Invalid => {
                                            // No gas cost
                                        }
                                        dora_ir::Operation::Stop => {
                                            // No gas cost
                                        }
                                        dora_ir::Operation::SelfDestruct => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                block,
                                                revert_block,
                                                |_rewriter| {
                                                    let dynamic_gas_cost = op.result(0)?.into();
                                                    Ok(dynamic_gas_cost)
                                                },
                                            )?;
                                        }
                                        _ => todo!(),
                                    }
                                } else {
                                    // Static gas computation.
                                    let gas_cost = get_static_cost_from_op(&name);
                                    debug_assert!(gas_cost > 0);
                                    self.insert_gas_check_block_before_op_block(
                                        op,
                                        block,
                                        revert_block,
                                        gas_cost as i64,
                                    )?;
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Inserts a gas check block before the given operation block.
    ///
    /// This function inserts a static gas check before an operation (`op`) in the specified block
    /// (`block`). If the gas cost is not sufficient, it branches to a revert block (`revert_block`).
    /// The gas cost is provided as a constant (`gas_cost`).
    ///
    /// # Parameters
    ///
    /// - `op`: The operation before which the gas check block is inserted.
    /// - `block`: The block where the gas check is inserted.
    /// - `revert_block`: The block to jump to if the gas check fails (insufficient gas).
    /// - `gas_cost`: The constant gas cost to be deducted from the gas counter.
    ///
    /// # Returns
    ///
    /// - `Result<()>`: Returns `Ok(())` on success or an error if something goes wrong.
    ///
    /// # Example
    ///
    /// ```no_check
    /// gaspass.insert_gas_check_block_before_op_block(&operation_ref, block_ref, revert_block_ref, 1000)?;
    /// ```
    pub fn insert_gas_check_block_before_op_block(
        &mut self,
        op: &OperationRef<'_, '_>,
        block: BlockRef<'_, '_>,
        revert_block: BlockRef<'_, '_>,
        gas_cost: i64,
    ) -> Result<()> {
        self.insert_dynamic_gas_check_block_before_op_block(op, block, revert_block, |rewriter| {
            rewriter.make(rewriter.iconst_64(gas_cost))
        })
    }

    /// Inserts a dynamic gas check block before the given operation block.
    ///
    /// This function inserts a dynamic gas check block before an operation (`op`) in the specified
    /// block (`block`). It allows dynamically calculating the gas cost using a provided closure
    /// (`gas_cost`). If the gas counter is insufficient, it branches to a revert block (`revert_block`).
    ///
    /// # Parameters
    ///
    /// - `op`: The operation before which the dynamic gas check block is inserted.
    /// - `block`: The block where the dynamic gas check is inserted.
    /// - `revert_block`: The block to jump to if the gas check fails (insufficient gas).
    /// - `gas_cost`: A closure that dynamically computes the gas cost.
    ///
    /// # Returns
    ///
    /// - `Result<()>`: Returns `Ok(())` on success or an error if something goes wrong.
    ///
    /// # Example
    ///
    /// ```no_check
    /// gaspass.insert_dynamic_gas_check_block_before_op_block(&operation_ref, block_ref, revert_block_ref, |rewriter| {
    ///     rewriter.make(rewriter.iconst_64(1000))
    /// })?;
    /// ```
    pub(crate) fn insert_dynamic_gas_check_block_before_op_block(
        &mut self,
        op: &OperationRef<'_, '_>,
        block: BlockRef<'_, '_>,
        revert_block: BlockRef<'_, '_>,
        gas_cost: impl for<'a> FnOnce(&'a Rewriter<'a, 'a>) -> Result<Value<'a, 'a>>,
    ) -> Result<()> {
        // To ensure the op rewriter live long as the context.
        let rewriter = Rewriter::new_with_block(self.ctx, block);
        let new_block = rewriter.split_block(block, Some(*op))?;
        // Get address of gas counter global
        let gas_counter_ptr =
            rewriter.make(rewriter.addressof(constants::GAS_COUNTER_GLOBAL, rewriter.ptr_ty()))?;
        let gas_counter =
            rewriter.make(rewriter.load(gas_counter_ptr, rewriter.intrinsics.i64_ty))?;
        let gas_value = gas_cost(&rewriter)?;
        let flag = rewriter.make(arith::cmpi(
            rewriter.context(),
            arith::CmpiPredicate::Sge,
            gas_counter,
            gas_value,
            rewriter.get_insert_location(),
        ))?;
        let new_gas_counter = rewriter.make(arith::subi(
            gas_counter,
            gas_value,
            rewriter.get_insert_location(),
        ))?;
        rewriter.create(llvm::store(
            rewriter.context(),
            new_gas_counter,
            gas_counter_ptr,
            rewriter.get_insert_location(),
            LoadStoreOptions::default(),
        ));
        rewriter.create(cf::cond_br(
            rewriter.context(),
            flag,
            &new_block,
            &revert_block,
            &[],
            &[rewriter.make(rewriter.iconst_8(ExitStatusCode::OutOfGas.to_u8() as i8))?],
            rewriter.get_insert_location(),
        ));
        Ok(())
    }
}

pub mod cost {
    use dora_runtime::constants::gas_cost;

    pub fn get_static_cost_from_op(name: &str) -> u64 {
        let cost = match name {
            "dora.add" => gas_cost::ADD,
            "dora.sub" => gas_cost::SUB,
            "dora.mul" => gas_cost::MUL,
            "dora.div" => gas_cost::DIV,
            "dora.sdiv" => gas_cost::SDIV,
            "dora.mod" => gas_cost::MOD,
            "dora.smod" => gas_cost::SMOD,
            "dora.addmod" => gas_cost::ADDMOD,
            "dora.mulmod" => gas_cost::MULMOD,
            "dora.exp" => gas_cost::EXP,
            "dora.signextend" => gas_cost::SIGNEXTEND,
            "dora.lt" => gas_cost::LT,
            "dora.gt" => gas_cost::GT,
            "dora.slt" => gas_cost::SLT,
            "dora.sgt" => gas_cost::SGT,
            "dora.eq" => gas_cost::EQ,
            "dora.iszero" => gas_cost::ISZERO,
            "dora.and" => gas_cost::AND,
            "dora.or" => gas_cost::OR,
            "dora.xor" => gas_cost::XOR,
            "dora.not" => gas_cost::NOT,
            "dora.byte" => gas_cost::BYTE,
            "dora.shl" => gas_cost::SHL,
            "dora.shr" => gas_cost::SHR,
            "dora.sar" => gas_cost::SAR,
            "dora.keccak256" => gas_cost::KECCAK256,
            "dora.address" => gas_cost::ADDRESS,
            "dora.balance" => gas_cost::BALANCE,
            "dora.origin" => gas_cost::ORIGIN,
            "dora.caller" => gas_cost::CALLER,
            "dora.callvalue" => gas_cost::CALLVALUE,
            "dora.calldataload" => gas_cost::CALLDATALOAD,
            "dora.calldatasize" => gas_cost::CALLDATASIZE,
            "dora.calldatacopy" => gas_cost::CALLDATACOPY,
            "dora.codesize" => gas_cost::CODESIZE,
            "dora.codecopy" => gas_cost::CODECOPY,
            "dora.gasprice" => gas_cost::GASPRICE,
            "dora.extcodesize" => gas_cost::EXTCODESIZE_WARM,
            "dora.extcodecopy" => gas_cost::EXTCODECOPY_WARM,
            "dora.returndatasize" => gas_cost::RETURNDATASIZE,
            "dora.returndatacopy" => gas_cost::RETURNDATACOPY,
            "dora.extcodehash" => gas_cost::EXTCODEHASH,
            "dora.blockhash" => gas_cost::BLOCKHASH,
            "dora.coinbase" => gas_cost::COINBASE,
            "dora.timestamp" => gas_cost::TIMESTAMP,
            "dora.number" => gas_cost::NUMBER,
            "dora.prevrandao" => gas_cost::PREVRANDAO,
            "dora.gaslimit" => gas_cost::GASLIMIT,
            "dora.chainid" => gas_cost::CHAINID,
            "dora.selfbalance" => gas_cost::SELFBALANCE,
            "dora.basefee" => gas_cost::BASEFEE,
            "dora.blobhash" => gas_cost::BLOBHASH,
            "dora.blobbasefee" => gas_cost::BLOBBASEFEE,
            "dora.mload" => gas_cost::MLOAD,
            "dora.mstore" => gas_cost::MSTORE,
            "dora.mstore8" => gas_cost::MSTORE8,
            "dora.sload" => gas_cost::SLOAD,
            "dora.sstore" => gas_cost::SSTORE,
            "dora.msize" => gas_cost::MSIZE,
            "dora.gas" => gas_cost::GAS,
            "dora.tload" => gas_cost::TLOAD,
            "dora.tstore" => gas_cost::TSTORE,
            "dora.mcopy" => gas_cost::MCOPY,
            "dora.log0" => gas_cost::LOG0,
            "dora.log1" => gas_cost::LOG1,
            "dora.log2" => gas_cost::LOG2,
            "dora.log3" => gas_cost::LOG3,
            "dora.log4" => gas_cost::LOG4,
            "dora.create" => gas_cost::CREATE,
            "dora.create2" => gas_cost::CREATE2,
            "dora.call" => gas_cost::CALL,
            "dora.return" => gas_cost::RETURN,
            "dora.staticcall" => gas_cost::STATICCALL,
            "dora.revert" => gas_cost::REVERT,
            "dora.selfdestruct" => gas_cost::SELFDESTRUCT,
            _ => 0,
        };

        cost as u64
    }
}

pub(crate) fn get_gas_counter<'c>(rewriter: &'c Rewriter) -> Result<Value<'c, 'c>> {
    let gas_counter = load_by_addr!(
        rewriter,
        constants::GAS_COUNTER_GLOBAL,
        rewriter.intrinsics.i64_ty
    );
    Ok(gas_counter)
}

pub(crate) fn create_var_with_gas_counter<'c>(
    context: &'c Context,
    rewriter: &'c Rewriter,
    location: Location<'c>,
) -> Result<Value<'c, 'c>> {
    let gas_counter = get_gas_counter(rewriter)?;
    memory::allocate_u64_and_assign_value(context, rewriter, gas_counter, location)
}

pub(crate) fn create_gas_var<'c>(
    context: &'c Context,
    rewriter: &'c Rewriter,
    location: Location<'c>,
) -> Result<Value<'c, 'c>> {
    let gas_ptr = create_var!(rewriter, context, rewriter.intrinsics.i64_ty, location);
    Ok(gas_ptr)
}

// This function computes memory copying cost (excluding expansion), which is given by the following equations
// memory_size_word = (memory_byte_size + 31) / 32
// memory_cost = 3 * memory_size_word
pub(crate) fn compute_copy_cost<'c>(
    rewriter: &'c Rewriter,
    memory_byte_size: Value<'c, 'c>,
) -> Result<Value<'c, 'c>> {
    let location = rewriter.get_insert_location();
    let uint64 = rewriter.intrinsics.i64_ty;

    let memory_size_extended = rewriter.make(arith::extui(memory_byte_size, uint64, location))?;
    let constant_3 = rewriter.make(rewriter.iconst_64(3))?;
    let constant_31 = rewriter.make(rewriter.iconst_64(31))?;
    let constant_32 = rewriter.make(rewriter.iconst_64(32))?;
    let memory_byte_size_plus_31 =
        rewriter.make(arith::addi(memory_size_extended, constant_31, location))?;

    let memory_size_word = rewriter.make(arith::divui(
        memory_byte_size_plus_31,
        constant_32,
        location,
    ))?;

    let memory_cost = rewriter.make(arith::muli(memory_size_word, constant_3, location))?;

    Ok(memory_cost)
}

// computes dynamic_gas = 375 * topic_count + 8 * size
pub(crate) fn compute_log_dynamic_gas<'c>(
    rewriter: &'c Rewriter,
    nth: u8,
    size: Value<'c, 'c>,
) -> Result<Value<'c, 'c>> {
    let location = rewriter.get_insert_location();
    let uint64 = rewriter.intrinsics.i64_ty;
    let constant_375 = rewriter.make(rewriter.iconst_256_from_u64(375)?)?;
    let constant_8 = rewriter.make(rewriter.iconst_256_from_u64(8)?)?;
    let topic_count = rewriter.make(rewriter.iconst_256_from_u64(nth as u64)?)?;
    let topic_count_x_375 = rewriter.make(arith::muli(topic_count, constant_375, location))?;
    let size_x_8 = rewriter.make(arith::muli(size, constant_8, location))?;
    let dynamic_gas = rewriter.make(arith::addi(topic_count_x_375, size_x_8, location))?;
    let dynamic_gas = rewriter.make(arith::trunci(dynamic_gas, uint64, location))?;
    Ok(dynamic_gas)
}
