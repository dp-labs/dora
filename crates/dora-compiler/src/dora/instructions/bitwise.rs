use crate::{
    arith_constant, conversion::rewriter::Rewriter, dora::conversion::ConversionPass,
    errors::Result, operands, rewrite_ctx,
};
use melior::{
    dialect::{
        arith::{self, CmpiPredicate},
        scf,
    },
    ir::{attribute::IntegerAttribute, operation::OperationRef, Block, Region},
    Context,
};
use num_bigint::BigUint;

impl<'c> ConversionPass<'c> {
    pub(crate) fn lt(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);
        let lt = rewriter.make(arith::cmpi(
            context,
            arith::CmpiPredicate::Ult,
            l,
            r,
            location,
        ))?;
        rewriter.make(arith::extui(lt, rewriter.intrinsics.i256_ty, location))?;
        Ok(())
    }

    pub(crate) fn gt(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);
        let gt = rewriter.make(arith::cmpi(
            context,
            arith::CmpiPredicate::Ugt,
            l,
            r,
            location,
        ))?;
        rewriter.make(arith::extui(gt, rewriter.intrinsics.i256_ty, location))?;
        Ok(())
    }

    pub(crate) fn slt(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);
        let slt = rewriter.make(arith::cmpi(
            context,
            arith::CmpiPredicate::Slt,
            l,
            r,
            location,
        ))?;
        rewriter.make(arith::extui(slt, rewriter.intrinsics.i256_ty, location))?;
        Ok(())
    }

    pub(crate) fn sgt(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);
        let sgt = rewriter.make(arith::cmpi(
            context,
            arith::CmpiPredicate::Sgt,
            l,
            r,
            location,
        ))?;
        rewriter.make(arith::extui(sgt, rewriter.intrinsics.i256_ty, location))?;
        Ok(())
    }

    pub(crate) fn eq(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);
        let eq = rewriter.make(arith::cmpi(
            context,
            arith::CmpiPredicate::Eq,
            l,
            r,
            location,
        ))?;
        rewriter.make(arith::extui(eq, rewriter.intrinsics.i256_ty, location))?;
        Ok(())
    }

    pub(crate) fn iszero(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, value);
        rewrite_ctx!(context, op, rewriter, location);
        let zero = rewriter.make(arith_constant!(
            rewriter,
            context,
            rewriter.intrinsics.i256_ty,
            0,
            location
        ))?;
        let is_zero = rewriter.make(arith::cmpi(
            context,
            arith::CmpiPredicate::Eq,
            value,
            zero,
            location,
        ))?;
        rewriter.make(arith::extui(is_zero, rewriter.intrinsics.i256_ty, location))?;
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
        rewriter.make(arith::xori(
            value,
            rewriter.make(rewriter.iconst_256(BigUint::from_bytes_be(&[0xff; 32]))?)?,
            location,
        ))?;
        Ok(())
    }

    pub(crate) fn byte(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        const BITS_PER_BYTE: u8 = 8;
        const MAX_SHIFT: u8 = 31;
        operands!(op, offset, shift);
        rewrite_ctx!(context, op, rewriter, location);

        let constant_bits_per_byte = rewriter.make(arith_constant!(
            rewriter,
            context,
            rewriter.intrinsics.i256_ty,
            BITS_PER_BYTE as i64,
            location
        ))?;
        let constant_max_shift_in_bits = rewriter.make(arith_constant!(
            rewriter,
            context,
            rewriter.intrinsics.i256_ty,
            (MAX_SHIFT * BITS_PER_BYTE) as i64,
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
            &[rewriter.intrinsics.i256_ty],
            {
                let region = Region::new();
                let block = region.append_block(Block::new(&[]));
                let rewriter = Rewriter::new_with_block(context, block);
                let zero = rewriter.make(arith_constant!(
                    rewriter,
                    context,
                    rewriter.intrinsics.i256_ty,
                    0,
                    location
                ))?;
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
                let mask = rewriter.make(arith_constant!(
                    rewriter,
                    context,
                    rewriter.intrinsics.i256_ty,
                    0xff,
                    location
                ))?;

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

        // Define the constant value 255
        let value_255 = rewriter.make(arith_constant!(
            rewriter,
            context,
            rewriter.intrinsics.i256_ty,
            255_i64,
            location
        ))?;

        // Compare if the shift amount (operand 0) is less than 255
        let flag = rewriter.make(arith::cmpi(
            context,
            CmpiPredicate::Ule,
            shift,
            value_255,
            location,
        ))?;

        rewriter.make(scf::r#if(
            flag,
            &[rewriter.intrinsics.i256_ty],
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
                let result = rewriter.make(arith_constant!(
                    rewriter,
                    context,
                    rewriter.intrinsics.i256_ty,
                    0_i64,
                    location
                ))?;
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

        // Define the constant value 255
        let value_255 = rewriter.make(arith_constant!(
            rewriter,
            context,
            rewriter.intrinsics.i256_ty,
            255_i64,
            location
        ))?;
        // Compare if the shift amount (operand 0) is less than 255
        let flag = rewriter.make(arith::cmpi(
            context,
            CmpiPredicate::Ule,
            shift,
            value_255,
            location,
        ))?;
        rewriter.make(scf::r#if(
            flag,
            &[rewriter.intrinsics.i256_ty],
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
                let result = rewriter.make(arith_constant!(
                    rewriter,
                    context,
                    rewriter.intrinsics.i256_ty,
                    0_i64,
                    location
                ))?;
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

        // Define the constant value 255
        let value_255 = rewriter.make(arith_constant!(
            rewriter,
            context,
            rewriter.intrinsics.i256_ty,
            255_i64,
            location
        ))?;

        // Ensure the shift amount is capped at 255 to avoid poisoning the result in `shrsi`
        let shift = rewriter.make(arith::minui(o1, value_255, location))?;
        // Perform the arithmetic shift right operation
        rewriter.make(arith::shrsi(o2, shift, location))?;
        Ok(())
    }
}
