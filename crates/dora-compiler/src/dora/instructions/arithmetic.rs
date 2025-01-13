use crate::{
    arith_constant,
    backend::IntCC,
    block_argument,
    conversion::rewriter::Rewriter,
    dora::{
        conversion::ConversionPass, gas::compute_exp_cost, memory::allocate_u256_and_assign_value,
    },
    errors::Result,
    gas_or_fail, operands, rewrite_ctx,
};
use dora_primitives::SpecId;
use dora_runtime::symbols;
use dora_runtime::ExitStatusCode;
use melior::{
    dialect::{arith, func, ods::llvm, scf},
    ir::{
        attribute::FlatSymbolRefAttribute, operation::OperationRef, r#type::IntegerType, Block,
        Region, ValueLike,
    },
    Context,
};

impl ConversionPass<'_> {
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

        let uint256 = rewriter.i256_ty();

        let zero = rewriter.make(rewriter.iconst_256_from_u64(0)?)?;
        let is_zero = rewriter.make(rewriter.icmp(IntCC::Equal, r, zero))?;
        rewriter.make(scf::r#if(
            is_zero,
            &[uint256],
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

        let uint256 = rewriter.i256_ty();

        let zero = rewriter.make(rewriter.iconst_256_from_u64(0)?)?;
        let is_zero = rewriter.make(rewriter.icmp(IntCC::Equal, r, zero))?;
        rewriter.make(scf::r#if(
            is_zero,
            &[uint256],
            {
                let region = Region::new();
                let block = region.append_block(Block::new(&[]));
                let rewriter = Rewriter::new_with_block(context, block);
                rewriter.create(scf::r#yield(&[zero], location));
                region
            },
            {
                let region = Region::new();
                let block = region.append_block(Block::new(&[]));
                let rewriter = Rewriter::new_with_block(context, block);
                let result = rewriter.make(arith::remui(l, r, location))?;
                rewriter.create(scf::r#yield(&[result], location));
                region
            },
            location,
        ))?;
        Ok(())
    }

    pub(crate) fn smod(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, l, r);
        rewrite_ctx!(context, op, rewriter, location);

        let uint256 = rewriter.i256_ty();

        let zero = rewriter.make(rewriter.iconst_256_from_u64(0)?)?;
        let is_zero = rewriter.make(rewriter.icmp(IntCC::Equal, r, zero))?;
        rewriter.make(scf::r#if(
            is_zero,
            &[uint256],
            {
                let region = Region::new();
                let block = region.append_block(Block::new(&[]));
                let rewriter = Rewriter::new_with_block(context, block);
                rewriter.create(scf::r#yield(&[zero], location));
                region
            },
            {
                let region = Region::new();
                let block = region.append_block(Block::new(&[]));
                let rewriter = Rewriter::new_with_block(context, block);
                let result = rewriter.make(arith::remsi(l, r, location))?;
                rewriter.create(scf::r#yield(&[result], location));
                region
            },
            location,
        ))?;
        Ok(())
    }

    pub(crate) fn addmod(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, a, b, den);
        rewrite_ctx!(context, op, rewriter, location);

        let uint256 = rewriter.i256_ty();
        let uint257 = rewriter.i257_ty();

        let a_i257 = rewriter.make(arith::extui(a, uint257, location))?;
        let b_i257 = rewriter.make(arith::extui(b, uint257, location))?;
        let den_i257 = rewriter.make(arith::extui(den, uint257, location))?;
        let add = rewriter.make(arith::addi(a_i257, b_i257, location))?;
        let umod = rewriter.make(arith::remui(add, den_i257, location))?;
        let result = rewriter.make(arith::trunci(umod, uint256, location))?;

        let zero = rewriter.make(rewriter.iconst_256_from_u64(0)?)?;
        let is_zero = rewriter.make(rewriter.icmp(IntCC::Equal, den, zero))?;
        rewriter.make(arith::select(is_zero, zero, result, location))?;
        Ok(())
    }

    pub(crate) fn mulmod(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, a, b, den);
        rewrite_ctx!(context, op, rewriter, location);

        let uint256 = rewriter.i256_ty();

        let uint512 = IntegerType::new(context, 512);
        let a_i512 = rewriter.make(arith::extui(a, uint512.into(), location))?;
        let b_i512 = rewriter.make(arith::extui(b, uint512.into(), location))?;
        let den_i512 = rewriter.make(arith::extui(den, uint512.into(), location))?;
        let add = rewriter.make(arith::muli(a_i512, b_i512, location))?;
        let umod = rewriter.make(arith::remui(add, den_i512, location))?;
        let result = rewriter.make(arith::trunci(umod, uint256, location))?;

        let zero = rewriter.make(rewriter.iconst_256_from_u64(0)?)?;
        let is_zero = rewriter.make(rewriter.icmp(IntCC::Equal, den, zero))?;
        rewriter.make(arith::select(is_zero, zero, result, location))?;
        Ok(())
    }

    pub(crate) fn exp(
        context: &Context,
        op: &OperationRef<'_, '_>,
        spec_id: &SpecId,
    ) -> Result<()> {
        operands!(op, l, r);
        block_argument!(op, _system_ctx, gas_counter_ptr);
        rewrite_ctx!(context, op, rewriter, NoDefer);

        let uint256 = rewriter.i256_ty();

        let gas = compute_exp_cost(&rewriter, r, spec_id)?;
        gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);
        // Note the power i256 overflow, thus we use the pow runtime function to deal this situation.
        let base_ptr = allocate_u256_and_assign_value(context, &rewriter, l, location)?;
        let exponent_ptr = allocate_u256_and_assign_value(context, &rewriter, r, location)?;
        rewriter.create(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::EXP),
            &[syscall_ctx.into(), base_ptr, exponent_ptr],
            &[],
            location,
        ));
        rewriter.create(rewriter.load(exponent_ptr, uint256));
        Ok(())
    }

    pub(crate) fn signextend(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, byte_size, value_to_extend);
        rewrite_ctx!(context, op, rewriter, location);

        let ty = value_to_extend.r#type();
        let bit_width = rewriter.int_ty_width(ty)? as i64;

        // Constant Definitions
        let max_byte_size = rewriter.make(arith_constant!(
            rewriter,
            context,
            ty,
            bit_width / 8 - 1,
            location
        ))?;
        let bits_per_byte = rewriter.make(arith_constant!(rewriter, context, ty, 8, location))?;
        let sign_bit_position_on_byte =
            rewriter.make(arith_constant!(rewriter, context, ty, 7, location))?;
        let max_bits = rewriter.make(arith_constant!(
            rewriter,
            context,
            ty,
            bit_width - 1,
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

    pub(crate) fn select(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, lhs, rhs, cond);
        rewrite_ctx!(context, op, rewriter, location);

        let cond_value = rewriter.make(rewriter.icmp_imm(IntCC::NotEqual, cond, 0)?)?;
        rewriter.create(arith::select(cond_value, lhs, rhs, location));
        Ok(())
    }
}
