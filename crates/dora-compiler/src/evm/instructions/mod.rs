use melior::ir::BlockRef;

use crate::conversion::builder::OpBuilder;
use crate::evm::backend::EVMBuilder;
use crate::evm::EVMCompiler;

use super::CtxType;

pub mod arithmetic;
pub mod bitwise;
pub mod contract;
pub mod control;
pub mod host;
pub mod host_env;
pub mod memory;
pub mod stack;
pub mod system;

impl<'c> EVMCompiler<'c> {
    #[inline]
    pub(crate) fn make_builder<'a>(
        ctx: &'a mut CtxType<'c>,
        block: BlockRef<'c, 'a>,
    ) -> EVMBuilder<'a, 'c> {
        EVMBuilder {
            builder: OpBuilder::new_with_block(ctx.context, block),
            ctx,
            use_static_stack: false,
        }
    }
}
