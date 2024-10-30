#[cfg(test)]
mod tests;

// Define EVM dialects `pub mod evm {...}`, we can import EVM dialect operations
// through `use dora_ir::evm;` from external crates or `use crate::evm;` from
// the current crate.
melior::dialect! {
    name: "evm",
    td_file: "crates/dora-ir/src/include/evm.td",
}

// Define WASM dialects `pub mod wasm {...}`, we can import WASM dialect operations
// through `use dora_ir::wasm;` from external crates or `use crate::wasm;` from
// the current crate.
melior::dialect! {
    name: "wasm",
    td_file: "crates/dora-ir/src/include/wasm.td",
}

// Define Dora dialects `pub mod dora {...}`, we can import Dora dialect operations
// through `use dora_ir::dora;` from external crates. or `use crate::dora;` from
// the current crate.
melior::dialect! {
    name: "dora",
    td_file: "crates/dora-ir/src/include/dora.td",
}

mod traits;
pub use traits::IRTypes;

mod operations;
pub use operations::Operation;
