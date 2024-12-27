use dora_runtime::ExitStatusCode;
use melior::dialect::cf;
use melior::ir::{Block, BlockRef, Region};
use num_bigint::BigUint;

use crate::backend::Builder;
use crate::backend::EVMBuilder;
use crate::backend::IntCC;
use crate::errors::Result;

use crate::evm::CtxType;
use crate::evm::EVMCompiler;

impl<'c> EVMCompiler<'c> {
    pub(crate) fn jump<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let pc = builder.stack_pop()?;
        // Appends operation to ok_block to jump to the `jump table block`
        // in the jump table block the pc is checked and if its ok
        // then it jumps to the block associated with that pc
        builder.ctx.add_jump_op(start_block, pc, builder.location());
        let empty_block = region.append_block(Block::new(&[]));
        Ok((start_block, empty_block))
    }

    pub(crate) fn jumpi<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let pc = builder.stack_pop()?;
        let condition = builder.stack_pop()?;
        let false_block = region.append_block(Block::new(&[]));
        let zero = builder.iconst_256(BigUint::ZERO)?;
        let cond = builder.icmp(IntCC::NotEqual, condition, zero)?;
        builder.brif(cond, builder.ctx.jumptable_block, false_block, &[pc], &[]);
        Ok((start_block, false_block))
    }

    pub(crate) fn jumpf<'r>(
        _ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        _target_section_index: u16,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));

        let false_block = region.append_block(Block::new(&[]));
        // TODO : Needs EVMBuilder complete
        Ok((start_block, false_block))
    }

    pub(crate) fn rjump<'r>(
        _ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        _relative_offset: u16,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));

        let empty_block = region.append_block(Block::new(&[]));
        // TODO : Needs EVMBuilder complete
        Ok((start_block, empty_block))
    }

    pub(crate) fn rjumpi<'r>(
        _ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        _relative_offset: u16,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));

        let false_block = region.append_block(Block::new(&[]));
        // TODO : Needs EVMBuilder complete
        Ok((start_block, false_block))
    }

    pub(crate) fn rjumpv<'r>(
        _ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        _max_index: u8,
        _relative_offsets: Vec<u16>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));

        let false_block = region.append_block(Block::new(&[]));
        // TODO : Needs EVMBuilder complete
        Ok((start_block, false_block))
    }

    pub(crate) fn pc<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        pc: usize,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let pc_value = builder.iconst_256(BigUint::from(pc as u64))?;
        builder.stack_push(pc_value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn jumpdest<'r>(
        _ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        _pc: usize,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        // Nothing to do here, register the start block in the outer op loop.
        Ok((start_block, start_block))
    }

    pub(crate) fn revert<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let offset = builder.stack_pop()?;
        let length = builder.stack_pop()?;
        builder.revert(offset, length);
        let empty_block = region.append_block(Block::new(&[]));
        Ok((start_block, empty_block))
    }

    pub(crate) fn stop<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        builder.stop();
        EVMCompiler::return_empty_result(ctx, start_block, ExitStatusCode::Stop)?;
        let empty_block = region.append_block(Block::new(&[]));
        Ok((start_block, empty_block))
    }

    #[inline]
    pub(crate) fn invalid<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        Self::invalid_with_error_code(ctx, region, ExitStatusCode::InvalidFEOpcode)
    }

    pub(crate) fn invalid_with_error_code<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        error_code: ExitStatusCode,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let start_block = region.append_block(Block::new(&[]));
        let empty_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        builder.invalid();
        start_block.append_operation(cf::br(
            &builder.ctx.revert_block,
            &[builder.make(builder.iconst_8(error_code as i8))?],
            builder.location(),
        ));
        Ok((start_block, empty_block))
    }
}
