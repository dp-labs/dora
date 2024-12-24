use crate::{conversion::rewriter::Rewriter, errors::Result};
use dora_primitives::spec::SpecId;
use dora_runtime::constants::gas_cost::{
    self, COPY_WORD_COST, INITCODE_WORD_COST, KECCAK256_WORD_COST,
};
use melior::{
    dialect::{
        arith::{self, CmpiPredicate},
        llvm, scf,
    },
    ir::{Block, Location, Region, Value},
};

/// Calculates the cost of the `EXP` opcode.
pub(crate) fn compute_exp_cost<'c>(
    rewriter: &'c Rewriter,
    exponent: Value<'c, 'c>, /*i256*/
    spec_id: &SpecId,
) -> Result<Value<'c, 'c>> {
    let location = rewriter.get_insert_location();
    let zero = rewriter.make(rewriter.iconst_256_from_u64(0)?)?;
    let is_exponent_zero = rewriter.make(arith::cmpi(
        rewriter.context(),
        CmpiPredicate::Eq,
        exponent,
        zero,
        location,
    ))?;

    rewriter.make(scf::r#if(
        is_exponent_zero,
        &[rewriter.intrinsics.i64_ty],
        {
            let region = Region::new();
            let block = region.append_block(Block::new(&[]));
            let rewriter = Rewriter::new_with_block(rewriter.context(), block);
            rewriter.create(scf::r#yield(
                &[rewriter.make(rewriter.iconst_64(0))?],
                location,
            ));
            region
        },
        {
            let region = Region::new();
            let block = region.append_block(Block::new(&[]));
            let rewriter = Rewriter::new_with_block(rewriter.context(), block);
            let leading_zeros = rewriter.make(llvm::intr_ctlz(
                rewriter.context(),
                exponent,
                false,
                rewriter.intrinsics.i256_ty,
                location,
            ))?;
            let number_of_bits = rewriter.make(arith::subi(
                rewriter.make(rewriter.iconst_256_from_u64(256)?)?,
                leading_zeros,
                location,
            ))?;
            let bits_with_offset = rewriter.make(arith::addi(
                number_of_bits,
                rewriter.make(rewriter.iconst_256_from_u64(7)?)?,
                location,
            ))?;
            let number_of_bytes = rewriter.make(arith::divui(
                bits_with_offset,
                rewriter.make(rewriter.iconst_256_from_u64(8)?)?,
                location,
            ))?;
            let total_gas_cost = rewriter.make(arith::muli(
                number_of_bytes,
                rewriter.make(rewriter.iconst_256_from_u64(
                    if spec_id.is_enabled_in(SpecId::SPURIOUS_DRAGON) {
                        50
                    } else {
                        10
                    },
                )?)?,
                location,
            ))?;

            let total_gas_cost = rewriter.make(arith::trunci(
                total_gas_cost,
                rewriter.intrinsics.i64_ty,
                location,
            ))?;
            rewriter.create(scf::r#yield(&[total_gas_cost], location));
            region
        },
        location,
    ))
}

/// Returns number of words what would fit to provided number of bytes,
/// i.e. it rounds up the number bytes to number of words `(len + 31) / 32`.
pub(crate) fn num_words<'c>(
    rewriter: &'c Rewriter,
    len: Value<'c, 'c>,
    location: Location<'c>,
) -> Result<Value<'c, 'c>> {
    let constant_31 = rewriter.make(rewriter.iconst_64(31))?;
    let constant_32 = rewriter.make(rewriter.iconst_64(32))?;
    let memory_byte_size_plus_31 = rewriter.make(arith::addi(len, constant_31, location))?;
    let memory_size_word = rewriter.make(arith::divui(
        memory_byte_size_plus_31,
        constant_32,
        location,
    ))?;
    Ok(memory_size_word)
}

/// Calculates the cost of buffer per word.
///
/// ```no_check
/// num_words   = (memory_byte_size + 31) / 32
/// cost        = num_words * multiple
/// ```
pub(crate) fn compute_per_word_cost<'c>(
    rewriter: &'c Rewriter,
    len: Value<'c, 'c>, /*i64*/
    multiple: u64,
) -> Result<Value<'c, 'c>> {
    let location = rewriter.get_insert_location();
    rewriter.make(arith::muli(
        num_words(rewriter, len, location)?,
        rewriter.make(rewriter.iconst_64(multiple as i64))?,
        location,
    ))
}

/// Computes copying cost (excluding expansion), which is given by the following equations:
///
/// ```no_check
/// memory_size_word    = (memory_byte_size + 31) / 32
/// memory_cost         = 3 * memory_size_word
/// ```
#[inline]
pub(crate) fn compute_copy_cost<'c>(
    rewriter: &'c Rewriter,
    memory_byte_size: Value<'c, 'c>,
) -> Result<Value<'c, 'c>> {
    compute_per_word_cost(rewriter, memory_byte_size, COPY_WORD_COST)
}

/// Computes keccak256 cost, which is given by the following equations:
///
/// ```no_check
/// memory_size_word    = (memory_byte_size + 31) / 32
/// memory_cost         = 6 * memory_size_word
/// ```
#[inline]
pub(crate) fn compute_keccak256_cost<'c>(
    rewriter: &'c Rewriter,
    memory_byte_size: Value<'c, 'c>,
) -> Result<Value<'c, 'c>> {
    compute_per_word_cost(rewriter, memory_byte_size, KECCAK256_WORD_COST)
}

/// Computes init code cost, which is given by the following equations:
///
/// ```no_check
/// memory_size_word    = (memory_byte_size + 31) / 32
/// memory_cost         = 2 * memory_size_word
/// ```
#[inline]
pub(crate) fn compute_initcode_cost<'c>(
    rewriter: &'c Rewriter,
    memory_byte_size: Value<'c, 'c>,
) -> Result<Value<'c, 'c>> {
    compute_per_word_cost(rewriter, memory_byte_size, INITCODE_WORD_COST)
}

/// Computes eofcreate/create2 cost, which is given by the following equations:
///
/// ```no_check
/// size_word   = (len + 31) / 32
/// memory_cost = 6 * size_word
/// cost        = len * memory_cost
/// ```
#[inline]
pub(crate) fn compute_eofcreate_create2_cost<'c>(
    rewriter: &'c Rewriter,
    len: Value<'c, 'c>,
) -> Result<Value<'c, 'c>> {
    rewriter.make(arith::addi(
        rewriter.make(rewriter.iconst_64(gas_cost::CREATE))?,
        compute_keccak256_cost(rewriter, len)?,
        rewriter.get_insert_location(),
    ))
}

/// Computes LOG opcode cost, which is given by the following equations:
///
/// Computes `dynamic_gas = 8 * size`.
///
/// Note: `375 * topic_count` is the static gas.
pub(crate) fn compute_log_dynamic_cost<'c>(
    rewriter: &'c Rewriter,
    size: Value<'c, 'c>,
) -> Result<Value<'c, 'c>> {
    let location = rewriter.get_insert_location();
    let constant_8 = rewriter.make(rewriter.iconst_64(8))?;
    let size_x_8 = rewriter.make(arith::muli(size, constant_8, location))?;
    Ok(size_x_8)
}

/// Computes memory gas cost, which is given by the following equations:
///
/// `memory_cost = (memory_size_word ** 2) / 512 + (3 * memory_size_word)`
pub(crate) fn memory_gas_cost<'c>(
    rewriter: &'c Rewriter,
    memory_size_word: Value<'c, 'c>,
) -> Result<Value<'c, 'c>> {
    let location = rewriter.get_insert_location();
    // Predefined constants
    let constant_3 = rewriter.make(rewriter.iconst_64(3))?;
    let constant_512 = rewriter.make(rewriter.iconst_64(512))?;

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
