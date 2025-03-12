use melior::ir::BlockRef;

use crate::backend::Builder;
use crate::errors::Result;

use crate::evm::{CtxType, EVMCompiler};

impl<'c> EVMCompiler<'c> {
    pub(crate) fn add<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let lhs = builder.stack_pop()?;
        let rhs = builder.stack_pop()?;
        let value = builder.iadd(lhs, rhs)?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn mul<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let lhs = builder.stack_pop()?;
        let rhs = builder.stack_pop()?;
        let value = builder.imul(lhs, rhs)?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn sub<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let lhs = builder.stack_pop()?;
        let rhs = builder.stack_pop()?;
        let value = builder.isub(lhs, rhs)?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn udiv<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let lhs = builder.stack_pop()?;
        let rhs = builder.stack_pop()?;
        let value = builder.udiv(lhs, rhs)?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn sdiv<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let lhs = builder.stack_pop()?;
        let rhs = builder.stack_pop()?;
        let value = builder.sdiv(lhs, rhs)?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn umod<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let lhs = builder.stack_pop()?;
        let rhs = builder.stack_pop()?;
        let value = builder.umod(lhs, rhs)?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn smod<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let lhs = builder.stack_pop()?;
        let rhs = builder.stack_pop()?;
        let value = builder.smod(lhs, rhs)?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn addmod<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let lhs = builder.stack_pop()?;
        let rhs = builder.stack_pop()?;
        let modulus = builder.stack_pop()?;
        let value = builder.addmod(lhs, rhs, modulus)?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn mulmod<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let lhs = builder.stack_pop()?;
        let rhs = builder.stack_pop()?;
        let modulus = builder.stack_pop()?;
        let value = builder.mulmod(lhs, rhs, modulus)?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn exp<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let base = builder.stack_pop()?;
        let exponent = builder.stack_pop()?;
        let value = builder.exp(base, exponent)?;
        builder.stack_push(value)?;
        Ok(start_block)
    }

    pub(crate) fn signextend<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<BlockRef<'r, 'c>> {
        let mut builder = Self::make_builder(ctx, start_block);
        let byte = builder.stack_pop()?;
        let value = builder.stack_pop()?;
        let value = builder.signextend(byte, value)?;
        builder.stack_push(value)?;
        Ok(start_block)
    }
}
