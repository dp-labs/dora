use wasmer_types::WasmError;

use crate::errors::CompileError;

impl From<WasmError> for CompileError {
    fn from(value: WasmError) -> Self {
        CompileError::Codegen(format!("{}", value))
    }
}
