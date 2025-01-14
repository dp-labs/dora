use melior::ir::BlockRef;

use crate::backend::Builder;
use crate::errors::Result;

use crate::evm::{CtxType, EVMCompiler};

impl<'c> EVMCompiler<'c> {
    pub(crate) fn lt<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let lhs = builder.stack_pop()?;
        let rhs = builder.stack_pop()?;
        let value = builder.lt(lhs, rhs)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn gt<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let lhs = builder.stack_pop()?;
        let rhs = builder.stack_pop()?;
        let value = builder.gt(lhs, rhs)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn slt<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let lhs = builder.stack_pop()?;
        let rhs = builder.stack_pop()?;
        let value = builder.slt(lhs, rhs)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn sgt<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let lhs = builder.stack_pop()?;
        let rhs = builder.stack_pop()?;
        let value = builder.sgt(lhs, rhs)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn eq<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let lhs = builder.stack_pop()?;
        let rhs = builder.stack_pop()?;
        let value = builder.eq(lhs, rhs)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn iszero<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.stack_pop()?;
        let value = builder.iszero(value)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn and<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let lhs = builder.stack_pop()?;
        let rhs = builder.stack_pop()?;
        let value = builder.and(lhs, rhs)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn or<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let lhs = builder.stack_pop()?;
        let rhs = builder.stack_pop()?;
        let value = builder.or(lhs, rhs)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn xor<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let lhs = builder.stack_pop()?;
        let rhs = builder.stack_pop()?;
        let value = builder.xor(lhs, rhs)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn not<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.stack_pop()?;
        let value = builder.not(value)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn byte<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let index = builder.stack_pop()?;
        let value = builder.stack_pop()?;
        let value = builder.byte(index, value)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn shl<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let shift = builder.stack_pop()?;
        let value = builder.stack_pop()?;
        let value = builder.shl(shift, value)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn shr<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let shift = builder.stack_pop()?;
        let value = builder.stack_pop()?;
        let value = builder.shr(shift, value)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn sar<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let shift = builder.stack_pop()?;
        let value = builder.stack_pop()?;
        let value = builder.sar(shift, value)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }
}
