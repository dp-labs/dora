use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("error linking: {0}")]
    Link(#[from] std::io::Error),
    #[error("llvm compile error: {0}")]
    LLVM(String),
    #[error("codegen error: {0}")]
    Codegen(String),
    #[error("melior error: {0}")]
    Melior(#[from] melior::Error),
    #[error("not yet implemented: {0}")]
    NotImplemented(String),
}

/// Compilation result.
pub type Result<T> = anyhow::Result<T>;

/// Compilation error.
pub type Error = anyhow::Error;
