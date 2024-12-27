use melior::ir::{Block, BlockRef, Region};

use crate::backend::{Builder, EVMBuilder};
use crate::errors::Result;

use crate::evm::{CtxType, EVMCompiler};

impl<'c> EVMCompiler<'c> {
    pub(crate) fn create<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.stack_pop()?;
        let offset = builder.stack_pop()?;
        let length = builder.stack_pop()?;
        let value = builder.create(value, offset, length)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn create2<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.stack_pop()?;
        let offset = builder.stack_pop()?;
        let length = builder.stack_pop()?;
        let salt = builder.stack_pop()?;
        let value = builder.create2(value, offset, length, salt)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn eofcreate<'r>(
        _ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        _initcontainer_index: u8,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        // TODO : Needs EVMBuilder complete
        Ok((start_block, start_block))
    }

    pub(crate) fn returncontract<'r>(
        _ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        _deploy_container_index: u8,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        // TODO : Needs EVMBuilder complete
        Ok((start_block, start_block))
    }

    pub(crate) fn call<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let gas = builder.stack_pop()?;
        let address = builder.stack_pop()?;
        let value = builder.stack_pop()?;
        let input_offset = builder.stack_pop()?;
        let input_length = builder.stack_pop()?;
        let output_offset = builder.stack_pop()?;
        let output_length = builder.stack_pop()?;
        let value = builder.call(
            gas,
            address,
            value,
            input_offset,
            input_length,
            output_offset,
            output_length,
        )?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn callcode<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let gas = builder.stack_pop()?;
        let address = builder.stack_pop()?;
        let value = builder.stack_pop()?;
        let input_offset = builder.stack_pop()?;
        let input_length = builder.stack_pop()?;
        let output_offset = builder.stack_pop()?;
        let output_length = builder.stack_pop()?;
        let value = builder.callcode(
            gas,
            address,
            value,
            input_offset,
            input_length,
            output_offset,
            output_length,
        )?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn delegatecall<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let gas = builder.stack_pop()?;
        let address = builder.stack_pop()?;
        let input_offset = builder.stack_pop()?;
        let input_length = builder.stack_pop()?;
        let output_offset = builder.stack_pop()?;
        let output_length = builder.stack_pop()?;
        let value = builder.delegatecall(
            gas,
            address,
            input_offset,
            input_length,
            output_offset,
            output_length,
        )?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn staticcall<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let gas = builder.stack_pop()?;
        let address = builder.stack_pop()?;
        let input_offset = builder.stack_pop()?;
        let input_length = builder.stack_pop()?;
        let output_offset = builder.stack_pop()?;
        let output_length = builder.stack_pop()?;
        let value = builder.staticcall(
            gas,
            address,
            input_offset,
            input_length,
            output_offset,
            output_length,
        )?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn extcall<'r>(
        _ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        // TODO : Needs EVMBuilder complete
        Ok((start_block, start_block))
    }

    pub(crate) fn extdelegatecall<'r>(
        _ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        // TODO : Needs EVMBuilder complete
        Ok((start_block, start_block))
    }

    pub(crate) fn extstaticcall<'r>(
        _ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        // TODO : Needs EVMBuilder complete
        Ok((start_block, start_block))
    }

    pub(crate) fn creturn<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let offset = builder.stack_pop()?;
        let length = builder.stack_pop()?;
        builder.creturn(offset, length);
        let empty_block = region.append_block(Block::new(&[]));
        Ok((start_block, empty_block))
    }
}
