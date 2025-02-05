use wasmer::{FunctionEnvMut, Memory};

/// A WASM env for the host functions.
pub struct WASMEnv {
    /// Mechanism for reading and writing the module's memory
    pub memory: Option<Memory>,
}

/// A WASM mut env for the host functions.
pub type WASMEnvMut<'a> = FunctionEnvMut<'a, WASMEnv>;
