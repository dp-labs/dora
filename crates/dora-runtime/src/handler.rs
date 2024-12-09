use std::sync::Arc;

use crate::{
    call::CallResult,
    context::{Contract, VMContext},
    db::Database,
    result::EVMError,
};

#[derive(Debug)]
pub struct Frame {
    /// Contract infortmation.
    pub contract: Contract,
    /// The gas limit
    pub gas_limit: u64,
}

pub type CallFrameHandle<'a, DB> =
    Arc<dyn Fn(Frame, &mut VMContext<'a, DB>) -> Result<CallResult, EVMError> + 'a>;

/// Handler acts as a proxy and allow to define different behavior for different
/// sections of the code.
pub struct Handler<'a, DB: Database> {
    /// Call frame handler.
    pub call_frame: CallFrameHandle<'a, DB>,
}
