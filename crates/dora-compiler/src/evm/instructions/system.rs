use melior::ir::{Block, BlockRef, Region};

use crate::backend::{Builder, EVMBuilder};
use crate::errors::Result;

use crate::evm::{CtxType, EVMCompiler};

impl<'c> EVMCompiler<'c> {
    pub(crate) fn keccak256<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let start = builder.stack_pop()?;
        let length = builder.stack_pop()?;
        let value = builder.keccak256(start, length)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn address<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.address()?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn caller<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.caller()?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn callvalue<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.callvalue()?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn calldataload<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let offset = builder.stack_pop()?;
        let value = builder.calldataload(offset)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn calldatasize<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.calldatasize()?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn calldatacopy<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let dest_offset = builder.stack_pop()?;
        let data_offset = builder.stack_pop()?;
        let length = builder.stack_pop()?;
        builder.calldatacopy(dest_offset, data_offset, length);
        Ok((start_block, start_block))
    }

    pub(crate) fn dataload<'r>(
        _ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        // TODO : Needs EVMBuilder complete
        Ok((start_block, start_block))
    }

    pub(crate) fn dataloadn<'r>(
        _ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        _offset: u16,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        // TODO : Needs EVMBuilder complete
        Ok((start_block, start_block))
    }

    pub(crate) fn datasize<'r>(
        _ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        // TODO : Needs EVMBuilder complete
        Ok((start_block, start_block))
    }

    pub(crate) fn datacopy<'r>(
        _ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        // TODO : Needs EVMBuilder complete
        Ok((start_block, start_block))
    }

    pub(crate) fn codesize<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.codesize()?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn codecopy<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let dest_offset = builder.stack_pop()?;
        let data_offset = builder.stack_pop()?;
        let length = builder.stack_pop()?;
        builder.codecopy(dest_offset, data_offset, length);
        Ok((start_block, start_block))
    }

    pub(crate) fn returndataload<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let offset = builder.stack_pop()?;
        let data = builder.returndataload(offset)?;
        builder.stack_push(data)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn returndatasize<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.returndatasize()?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn returndatacopy<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let dest_offset = builder.stack_pop()?;
        let data_offset = builder.stack_pop()?;
        let length = builder.stack_pop()?;
        builder.returndatacopy(dest_offset, data_offset, length);
        Ok((start_block, start_block))
    }

    pub(crate) fn gas<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.gas()?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }
}
