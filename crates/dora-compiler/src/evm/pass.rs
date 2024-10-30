use super::conversion;
use crate::errors::Result;
use conversion::ConversionPass;
use melior::{ir::Module as MLIRModule, Context};

pub fn run(ctx: &Context, module: &mut MLIRModule) -> Result<()> {
    let mut conversion_pass = ConversionPass { ctx };
    conversion_pass.run(module.as_operation())?;
    Ok(())
}
