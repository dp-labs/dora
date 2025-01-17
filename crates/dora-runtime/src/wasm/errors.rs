use thiserror::*;
use wasmer::MemoryAccessError;

pub type MaybeEscape = Result<(), Escape>;
pub type EscapeResult<T> = Result<T, Escape>;

#[derive(Error, Debug)]
pub enum Escape {
    #[error("failed to access memory: `{0}`")]
    Memory(MemoryAccessError),
    #[error("internal error: `{0}`")]
    Internal(String),
    #[error("logic error: `{0}`")]
    Logical(String),
    #[error("out of gas")]
    OutOfGas,
    #[error("exit early: `{0}`")]
    Exit(u32),
}

impl Escape {
    pub fn internal<T>(error: &'static str) -> Result<T, Escape> {
        Err(Self::Internal(error.to_string()))
    }

    pub fn logical<T>(error: &'static str) -> Result<T, Escape> {
        Err(Self::Logical(error.to_string()))
    }

    pub fn out_of_gas<T>() -> Result<T, Escape> {
        Err(Self::OutOfGas)
    }
}

impl From<MemoryAccessError> for Escape {
    fn from(err: MemoryAccessError) -> Self {
        Self::Memory(err)
    }
}
