use melior::ir::BlockRef;

use crate::backend::{Builder, EVMBuilder};
use crate::errors::Result;

use crate::evm::{CtxType, EVMCompiler};

impl<'c> EVMCompiler<'c> {
    pub(crate) fn chainid<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.chainid()?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn coinbase<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.coinbase()?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn timestamp<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.timestamp()?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn number<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.number()?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn prevrandao<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.prevrandao()?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn gaslimit<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.gaslimit()?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn gasprice<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.gasprice()?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn basefee<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.basefee()?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn origin<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.origin()?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn blobhash<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let index = builder.stack_pop()?;
        let value = builder.blobhash(index)?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn blobbasefee<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let value = builder.blobbasefee()?;
        builder.stack_push(value)?;
        Ok(start_block)
    }
}
