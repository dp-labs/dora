use dora_runtime::ExitStatusCode;
use melior::ir::{Block, BlockRef, Region};

use crate::backend::{Builder, EVMBuilder};
use crate::errors::Result;

use crate::evm::{CtxType, EVMCompiler};

impl<'c> EVMCompiler<'c> {
    pub(crate) fn balance<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let account = builder.stack_pop()?;
        let value = builder.balance(account)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn selfbalance<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.selfbalance()?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn extcodesize<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let account = builder.stack_pop()?;
        let value = builder.extcodesize(account)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn extcodehash<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let account = builder.stack_pop()?;
        let value = builder.extcodehash(account)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn extcodecopy<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let account = builder.stack_pop()?;
        let dest_offset = builder.stack_pop()?;
        let code_offset = builder.stack_pop()?;
        let length = builder.stack_pop()?;
        builder.extcodecopy(account, dest_offset, code_offset, length);
        Ok((start_block, start_block))
    }

    pub(crate) fn blockhash<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let block_number = builder.stack_pop()?;
        let block_hash = builder.blockhash(block_number)?;
        builder.stack_push(block_hash)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn sload<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let key = builder.stack_pop()?;
        let value = builder.sload(key)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn sstore<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let key = builder.stack_pop()?;
        let value = builder.stack_pop()?;
        builder.sstore(key, value);
        Ok((start_block, start_block))
    }

    pub(crate) fn tload<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let key = builder.stack_pop()?;
        let value = builder.tload(key)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn tstore<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let key = builder.stack_pop()?;
        let value = builder.stack_pop()?;
        builder.tstore(key, value);
        Ok((start_block, start_block))
    }

    pub(crate) fn log<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        nth: u8,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        debug_assert!(nth <= 4);
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let offset = builder.stack_pop()?;
        let length = builder.stack_pop()?;
        match nth {
            0 => {
                builder.log0(offset, length);
            }
            1 => {
                let topic = builder.stack_pop()?;
                builder.log1(offset, length, topic);
            }
            2 => {
                let topic1 = builder.stack_pop()?;
                let topic2 = builder.stack_pop()?;
                builder.log2(offset, length, topic1, topic2);
            }
            3 => {
                let topic1 = builder.stack_pop()?;
                let topic2 = builder.stack_pop()?;
                let topic3 = builder.stack_pop()?;
                builder.log3(offset, length, topic1, topic2, topic3);
            }
            4 => {
                let topic1 = builder.stack_pop()?;
                let topic2 = builder.stack_pop()?;
                let topic3 = builder.stack_pop()?;
                let topic4 = builder.stack_pop()?;
                builder.log4(offset, length, topic1, topic2, topic3, topic4);
            }
            _ => unreachable!("nth should satisfy 0 <= nth <= 4"),
        }
        Ok((start_block, start_block))
    }

    pub(crate) fn selfdestruct<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let recipient = builder.stack_pop()?;
        builder.selfdestruct(recipient);
        EVMCompiler::return_empty_result(ctx, start_block, ExitStatusCode::Selfdestruct)?;
        let empty_block = region.append_block(Block::new(&[]));
        Ok((start_block, empty_block))
    }
}
