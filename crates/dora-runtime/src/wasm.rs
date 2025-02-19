use wasmer::{Exports, Store};
use wasmer_vm::{VMContext, VMInstance, VMOffsets};

pub mod context;
pub mod env;
pub mod errors;
pub mod host;
pub mod memory;
pub mod ptr;
pub mod trap;

/// Define a struct `WASMInstance` to represent a WASM instance for the dora runtime.
#[derive(Debug)]
pub struct WASMInstance {
    /// The store represents all global state that can be manipulated by WASM programs.
    /// It consists of the runtime representation of all instances of functions, tables,
    /// memories, and globals that have been allocated during the lifetime of the abstract machine.
    pub store: Store,
    /// A handle holding an Instance of a WASM module.
    pub instance: VMInstance,
    /// Exports is a special kind of map that allows easily unwrapping the types of instances.
    pub exports: Exports,
}

impl WASMInstance {
    /// Return a raw pointer to the vmctx used by compiled wasm code.
    #[inline]
    pub fn vmctx_ptr(&self) -> *mut VMContext {
        self.instance.vmctx_ptr()
    }

    /// Return a reference to the VMOffsets to get offsets in the Self::vmctx_ptr region.
    pub fn vmoffsets(&self) -> &VMOffsets {
        self.instance.vmoffsets()
    }
}
