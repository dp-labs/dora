use melior::ir::BlockRef;
use num_bigint::BigUint;

use crate::backend::Builder;
use crate::errors::Result;

use crate::evm::{CtxType, EVMCompiler};

impl<'c> EVMCompiler<'c> {
    pub(crate) fn push<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
        value_to_push: BigUint,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.iconst_256(value_to_push)?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn pop<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        builder.stack_pop()?;
        Ok(start_block)
    }

    pub(crate) fn dup<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
        n: usize,
    ) -> Result<BlockRef<'r, 'c>> {
        debug_assert!(n > 0 && n <= 16);

        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.stack_peek_nth(n)?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn swap<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
        n: usize,
    ) -> Result<BlockRef<'r, 'c>> {
        debug_assert!(n > 0 && n <= 16);

        let mut builder = Self::make_builder(ctx, start_block);
        builder.stack_exchange(0, n)?;
        Ok(start_block)
    }

    pub(crate) fn dupn<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
        stack_index: u8,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.stack_peek_nth(stack_index as usize + 1)?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn swapn<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
        stack_index: u8,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        builder.stack_exchange(0, stack_index as usize + 1)?;
        Ok(start_block)
    }

    pub(crate) fn exchange<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
        imm: u8,
    ) -> Result<BlockRef<'r, 'c>> {
        let n = (imm >> 4) + 1;
        let m = (imm & 0x0F) + 1;
        let mut builder = Self::make_builder(ctx, start_block);
        builder.stack_exchange(n as usize, m as usize)?;
        Ok(start_block)
    }
}
