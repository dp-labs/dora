use crate::constants::MAX_STACK_SIZE;
use dora_primitives::Bytes32;

#[repr(C)]
pub struct Stack([Bytes32; MAX_STACK_SIZE]);

impl Stack {
    #[inline]
    pub const fn new() -> Self {
        Self([Bytes32::ZERO; MAX_STACK_SIZE])
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}
