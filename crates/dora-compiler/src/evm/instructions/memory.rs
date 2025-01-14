use melior::ir::BlockRef;

use crate::backend::{Builder, EVMBuilder};
use crate::errors::Result;

use crate::evm::{CtxType, EVMCompiler};

impl<'c> EVMCompiler<'c> {
    pub(crate) fn mload<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let offset = builder.stack_pop()?;
        let value = builder.mload(offset)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn mstore<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let offset = builder.stack_pop()?;
        let data = builder.stack_pop()?;
        builder.mstore(offset, data);
        Ok((start_block, start_block))
    }

    pub(crate) fn mstore8<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let offset = builder.stack_pop()?;
        let data = builder.stack_pop()?;
        builder.mstore8(offset, data);
        Ok((start_block, start_block))
    }

    pub(crate) fn msize<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.msize()?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn mcopy<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let dest_offset = builder.stack_pop()?;
        let src_offset = builder.stack_pop()?;
        let length = builder.stack_pop()?;
        builder.mcopy(dest_offset, src_offset, length);
        Ok((start_block, start_block))
    }
}
