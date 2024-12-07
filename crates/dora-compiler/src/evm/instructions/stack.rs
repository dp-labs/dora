use melior::ir::{Block, BlockRef, Region};
use num_bigint::BigUint;

use crate::backend::Builder;
use crate::errors::Result;

use crate::evm::{CtxType, EVMCompiler};

impl<'c> EVMCompiler<'c> {
    pub(crate) fn push<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        value_to_push: BigUint,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.iconst_256(value_to_push)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn pop<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        builder.stack_pop()?;
        Ok((start_block, start_block))
    }

    pub(crate) fn dup<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        n: usize,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        debug_assert!(n > 0 && n <= 16);

        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.stack_peek_nth(n)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn dupn<'r>(
        _ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        _stack_index: u8,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        // TODO : Needs EVMBuilder complete
        Ok((start_block, start_block))
    }

    pub(crate) fn swap<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        n: usize,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        debug_assert!(n > 0 && n <= 16);

        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        builder.stack_exchange(0, n)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn swapn<'r>(
        _ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        _stack_index: u8,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        // TODO : Needs EVMBuilder complete
        Ok((start_block, start_block))
    }

    pub(crate) fn exchange<'r>(
        _ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        _imm: u8,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        // TODO : Needs EVMBuilder complete
        Ok((start_block, start_block))
    }
}
