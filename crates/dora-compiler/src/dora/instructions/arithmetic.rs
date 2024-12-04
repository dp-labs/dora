use crate::{
    arith_constant,
    backend::IntCC,
    conversion::rewriter::{DeferredRewriter, Rewriter},
    dora::conversion::ConversionPass,
    errors::Result,
    operands, rewrite_ctx,
};
use melior::{
    dialect::{
        arith,
        ods::{llvm, math},
        scf,
    },
    ir::{
        attribute::IntegerAttribute, operation::OperationRef, r#type::IntegerType, Block, Region,
    },
    Context,
};

impl<'c> ConversionPass<'c> {
    pub(crate) fn add(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);
        rewriter.make(arith::addi(l, r, location))?;
        Ok(())
    }

    pub(crate) fn mul(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);
        rewriter.make(arith::muli(l, r, location))?;
        Ok(())
    }

    pub(crate) fn sub(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);
        rewriter.make(arith::subi(l, r, location))?;
        Ok(())
    }

    pub(crate) fn udiv(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);
        let result = rewriter.make(arith::divui(l, r, location))?;
        let zero = rewriter.make(rewriter.iconst_256_from_u64(0)?)?;
        let is_zero = rewriter.make(rewriter.icmp(IntCC::Equal, r, zero))?;
        rewriter.make(arith::select(is_zero, zero, result, location))?;
        Ok(())
    }

    pub(crate) fn sdiv(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);
        let zero = rewriter.make(rewriter.iconst_256_from_u64(0)?)?;
        let is_zero = rewriter.make(rewriter.icmp(IntCC::Equal, r, zero))?;
        rewriter.make(scf::r#if(
            is_zero,
            &[rewriter.intrinsics.i256_ty],
            {
                let region = Region::new();
                let block = region.append_block(Block::new(&[]));
                let rewriter = Rewriter::new_with_block(context, block);
                rewriter.create(scf::r#yield(&[zero], location));
                region
            },
            {
                // Note the sdiv overflow
                let region = Region::new();
                let block = region.append_block(Block::new(&[]));
                let rewriter = Rewriter::new_with_block(context, block);
                // i256::min 0x80000000_00000000_00000000_00000000
                let i256_min = rewriter.make(rewriter.iconst_256_min()?)?;
                let l_is_i256_min = rewriter.make(rewriter.icmp(IntCC::Equal, l, i256_min))?;
                let r_is_neg1 = rewriter.make(rewriter.icmp_imm(IntCC::Equal, r, -1)?)?;
                let is_sdiv_edge_case =
                    rewriter.make(arith::andi(l_is_i256_min, r_is_neg1, location))?;
                let result = rewriter.make(arith::divsi(l, r, location))?;
                rewriter.create(scf::r#yield(
                    &[rewriter.make(arith::select(
                        is_sdiv_edge_case,
                        i256_min,
                        result,
                        location,
                    ))?],
                    location,
                ));
                region
            },
            location,
        ))?;
        Ok(())
    }

    pub(crate) fn umod(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);
        let result = rewriter.make(arith::remui(l, r, location))?;
        let zero = rewriter.make(rewriter.iconst_256_from_u64(0)?)?;
        let is_zero = rewriter.make(rewriter.icmp(IntCC::Equal, r, zero))?;
        rewriter.make(arith::select(is_zero, zero, result, location))?;
        Ok(())
    }

    pub(crate) fn smod(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);
        let result = rewriter.make(arith::remsi(l, r, location))?;
        let zero = rewriter.make(rewriter.iconst_256_from_u64(0)?)?;
        let is_zero = rewriter.make(rewriter.icmp(IntCC::Equal, r, zero))?;
        rewriter.make(arith::select(is_zero, zero, result, location))?;
        Ok(())
    }

    pub(crate) fn addmod(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, a, b, den);
        rewrite_ctx!(context, op, rewriter, location);

        let a = rewriter.make(arith::extui(a, rewriter.intrinsics.i257_ty, location))?;
        let b = rewriter.make(arith::extui(b, rewriter.intrinsics.i257_ty, location))?;
        let den = rewriter.make(arith::extui(den, rewriter.intrinsics.i257_ty, location))?;
        let add = rewriter.make(arith::addi(a, b, location))?;
        let umod = rewriter.make(arith::remui(add, den, location))?;

        rewriter.make(arith::trunci(umod, rewriter.intrinsics.i256_ty, location))?;
        Ok(())
    }

    pub(crate) fn mulmod(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, a, b, den);
        rewrite_ctx!(context, op, rewriter, location);

        let uint512 = IntegerType::new(context, 512);
        let a = rewriter.make(arith::extui(a, uint512.into(), location))?;
        let b = rewriter.make(arith::extui(b, uint512.into(), location))?;
        let den = rewriter.make(arith::extui(den, uint512.into(), location))?;
        let add = rewriter.make(arith::muli(a, b, location))?;
        let umod = rewriter.make(arith::remui(add, den, location))?;

        rewriter.make(arith::trunci(umod, rewriter.intrinsics.i256_ty, location))?;
        Ok(())
    }

    pub(crate) fn exp(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);
        rewriter.make(math::ipowi(context, l, r, location).into())?;
        Ok(())
    }

    pub(crate) fn signextend(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, byte_size, value_to_extend);
        rewrite_ctx!(context, op, rewriter, location);

        // Constant Definitions
        let max_byte_size = rewriter.make(arith_constant!(
            rewriter,
            context,
            rewriter.intrinsics.i256_ty,
            31,
            location
        ))?;
        let bits_per_byte = rewriter.make(arith_constant!(
            rewriter,
            context,
            rewriter.intrinsics.i256_ty,
            8,
            location
        ))?;
        let sign_bit_position_on_byte = rewriter.make(arith_constant!(
            rewriter,
            context,
            rewriter.intrinsics.i256_ty,
            7,
            location
        ))?;
        let max_bits = rewriter.make(arith_constant!(
            rewriter,
            context,
            rewriter.intrinsics.i256_ty,
            255,
            location
        ))?;

        // byte_size = min(max_byte_size, byte_size)
        let byte_size = rewriter.make(arith::minui(byte_size, max_byte_size, location))?;

        // bits_to_shift = max_bits - byte_size * bits_per_byte + sign_bit_position_on_byte
        let byte_number_in_bits = rewriter.make(arith::muli(byte_size, bits_per_byte, location))?;
        let value_size_in_bits = rewriter.make(arith::addi(
            byte_number_in_bits,
            sign_bit_position_on_byte,
            location,
        ))?;
        let bits_to_shift = rewriter.make(arith::subi(max_bits, value_size_in_bits, location))?;

        // value_to_extend << bits_to_shift
        let left_shifted_value =
            rewriter.make(llvm::shl(context, value_to_extend, bits_to_shift, location).into())?;

        // value_to_extend >> bits_to_shift  (sign extended)
        rewriter.make(llvm::ashr(context, left_shifted_value, bits_to_shift, location).into())?;
        Ok(())
    }
}
