use std::mem::offset_of;

use super::memory;
use crate::backend::IntCC;
use crate::conversion::builder::OpBuilder;
use crate::dora::utils;
use crate::{
    arith_constant, check_op_oog,
    conversion::{rewriter::Rewriter, walker::walk_operation},
    create_var,
    errors::Result,
    load_by_addr, maybe_revert_here, u256_to_64,
    value::IntoContextOperation,
};
use dora_primitives::spec::SpecId;
use dora_runtime::symbols::CTX_IS_STATIC;
use dora_runtime::{
    constants::{
        self,
        gas_cost::{self, COPY_WORD_COST, INIT_WORD_COST, KECCAK256_WORD_COST, MAX_INITCODE_SIZE},
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

                                let base_gas = info.base_gas();
                                if base_gas > 0 {
                                    self.insert_gas_check_block_before_op_block(
                                        op,
                                        revert_block,
                                        base_gas as i64,
                                    )?;
                                }

                                if info.is_dynamic() {
                                    match dora_op {
                                        dora_ir::Operation::Balance => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                &op.next_in_block().unwrap(),
                                                revert_block,
                                                |rewriter| {
                                                    let result_ptr = op.result(0)?.into();
                                                    rewriter.get_field_value(
                                                        result_ptr,
                                                        offset_of!(
                                                            dora_runtime::context::Result<()>,
                                                            gas_used
                                                        ),
                                                        rewriter.intrinsics.i64_ty,
                                                    )
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::Exp => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
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
                                                                let total_gas_cost =
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
                                                |rewriter| {
                                                    let size = op.operand(1)?;
                                                    let location = rewriter.get_insert_location();
                                                    let size = rewriter.make(arith::trunci(
                                                        size,
                                                        rewriter.intrinsics.i64_ty,
                                                        location,
                                                    ))?;
                                                    compute_keccak256_cost(rewriter, size)
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::CallDataCopy => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
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
                                                    let total_gas_cost =
                                                        compute_resize_memory_cost(
                                                            op,
                                                            rewriter,
                                                            dest_offset,
                                                            size,
                                                        )?;
                                                    Ok(total_gas_cost)
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::CodeCopy => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
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
                                                    let total_gas_cost =
                                                        compute_resize_memory_cost(
                                                            op,
                                                            rewriter,
                                                            dest_offset,
                                                            size,
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
                                                revert_block,
                                                |rewriter| {
                                                    let result_ptr = op.result(0)?.into();
                                                    rewriter.get_field_value(
                                                        result_ptr,
                                                        offset_of!(
                                                            dora_runtime::context::Result<u64>,
                                                            gas_used
                                                        ),
                                                        rewriter.intrinsics.i64_ty,
                                                    )
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::ExtCodeCopy => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
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
                                                    let total_gas_cost =
                                                        compute_resize_memory_cost(
                                                            op,
                                                            rewriter,
                                                            dest_offset,
                                                            size,
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
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                &op.next_in_block().unwrap(),
                                                revert_block,
                                                |rewriter| {
                                                    let result_ptr = op.result(0)?.into();
                                                    rewriter.get_field_value(
                                                        result_ptr,
                                                        offset_of!(
                                                            dora_runtime::context::Result<()>,
                                                            gas_used
                                                        ),
                                                        rewriter.intrinsics.i64_ty,
                                                    )
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::ReturnDataCopy => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
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
                                        dora_ir::Operation::ExtCodeHash => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                &op.next_in_block().unwrap(),
                                                revert_block,
                                                |rewriter| {
                                                    let result_ptr = op.result(0)?.into();
                                                    rewriter.get_field_value(
                                                        result_ptr,
                                                        offset_of!(
                                                            dora_runtime::context::Result<()>,
                                                            gas_used
                                                        ),
                                                        rewriter.intrinsics.i64_ty,
                                                    )
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::MLoad => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
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
                                                    let total_gas_cost =
                                                        compute_resize_memory_cost(
                                                            op,
                                                            rewriter,
                                                            dest_offset,
                                                            size,
                                                        )?;
                                                    Ok(total_gas_cost)
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::MStore => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
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
                                                    let total_gas_cost =
                                                        compute_resize_memory_cost(
                                                            op,
                                                            rewriter,
                                                            dest_offset,
                                                            size,
                                                        )?;
                                                    Ok(total_gas_cost)
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::MStore8 => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
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
                                                        rewriter.make(rewriter.iconst_64(8))?;
                                                    compute_resize_memory_cost(
                                                        op,
                                                        rewriter,
                                                        dest_offset,
                                                        size,
                                                    )
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::SLoad => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                &op.next_in_block().unwrap(),
                                                revert_block,
                                                |rewriter| {
                                                    let result_ptr = op.result(0)?.into();
                                                    rewriter.get_field_value(
                                                        result_ptr,
                                                        offset_of!(
                                                            dora_runtime::context::Result<()>,
                                                            gas_used
                                                        ),
                                                        rewriter.intrinsics.i64_ty,
                                                    )
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::SStore => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                &op.next_in_block().unwrap(),
                                                revert_block,
                                                |rewriter| {
                                                    let result_ptr = op.result(0)?.into();
                                                    rewriter.get_field_value(
                                                        result_ptr,
                                                        offset_of!(
                                                            dora_runtime::context::Result<()>,
                                                            gas_used
                                                        ),
                                                        rewriter.intrinsics.i64_ty,
                                                    )
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::MCopy => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
                                                    let size = op.operand(2)?;
                                                    let size = rewriter.make(arith::trunci(
                                                        size,
                                                        rewriter.intrinsics.i64_ty,
                                                        location,
                                                    ))?;
                                                    compute_copy_cost(rewriter, size)
                                                },
                                            )?;
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
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
                                                    let offset = rewriter.make(arith::maxui(
                                                        dest_offset,
                                                        offset,
                                                        location,
                                                    ))?;
                                                    compute_resize_memory_cost(
                                                        op, rewriter, offset, size,
                                                    )
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::Log0 => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
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
                                                    let total_gas_cost =
                                                        compute_resize_memory_cost(
                                                            op,
                                                            rewriter,
                                                            dest_offset,
                                                            size,
                                                        )?;
                                                    let dynamic_gas_cost =
                                                        compute_log_dynamic_cost(
                                                            rewriter, 0, size,
                                                        )?;
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
                                                    let total_gas_cost =
                                                        compute_resize_memory_cost(
                                                            op,
                                                            rewriter,
                                                            dest_offset,
                                                            size,
                                                        )?;
                                                    let dynamic_gas_cost =
                                                        compute_log_dynamic_cost(
                                                            rewriter, 1, size,
                                                        )?;
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
                                                    let total_gas_cost =
                                                        compute_resize_memory_cost(
                                                            op,
                                                            rewriter,
                                                            dest_offset,
                                                            size,
                                                        )?;
                                                    let dynamic_gas_cost =
                                                        compute_log_dynamic_cost(
                                                            rewriter, 2, size,
                                                        )?;
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
                                                    let total_gas_cost =
                                                        compute_resize_memory_cost(
                                                            op,
                                                            rewriter,
                                                            dest_offset,
                                                            size,
                                                        )?;
                                                    let dynamic_gas_cost =
                                                        compute_log_dynamic_cost(
                                                            rewriter, 3, size,
                                                        )?;
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
                                                    let total_gas_cost =
                                                        compute_resize_memory_cost(
                                                            op,
                                                            rewriter,
                                                            dest_offset,
                                                            size,
                                                        )?;
                                                    let dynamic_gas_cost =
                                                        compute_log_dynamic_cost(
                                                            rewriter, 4, size,
                                                        )?;
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
                                        dora_ir::Operation::Create
                                        | dora_ir::Operation::Create2 => {
                                            // Ensure non static call before the gas computation.
                                            let rewriter = Rewriter::new_with_op(self.ctx, *op);
                                            let ctx_is_static_ptr = rewriter.make(
                                                rewriter
                                                    .addressof(CTX_IS_STATIC, rewriter.ptr_ty()),
                                            )?;
                                            let ctx_is_static = rewriter.make(rewriter.load(
                                                ctx_is_static_ptr,
                                                rewriter.intrinsics.i1_ty,
                                            ))?;
                                            maybe_revert_here!(
                                                op,
                                                rewriter,
                                                ctx_is_static,
                                                ExitStatusCode::StateChangeDuringStaticCall
                                            );
                                            // Limit is set as double of max contract bytecode size.
                                            let max_initcode_size = self
                                                .options
                                                .limit_contract_code_size
                                                .map(|limit| limit.saturating_mul(2))
                                                .unwrap_or(MAX_INITCODE_SIZE);
                                            let size = op.operand(2)?;
                                            let rewriter = Rewriter::new_with_op(self.ctx, *op);
                                            u256_to_64!(op, rewriter, size);
                                            let max_initcode_size = rewriter.make(
                                                rewriter.iconst_64(max_initcode_size as i64),
                                            )?;
                                            let revert_flag = rewriter.make(arith::cmpi(
                                                self.ctx,
                                                CmpiPredicate::Ugt,
                                                size,
                                                max_initcode_size,
                                                rewriter.get_insert_location(),
                                            ))?;
                                            maybe_revert_here!(
                                                op,
                                                rewriter,
                                                revert_flag,
                                                ExitStatusCode::CreateInitCodeSizeLimit
                                            );
                                            // Check init code gas cost
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
                                                    let size = op.operand(2)?;
                                                    let size = rewriter.make(arith::trunci(
                                                        size,
                                                        rewriter.intrinsics.i64_ty,
                                                        location,
                                                    ))?;

                                                    let zero =
                                                        rewriter.make(rewriter.iconst_64(0))?;
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
                                                                    // Init code gas cost
                                                                    compute_initcode_cost(
                                                                        &rewriter, size,
                                                                    )
                                                                } else {
                                                                    rewriter
                                                                        .make(rewriter.iconst_64(0))
                                                                }?;

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
                                            // Check memory extend gas cost
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
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

                                                    let zero =
                                                        rewriter.make(rewriter.iconst_64(0))?;
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

                                                                let dest_offset = rewriter.make(
                                                                    arith::trunci(
                                                                        dest_offset,
                                                                        rewriter.intrinsics.i64_ty,
                                                                        location,
                                                                    ),
                                                                )?;

                                                                let gas_cost =
                                                                    compute_resize_memory_cost(
                                                                        op,
                                                                        &rewriter,
                                                                        dest_offset,
                                                                        size,
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
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
                                                    let is_create2 = matches!(
                                                        dora_op,
                                                        dora_ir::Operation::Create
                                                    );
                                                    if is_create2 {
                                                        let size = op.operand(2)?;
                                                        let size = rewriter.make(arith::trunci(
                                                            size,
                                                            rewriter.intrinsics.i64_ty,
                                                            location,
                                                        ))?;
                                                        compute_create2_cost(rewriter, size)
                                                    } else {
                                                        rewriter.make(
                                                            rewriter.iconst_64(gas_cost::CREATE),
                                                        )
                                                    }
                                                },
                                            )?;
                                            if spec_id.is_enabled_in(SpecId::TANGERINE) {
                                                self.insert_dynamic_gas_check_block_before_op_block(
                                                    op,
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
                                        dora_ir::Operation::Call
                                        | dora_ir::Operation::DelegateCall
                                        | dora_ir::Operation::StaticCall
                                        | dora_ir::Operation::CallCode => {
                                            if matches!(
                                                dora_op,
                                                dora_ir::Operation::Call
                                                    | dora_ir::Operation::CallCode
                                            ) {
                                                // Static call value is zero check
                                                let rewriter = Rewriter::new_with_op(self.ctx, *op);
                                                let location = rewriter.get_insert_location();
                                                let ctx_is_static_ptr =
                                                    rewriter.make(rewriter.addressof(
                                                        CTX_IS_STATIC,
                                                        rewriter.ptr_ty(),
                                                    ))?;
                                                let ctx_is_static =
                                                    rewriter.make(rewriter.load(
                                                        ctx_is_static_ptr,
                                                        rewriter.intrinsics.i1_ty,
                                                    ))?;
                                                let zero = rewriter
                                                    .make(rewriter.iconst_256_from_u64(0)?)?;
                                                let value = op.operand(2)?;
                                                let value_is_not_zero = rewriter.make(
                                                    rewriter.icmp(IntCC::NotEqual, value, zero),
                                                )?;
                                                let revert_flag = rewriter.make(arith::andi(
                                                    ctx_is_static,
                                                    value_is_not_zero,
                                                    location,
                                                ))?;

                                                maybe_revert_here!(
                                                    op,
                                                    rewriter,
                                                    revert_flag,
                                                    ExitStatusCode::CallNotAllowedInsideStatic
                                                );
                                            }

                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
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
                                                    compute_resize_memory_cost(
                                                        op,
                                                        rewriter,
                                                        args_offset,
                                                        args_size,
                                                    )
                                                },
                                            )?;

                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
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
                                                    compute_resize_memory_cost(
                                                        op, rewriter, ret_offset, ret_size,
                                                    )
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::Return => {
                                            self.insert_dynamic_gas_check_block_before_op_block(
                                                op,
                                                revert_block,
                                                |rewriter| {
                                                    let location = rewriter.get_insert_location();
                                                    let offset = op.operand(0)?;
                                                    let offset = rewriter.make(arith::trunci(
                                                        offset,
                                                        rewriter.intrinsics.i64_ty,
                                                        location,
                                                    ))?;
                                                    let size = op.operand(1)?;
                                                    let size = rewriter.make(arith::trunci(
                                                        size,
                                                        rewriter.intrinsics.i64_ty,
                                                        location,
                                                    ))?;
                                                    compute_resize_memory_cost(
                                                        op, rewriter, offset, size,
                                                    )
                                                },
                                            )?;
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
                                                &op.next_in_block().unwrap(),
                                                revert_block,
                                                |rewriter| {
                                                    let result_ptr = op.result(0)?.into();
                                                    rewriter.get_field_value(
                                                        result_ptr,
                                                        offset_of!(
                                                            dora_runtime::context::Result<u64>,
                                                            gas_used
                                                        ),
                                                        rewriter.intrinsics.i64_ty,
                                                    )
                                                },
                                            )?;
                                        }
                                        dora_ir::Operation::ReturnDataLoad => unimplemented!(
                                            "https://github.com/dp-labs/dora/issues/77"
                                        ),
                                        x => unimplemented!("dynamic gas computation for {:?}", x),
                                    }
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
        revert_block: BlockRef<'_, '_>,
        gas_cost: i64,
    ) -> Result<()> {
        self.insert_dynamic_gas_check_block_before_op_block(op, revert_block, |rewriter| {
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
        revert_block: BlockRef<'_, '_>,
        gas_cost: impl for<'a> FnOnce(&'a Rewriter<'a, 'a>) -> Result<Value<'a, 'a>>,
    ) -> Result<()> {
        if let Some(block) = op.block() {
            if let Some(region) = block.parent_region() {
                let update_gas_remaining_block = region.append_block(Block::new(&[]));
                // To ensure the op rewriter live long as the context.
                let rewriter = Rewriter::new_with_block(self.ctx, block);
                let location = rewriter.get_insert_location();
                let new_block = rewriter.split_block(block, Some(*op))?;
                // Get address of gas counter global
                let gas_counter_ptr = rewriter
                    .make(rewriter.addressof(constants::GAS_COUNTER_GLOBAL, rewriter.ptr_ty()))?;
                let gas_counter =
                    rewriter.make(rewriter.load(gas_counter_ptr, rewriter.intrinsics.i64_ty))?;
                let gas_value = gas_cost(&rewriter)?;
                let flag = rewriter.make(arith::cmpi(
                    rewriter.context(),
                    arith::CmpiPredicate::Uge,
                    gas_counter,
                    gas_value,
                    location,
                ))?;
                rewriter.create(cf::cond_br(
                    rewriter.context(),
                    flag,
                    &update_gas_remaining_block,
                    &revert_block,
                    &[],
                    &[rewriter.make(rewriter.iconst_8(ExitStatusCode::OutOfGas.to_u8() as i8))?],
                    location,
                ));
                let rewriter = Rewriter::new_with_block(self.ctx, update_gas_remaining_block);
                let new_gas_counter =
                    rewriter.make(arith::subi(gas_counter, gas_value, location))?;
                rewriter.create(llvm::store(
                    rewriter.context(),
                    new_gas_counter,
                    gas_counter_ptr,
                    location,
                    LoadStoreOptions::default(),
                ));
                rewriter.create(cf::br(&new_block, &[], location));
            }
        }
        Ok(())
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

/// Calculate the cost of buffer per word.
/// num_words = (memory_byte_size + 31) / 32
/// cost = num_words * multiple
pub(crate) fn compute_per_word_cost<'c>(
    rewriter: &'c Rewriter,
    len: Value<'c, 'c>, /*i64*/
    multiple: u64,
) -> Result<Value<'c, 'c>> {
    let location = rewriter.get_insert_location();
    let constant_multiple = rewriter.make(rewriter.iconst_64(multiple as i64))?;
    let constant_31 = rewriter.make(rewriter.iconst_64(31))?;
    let constant_32 = rewriter.make(rewriter.iconst_64(32))?;
    let memory_byte_size_plus_31 = rewriter.make(arith::addi(len, constant_31, location))?;

    let memory_size_word = rewriter.make(arith::divui(
        memory_byte_size_plus_31,
        constant_32,
        location,
    ))?;

    let memory_cost = rewriter.make(arith::muli(memory_size_word, constant_multiple, location))?;

    Ok(memory_cost)
}

/// This function computes copying cost (excluding expansion), which is given by the following equations
/// memory_size_word = (memory_byte_size + 31) / 32
/// memory_cost = 3 * memory_size_word
#[inline]
pub(crate) fn compute_copy_cost<'c>(
    rewriter: &'c Rewriter,
    memory_byte_size: Value<'c, 'c>,
) -> Result<Value<'c, 'c>> {
    compute_per_word_cost(rewriter, memory_byte_size, COPY_WORD_COST)
}

/// This function computes keccak256 cost, which is given by the following equations
/// memory_size_word = (memory_byte_size + 31) / 32
/// memory_cost = 6 * memory_size_word
#[inline]
pub(crate) fn compute_keccak256_cost<'c>(
    rewriter: &'c Rewriter,
    memory_byte_size: Value<'c, 'c>,
) -> Result<Value<'c, 'c>> {
    compute_per_word_cost(rewriter, memory_byte_size, KECCAK256_WORD_COST)
}

/// This function computes init code cost, which is given by the following equations
/// memory_size_word = (memory_byte_size + 31) / 32
/// memory_cost = 2 * memory_size_word
#[inline]
pub(crate) fn compute_initcode_cost<'c>(
    rewriter: &'c Rewriter,
    memory_byte_size: Value<'c, 'c>,
) -> Result<Value<'c, 'c>> {
    compute_per_word_cost(rewriter, memory_byte_size, INIT_WORD_COST)
}

/// This function computes create2 cost, which is given by the following equations
/// size_word = (len + 31) / 32
/// memory_cost = 6 * size_word
/// cost = len * memory_cost
#[inline]
pub(crate) fn compute_create2_cost<'c>(
    rewriter: &'c Rewriter,
    len: Value<'c, 'c>,
) -> Result<Value<'c, 'c>> {
    rewriter.make(arith::addi(
        rewriter.make(rewriter.iconst_64(gas_cost::CREATE))?,
        compute_keccak256_cost(rewriter, len)?,
        rewriter.get_insert_location(),
    ))
}

/// This function computes LOG opcode cost, which is given by the following equations
/// computes dynamic_gas = 375 * topic_count + 8 * size
pub(crate) fn compute_log_dynamic_cost<'c>(
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

pub(crate) fn compute_resize_memory_cost<'c>(
    _op: &OperationRef<'c, 'c>,
    rewriter: &'c Rewriter,
    offset: Value<'c, 'c>,
    len: Value<'c, 'c>,
) -> Result<Value<'c, 'c>> {
    let context = rewriter.context();
    let location = rewriter.get_insert_location();
    let required_size = rewriter.make(arith::addi(offset, len, location))?;
    // Load memory size
    let memory_size_ptr =
        rewriter.make(rewriter.addressof(constants::MEMORY_SIZE_GLOBAL, rewriter.ptr_ty()))?;
    let memory_size = rewriter.make(llvm::load(
        context,
        memory_size_ptr,
        rewriter.intrinsics.i64_ty,
        location,
        LoadStoreOptions::default(),
    ))?;
    let rounded_required_size = utils::round_up_32(required_size, context, rewriter, location)?;
    let extension_flag = rewriter.make(arith::cmpi(
        context,
        CmpiPredicate::Ult,
        memory_size,
        rounded_required_size,
        location,
    ))?;
    let dynamic_gas_value = rewriter.make(scf::r#if(
        extension_flag,
        &[rewriter.intrinsics.i64_ty],
        {
            let region = Region::new();
            let block = region.append_block(Block::new(&[]));
            let rewriter = Rewriter::new_with_block(context, block);
            // dynamic gas computation in the gas pass
            let memory_cost_before = memory_gas_cost(&rewriter, memory_size)?;
            let memory_cost_after = memory_gas_cost(&rewriter, rounded_required_size)?;
            let dynamic_gas_value =
                rewriter.make(arith::subi(memory_cost_after, memory_cost_before, location))?;
            rewriter.create(scf::r#yield(&[dynamic_gas_value], location));
            region
        },
        {
            let region = Region::new();
            let block = region.append_block(Block::new(&[]));
            let rewriter = Rewriter::new_with_block(context, block);
            rewriter.create(scf::r#yield(
                &[rewriter.make(rewriter.iconst_64(0))?],
                location,
            ));
            region
        },
        location,
    ))?;
    Ok(dynamic_gas_value)
}

// This function computes memory gas cost, which is given by the following equations.
// memory_size_word = (memory_byte_size + 31) / 32
// memory_cost = (memory_size_word ** 2) / 512 + (3 * memory_size_word)
pub(crate) fn memory_gas_cost<'c>(
    rewriter: &'c Rewriter,
    memory_byte_size: Value<'c, 'c>,
) -> Result<Value<'c, 'c>> {
    let location = rewriter.get_insert_location();
    // Helper function to create constants
    let make_constant =
        |val: i64| -> Result<Value<'c, 'c>> { rewriter.make(rewriter.iconst_64(val)) };

    // Predefined constants
    let constant_3 = make_constant(3)?;
    let constant_31 = make_constant(31)?;
    let constant_32 = make_constant(32)?;
    let constant_512 = make_constant(512)?;

    // Memory calculations
    let memory_byte_size_plus_31 =
        rewriter.make(arith::addi(memory_byte_size, constant_31, location))?;
    let memory_size_word = rewriter.make(arith::divui(
        memory_byte_size_plus_31,
        constant_32,
        location,
    ))?;

    // Word-based calculations
    let memory_size_word_squared =
        rewriter.make(arith::muli(memory_size_word, memory_size_word, location))?;
    let memory_size_word_squared_divided_by_512 = rewriter.make(arith::divui(
        memory_size_word_squared,
        constant_512,
        location,
    ))?;
    let memory_size_word_times_3 =
        rewriter.make(arith::muli(memory_size_word, constant_3, location))?;

    // Final memory cost calculation
    let memory_cost = rewriter.make(arith::addi(
        memory_size_word_squared_divided_by_512,
        memory_size_word_times_3,
        location,
    ))?;
    Ok(memory_cost)
}
