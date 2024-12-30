use super::conversion;
use crate::errors::Result;
use conversion::ConversionPass;
use melior::{ir::Module as MLIRModule, Context};

/// Define a public function `run` to execute the conversion process.
///
/// # Parameters
/// - `ctx`: An MLIR context, used to manage the global state of MLIR.
/// - `module`: A mutable MLIR module representing the MLIR code to be transformed.
///
/// # Returns
/// - `Result<()>`, indicating success or failure of the operation.
pub fn run(ctx: &Context, module: &mut MLIRModule) -> Result<()> {
    let mut conversion_pass = ConversionPass { ctx };
    conversion_pass.run(module.as_operation())
}
