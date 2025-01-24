use crate::{
    arith_constant, conversion::rewriter::Rewriter, dora::conversion::ConversionPass,
    errors::Result, operands, rewrite_ctx,
};
use melior::{
    dialect::{
        arith::{self, CmpiPredicate},
        llvm, scf,
    },
    ir::{attribute::StringAttribute, operation::OperationRef, Block, Region, TypeLike, ValueLike},
    Context,
};
use num_bigint::BigUint;

impl ConversionPass<'_> {
    pub(crate) fn lte(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);

        let ty = op.result(0)?.r#type();

        let lt = if l.r#type().is_float() {
            rewriter.make(arith::cmpf(
                context,
                arith::CmpfPredicate::Ole,
                l,
                r,
                location,
            ))?
        } else {
            rewriter.make(arith::cmpi(
                context,
                arith::CmpiPredicate::Ule,
                l,
                r,
                location,
            ))?
        };
        rewriter.make(arith::extui(lt, ty, location))?;
        Ok(())
    }

    pub(crate) fn gte(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);

        let ty = op.result(0)?.r#type();

        let gt = if l.r#type().is_float() {
            rewriter.make(arith::cmpf(
                context,
                arith::CmpfPredicate::Oge,
                l,
                r,
                location,
            ))?
        } else {
            rewriter.make(arith::cmpi(
                context,
                arith::CmpiPredicate::Uge,
                l,
                r,
                location,
            ))?
        };
        rewriter.make(arith::extui(gt, ty, location))?;
        Ok(())
    }

    pub(crate) fn slte(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);

        let ty = op.result(0)?.r#type();

        let slt = if l.r#type().is_float() {
            rewriter.make(arith::cmpf(
                context,
                arith::CmpfPredicate::Ole,
                l,
                r,
                location,
            ))?
        } else {
            rewriter.make(arith::cmpi(
                context,
                arith::CmpiPredicate::Sle,
                l,
                r,
                location,
            ))?
        };
        rewriter.make(arith::extui(slt, ty, location))?;
        Ok(())
    }

    pub(crate) fn sgte(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);

        let ty = op.result(0)?.r#type();

        let sgt = if l.r#type().is_float() {
            rewriter.make(arith::cmpf(
                context,
                arith::CmpfPredicate::Oge,
                l,
                r,
                location,
            ))?
        } else {
            rewriter.make(arith::cmpi(
                context,
                arith::CmpiPredicate::Sge,
                l,
                r,
                location,
            ))?
        };
        rewriter.make(arith::extui(sgt, ty, location))?;
        Ok(())
    }

    pub(crate) fn lt(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);

        let ty = op.result(0)?.r#type();

        let lt = if l.r#type().is_float() {
            rewriter.make(arith::cmpf(
                context,
                arith::CmpfPredicate::Olt,
                l,
                r,
                location,
            ))?
        } else {
            rewriter.make(arith::cmpi(
                context,
                arith::CmpiPredicate::Ult,
                l,
                r,
                location,
            ))?
        };
        rewriter.make(arith::extui(lt, ty, location))?;
        Ok(())
    }

    pub(crate) fn gt(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);

        let ty = op.result(0)?.r#type();

        let gt = if l.r#type().is_float() {
            rewriter.make(arith::cmpf(
                context,
                arith::CmpfPredicate::Ogt,
                l,
                r,
                location,
            ))?
        } else {
            rewriter.make(arith::cmpi(
                context,
                arith::CmpiPredicate::Ugt,
                l,
                r,
                location,
            ))?
        };
        rewriter.make(arith::extui(gt, ty, location))?;
        Ok(())
    }

    pub(crate) fn slt(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);

        let ty = op.result(0)?.r#type();

        let slt = if l.r#type().is_float() {
            rewriter.make(arith::cmpf(
                context,
                arith::CmpfPredicate::Olt,
                l,
                r,
                location,
            ))?
        } else {
            rewriter.make(arith::cmpi(
                context,
                arith::CmpiPredicate::Slt,
                l,
                r,
                location,
            ))?
        };
        rewriter.make(arith::extui(slt, ty, location))?;
        Ok(())
    }

    pub(crate) fn sgt(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);

        let ty = op.result(0)?.r#type();

        let sgt = if l.r#type().is_float() {
            rewriter.make(arith::cmpf(
                context,
                arith::CmpfPredicate::Ogt,
                l,
                r,
                location,
            ))?
        } else {
            rewriter.make(arith::cmpi(
                context,
                arith::CmpiPredicate::Sgt,
                l,
                r,
                location,
            ))?
        };
        rewriter.make(arith::extui(sgt, ty, location))?;
        Ok(())
    }

    pub(crate) fn eq(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);

        let ty = op.result(0)?.r#type();

        let eq = if l.r#type().is_float() {
            rewriter.make(arith::cmpf(
                context,
                arith::CmpfPredicate::Oeq,
                l,
                r,
                location,
            ))?
        } else {
            rewriter.make(arith::cmpi(
                context,
                arith::CmpiPredicate::Eq,
                l,
                r,
                location,
            ))?
        };
        rewriter.make(arith::extui(eq, ty, location))?;
        Ok(())
    }

    pub(crate) fn iszero(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, value);
        rewrite_ctx!(context, op, rewriter, location);

        let value_ty = value.r#type();
        let ret_ty = op.result(0)?.r#type();

        let zero = rewriter.make(rewriter.iconst(value_ty, 0))?;
        let is_zero = if value.r#type().is_float() {
            rewriter.make(arith::cmpf(
                context,
                arith::CmpfPredicate::Oeq,
                value,
                zero,
                location,
            ))?
        } else {
            rewriter.make(arith::cmpi(
                context,
                arith::CmpiPredicate::Eq,
                value,
                zero,
                location,
            ))?
        };
        rewriter.make(arith::extui(is_zero, ret_ty, location))?;
        Ok(())
    }

    pub(crate) fn and(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);
        rewriter.make(arith::andi(l, r, location))?;
        Ok(())
    }

    pub(crate) fn or(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);
        rewriter.make(arith::ori(l, r, location))?;
        Ok(())
    }

    pub(crate) fn xor(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);
        rewriter.make(arith::xori(l, r, location))?;
        Ok(())
    }

    pub(crate) fn not(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, value);
        rewrite_ctx!(context, op, rewriter, location);

        fn get_ones_bigint(int_width: usize) -> BigUint {
            let one = BigUint::from(1_u8);
            let shifted = one.clone() << int_width;
            shifted - one
        }

        let value_ty = value.r#type();
        let int_width = rewriter.int_ty_width(value_ty)?;
        rewriter.make(arith::xori(
            value,
            rewriter
                .make(rewriter.iconst_biguint(value_ty, get_ones_bigint(int_width as usize))?)?,
            location,
        ))?;

        Ok(())
    }

    pub(crate) fn byte(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        const BITS_PER_BYTE: u32 = 8;
        operands!(op, offset, shift);
        rewrite_ctx!(context, op, rewriter, location);

        let ty = offset.r#type();
        let ty_width = rewriter.int_ty_width(ty)?;
        let max_shift = if ty_width >= 8 { ty_width / 8 - 1 } else { 0 };

        let constant_bits_per_byte = rewriter.make(arith_constant!(
            rewriter,
            context,
            ty,
            BITS_PER_BYTE as i64,
            location
        ))?;
        let constant_max_shift_in_bits = rewriter.make(arith_constant!(
            rewriter,
            context,
            ty,
            (max_shift * BITS_PER_BYTE) as i64,
            location
        ))?;

        // Calculate the offset in bits: offset_in_bits = operand_0 * BITS_PER_BYTE
        let offset_in_bits =
            rewriter.make(arith::muli(offset, constant_bits_per_byte, location))?;

        // Compare if offset > max_shift
        let is_offset_oob = rewriter.make(arith::cmpi(
            context,
            arith::CmpiPredicate::Ugt,
            offset_in_bits,
            constant_max_shift_in_bits,
            location,
        ))?;

        rewriter.create(scf::r#if(
            is_offset_oob,
            &[ty],
            {
                let region = Region::new();
                let block = region.append_block(Block::new(&[]));
                let rewriter = Rewriter::new_with_block(context, block);
                let zero = rewriter.make(arith_constant!(rewriter, context, ty, 0, location))?;
                rewriter.create(scf::r#yield(&[zero], location));
                region
            },
            {
                let region = Region::new();
                let block = region.append_block(Block::new(&[]));
                let rewriter = Rewriter::new_with_block(context, block);

                // Calculate the number of bits to shift right: shift_right_in_bits = max_shift - offset
                let shift_right_in_bits = rewriter.make(arith::subi(
                    constant_max_shift_in_bits,
                    offset_in_bits,
                    location,
                ))?;

                // Perform the right shift operation: shifted_right_value = operand_1 >> shift_right_in_bits
                let shifted_value =
                    rewriter.make(arith::shrui(shift, shift_right_in_bits, location))?;

                // Define the mask for isolating the desired byte
                let mask = rewriter.make(arith_constant!(rewriter, context, ty, 0xff, location))?;

                // Apply the bitwise AND operation: result = shifted_right_value & mask
                let result = rewriter.make(arith::andi(shifted_value, mask, location))?;
                rewriter.create(scf::r#yield(&[result], location));
                region
            },
            location,
        ));

        Ok(())
    }

    pub(crate) fn shl(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, shift, value);
        rewrite_ctx!(context, op, rewriter, location);

        let ty = shift.r#type();

        // Define the constant max shift value
        let max_shift =
            rewriter.make(rewriter.iconst(ty, rewriter.int_ty_width(ty)? as i64 - 1))?;

        // Compare if the shift amount (operand 0) is less than 255
        let flag = rewriter.make(arith::cmpi(
            context,
            CmpiPredicate::Ule,
            shift,
            max_shift,
            location,
        ))?;

        rewriter.make(scf::r#if(
            flag,
            &[ty],
            {
                let region = Region::new();
                let block = region.append_block(Block::new(&[]));
                let rewriter = Rewriter::new_with_block(context, block);

                // if shift is less than or equal 255
                let result = rewriter.make(arith::shli(value, shift, location))?;
                rewriter.create(scf::r#yield(&[result], location));
                region
            },
            {
                let region = Region::new();
                let block = region.append_block(Block::new(&[]));
                let rewriter = Rewriter::new_with_block(context, block);

                // if shift is greater than 255
                let result = rewriter.make(rewriter.iconst(ty, 0))?;
                rewriter.create(scf::r#yield(&[result], location));
                region
            },
            location,
        ))?;
        Ok(())
    }

    pub(crate) fn shr(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, shift, value);
        rewrite_ctx!(context, op, rewriter, location);

        let ty = shift.r#type();

        // Define the constant max shift value
        let max_shift =
            rewriter.make(rewriter.iconst(ty, rewriter.int_ty_width(ty)? as i64 - 1))?;
        // Compare if the shift amount (operand 0) is less than 255
        let flag = rewriter.make(arith::cmpi(
            context,
            CmpiPredicate::Ule,
            shift,
            max_shift,
            location,
        ))?;
        rewriter.make(scf::r#if(
            flag,
            &[ty],
            {
                let region = Region::new();
                let block = region.append_block(Block::new(&[]));
                let rewriter = Rewriter::new_with_block(context, block);

                // if shift is less than or equal 255
                let result = rewriter.make(arith::shrui(value, shift, location))?;
                rewriter.create(scf::r#yield(&[result], location));
                region
            },
            {
                let region = Region::new();
                let block = region.append_block(Block::new(&[]));
                let rewriter = Rewriter::new_with_block(context, block);

                // if shift is greater than 255
                let result = rewriter.make(rewriter.iconst(ty, 0))?;
                rewriter.create(scf::r#yield(&[result], location));
                region
            },
            location,
        ))?;
        Ok(())
    }

    pub(crate) fn sar(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, o1, o2);
        rewrite_ctx!(context, op, rewriter, location);

        let ty = o1.r#type();

        // Define the constant max shift value
        let max_shift =
            rewriter.make(rewriter.iconst(ty, rewriter.int_ty_width(ty)? as i64 - 1))?;
        // Ensure the shift amount is capped at the max shift to avoid poisoning the result in `shrsi`
        let shift = rewriter.make(arith::minui(o1, max_shift, location))?;
        // Perform the arithmetic shift right operation
        rewriter.make(arith::shrsi(o2, shift, location))?;
        Ok(())
    }

    pub(crate) fn rotl(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, value, shift);
        rewrite_ctx!(context, op, rewriter, location);

        let ty = value.r#type();
        let bit_width = rewriter.int_ty_width(ty)? as i64;

        // Define constants
        let max_shift = rewriter.make(rewriter.iconst(ty, bit_width))?;
        let shift_mod = rewriter.make(arith::remui(shift, max_shift, location))?;

        // Calculate the rotated left value
        let left_shifted = rewriter.make(arith::shli(value, shift_mod, location))?;
        let right_shift_amount = rewriter.make(arith::subi(max_shift, shift_mod, location))?;
        let right_shifted = rewriter.make(arith::shrui(value, right_shift_amount, location))?;
        rewriter.make(arith::ori(left_shifted, right_shifted, location))?;

        Ok(())
    }

    pub(crate) fn rotr(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, value, shift);
        rewrite_ctx!(context, op, rewriter, location);

        let ty = value.r#type();
        let bit_width = rewriter.int_ty_width(ty)? as i64;

        // Define constants
        let max_shift = rewriter.make(rewriter.iconst(ty, bit_width))?;
        let shift_mod = rewriter.make(arith::remui(shift, max_shift, location))?;

        // Calculate the rotated right value
        let right_shifted = rewriter.make(arith::shrui(value, shift_mod, location))?;
        let left_shift_amount = rewriter.make(arith::subi(max_shift, shift_mod, location))?;
        let left_shifted = rewriter.make(arith::shli(value, left_shift_amount, location))?;
        rewriter.make(arith::ori(right_shifted, left_shifted, location))?;

        Ok(())
    }

    pub(crate) fn clz(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        Self::call_llvm_intrinsic(context, op, "llvm.ctlz")
    }

    pub(crate) fn ctz(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        Self::call_llvm_intrinsic(context, op, "llvm.cttz")
    }

    pub(crate) fn popcnt(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        Self::call_llvm_intrinsic(context, op, "llvm.ctpop")
    }

    fn call_llvm_intrinsic(
        context: &Context,
        op: &OperationRef<'_, '_>,
        intrinsic_base: &str,
    ) -> Result<()> {
        operands!(op, value);
        rewrite_ctx!(context, op, rewriter, location);

        let ty = value.r#type();
        let bit_width = rewriter.int_ty_width(ty)? as i64;
        let is_zero_undef = rewriter.make(rewriter.iconst(rewriter.i1_ty(), 0))?;

        let intrinsic_name = format!("{}.i{}", intrinsic_base, bit_width);
        let intrinsic_attr = StringAttribute::new(context, &intrinsic_name);

        let args = vec![value, is_zero_undef];
        let result_types = vec![ty];

        rewriter.make(llvm::call_intrinsic(
            context,
            intrinsic_attr,
            &args,
            &result_types,
            location,
        ))?;

        Ok(())
    }
}
