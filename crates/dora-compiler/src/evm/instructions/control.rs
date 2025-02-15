use dora_runtime::ExitStatusCode;
use melior::dialect::{arith, cf};
use melior::ir::{Block, BlockRef, Region};
use num_bigint::BigUint;

use crate::backend::Builder;
use crate::backend::EVMBuilder;
use crate::backend::IntCC;
use crate::conversion::builder::OpBuilder;
use crate::errors::Result;
use crate::evm::MAX_STACK_SIZE;

use crate::evm::CtxType;
use crate::evm::EVMCompiler;

impl<'c> EVMCompiler<'c> {
    pub(crate) fn jump<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
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
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let pc = builder.stack_pop()?;
        let condition = builder.stack_pop()?;
        let false_block = region.append_block(Block::new(&[]));
        let zero = builder.iconst_256(BigUint::ZERO)?;
        let cond = builder.icmp(IntCC::NotEqual, condition, zero)?;
        builder.brif(cond, builder.ctx.jumptable_block, false_block, &[pc], &[]);
        Ok((start_block, false_block))
    }

    pub(crate) fn rjump<'r>(
        _ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        start_block: BlockRef<'r, 'c>,
        _relative_offset: u16,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let empty_block = region.append_block(Block::new(&[]));
        // TODO : Needs EVMBuilder complete
        Ok((start_block, empty_block))
    }

    pub(crate) fn rjumpi<'r>(
        _ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        start_block: BlockRef<'r, 'c>,
        _relative_offset: u16,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let false_block = region.append_block(Block::new(&[]));
        // TODO : Needs EVMBuilder complete
        Ok((start_block, false_block))
    }

    pub(crate) fn rjumpv<'r>(
        _ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        start_block: BlockRef<'r, 'c>,
        _max_index: u8,
        _relative_offsets: Vec<u16>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let false_block = region.append_block(Block::new(&[]));
        // TODO : Needs EVMBuilder complete
        Ok((start_block, false_block))
    }

    pub(crate) fn callf<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        start_block: BlockRef<'r, 'c>,
        target_section_index: u16,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        Self::callf_common(ctx, region, start_block, target_section_index, false)
    }

    pub(crate) fn retf<'r>(
        _ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        // TODO : Needs EVMBuilder complete
        Ok((start_block, start_block))
    }

    pub(crate) fn jumpf<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        start_block: BlockRef<'r, 'c>,
        target_section_index: u16,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        Self::callf_common(ctx, region, start_block, target_section_index, true)
    }

    pub(crate) fn pc<'r>(
        ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
        pc: usize,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let mut builder = Self::make_builder(ctx, start_block);
        let pc_value = builder.iconst_256(BigUint::from(pc as u64))?;
        builder.stack_push(pc_value)?;
        Ok((start_block, start_block))
    }

    pub(crate) fn jumpdest<'r>(
        _ctx: &mut CtxType<'c>,
        start_block: BlockRef<'r, 'c>,
        _pc: usize,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        // Nothing to do here, register the start block in the outer op loop.
        Ok((start_block, start_block))
    }

    pub(crate) fn revert<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
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
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
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
        start_block: BlockRef<'r, 'c>,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        Self::invalid_with_error_code(ctx, region, start_block, ExitStatusCode::InvalidFEOpcode)
    }

    pub(crate) fn invalid_with_error_code<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        start_block: BlockRef<'r, 'c>,
        error_code: ExitStatusCode,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let empty_block = region.append_block(Block::new(&[]));
        let mut builder = Self::make_builder(ctx, start_block);
        let error_code = builder.iconst_8(error_code as i8)?;
        builder.invalid();
        start_block.append_operation(cf::br(
            &builder.ctx.revert_block,
            &[error_code],
            builder.location(),
        ));
        Ok((start_block, empty_block))
    }

    /// Deals `CALLF` and `JUMPF` instruction.
    pub(crate) fn callf_common<'r>(
        ctx: &mut CtxType<'c>,
        region: &'r Region<'c>,
        start_block: BlockRef<'r, 'c>,
        target_section_index: u16,
        is_jumpf: bool,
    ) -> Result<(BlockRef<'r, 'c>, BlockRef<'r, 'c>)> {
        let empty_block = region.append_block(Block::new(&[]));
        let builder = OpBuilder::new_with_block(ctx.context, start_block);
        let eof = ctx.program.eof().ok_or(anyhow::anyhow!(
            "internal error: encountered EOF operators but the EOF container is empty"
        ))?;
        let types = eof
            .body
            .types_section
            .get(target_section_index as usize)
            .ok_or(anyhow::anyhow!(
                "section {target_section_index}: types not found"
            ))?;
        let max_height = types.max_stack_size - types.inputs as u16;
        let mut max_len =
            builder.make(builder.load(ctx.values.stack_size_ptr, builder.i64_ty()))?;
        if max_height != 0 {
            max_len = builder.make(arith::addi(
                max_len,
                builder.make(builder.iconst_64(max_height as i64))?,
                builder.get_insert_location(),
            ))?;
        }

        let revert = builder.make(builder.icmp_imm(
            IntCC::UnsignedGreaterThan,
            max_len,
            MAX_STACK_SIZE as i64,
        )?)?;
        let code = builder.make(builder.iconst_8(ExitStatusCode::StackOverflow.to_u8() as i8))?;
        builder.create(cf::cond_br(
            ctx.context,
            revert,
            &ctx.revert_block,
            &empty_block,
            &[code],
            &[],
            builder.get_insert_location(),
        ));
        // TODO : runtime function stack
        if is_jumpf {
            // function stack push
        } else {
            // call function stack push
        }
        // TODO : direct jump to the op block
        let op_index = ctx.program.eof_section_index(target_section_index as usize);
        let op_block = ctx.operation_blocks[op_index];
        builder.create(cf::br(&op_block, &[], builder.get_insert_location()));
        Ok((start_block, empty_block))
    }
}
