use melior::ir::{Block, BlockRef, Region};

use crate::backend::{Builder, EVMBuilder};
use crate::errors::Result;

use crate::evm::{CtxType, EVMCompiler};

impl<'c> EVMCompiler<'c> {
    pub(crate) fn create<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
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
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
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
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
        initcontainer_index: u8,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let initcontainer_index = builder.iconst_8(initcontainer_index as i8)?;
        let value = builder.stack_pop()?;
        let salt = builder.stack_pop()?;
        let input_offset = builder.stack_pop()?;
        let input_size = builder.stack_pop()?;
        let value =
            builder.eofcreate(initcontainer_index, value, salt, input_offset, input_size)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn returncontract<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
        deploy_container_index: u8,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let deploy_container_index = builder.iconst_8(deploy_container_index as i8)?;
        let aux_data_offset = builder.stack_pop()?;
        let aux_data_size = builder.stack_pop()?;
        builder.returncontract(deploy_container_index, aux_data_offset, aux_data_size);
        Ok((start_block, start_block))
    }

    pub(crate) fn call<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
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
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
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
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
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
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
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
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let target_address = builder.stack_pop()?;
        let input_offset = builder.stack_pop()?;
        let input_size = builder.stack_pop()?;
        let value = builder.stack_pop()?;
        let value = builder.extcall(target_address, input_offset, input_size, value)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn extdelegatecall<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let target_address = builder.stack_pop()?;
        let input_offset = builder.stack_pop()?;
        let input_size = builder.stack_pop()?;
        let value = builder.extdelegatecall(target_address, input_offset, input_size)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn extstaticcall<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let target_address = builder.stack_pop()?;
        let input_offset = builder.stack_pop()?;
        let input_size = builder.stack_pop()?;
        let value = builder.extstaticcall(target_address, input_offset, input_size)?;
        builder.stack_push(value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn creturn<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let offset = builder.stack_pop()?;
        let length = builder.stack_pop()?;
        builder.creturn(offset, length);
        let empty_block = region.append_block(Block::new(&[]));
        Ok((start_block, empty_block))
    }
}
