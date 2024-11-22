use self::gas::GasOptions;
use super::{conversion, gas, storage};
use crate::errors::Result;
use melior::{ir::Module as MLIRModule, Context};

/// Options for configuring a pass, including program-related settings.
///
/// This struct contains various configuration options that are used during pass execution.
/// Currently, it includes the `program_code_size`, which specifies the size of the program's code.
///
/// # Fields
///
/// - `program_code_size`: The size of the program code, in bytes.
#[derive(Debug)]
pub struct PassOptions {
    pub program_code_size: u32,
}

/// Executes the conversion pass on the given MLIR module.
///
/// This function runs a conversion pass on the provided `MLIRModule` using the options specified
/// in `PassOptions`. The conversion process is handled by the `ConversionPass` struct, which takes
/// the program's code size as a parameter.
///
/// # Parameters
///
/// - `_context`: The MLIR `Context` in which the pass is executed. This is not used directly in the
///   function but passed in for compatibility.
/// - `module`: The `MLIRModule` on which the conversion pass will be run.
/// - `opts`: The `PassOptions` containing configuration for the pass, such as the program code size.
///
/// # Returns
///
/// - `Result<()>`: Returns `Ok(())` if the pass runs successfully, or an error if the process fails.
///
/// # Example
///
/// ```no_check
/// let opts = PassOptions { program_code_size: 1024 };
/// run(&context, &mut mlir_module, &opts)?;
/// ```
pub fn run(ctx: &Context, module: &mut MLIRModule, opts: &PassOptions) -> Result<()> {
    let mut conversion_pass = conversion::ConversionPass {
        ctx,
        program_code_size: opts.program_code_size,
    };
    conversion_pass.run(module.as_operation())
}

/// Executes the gas pass on the given MLIR module.
///
/// This function runs a gas pass on the provided `MLIRModule`. The gas pass is responsible for
/// inserting and managing gas checks within the module's operations. The `GasPass` struct is used
/// to perform the necessary transformations on the module.
///
/// # Parameters
///
/// - `_context`: The MLIR `Context` in which the pass is executed. This parameter is not used directly,
///   but is passed for compatibility with the framework.
/// - `module`: The `MLIRModule` on which the gas pass will be run.
///
/// # Returns
///
/// - `Result<()>`: Returns `Ok(())` if the gas pass runs successfully, or an error if the process fails.
///
/// # Example
///
/// ```no_check
/// run_gas_pass(&context, &mut mlir_module)?;
/// ```
pub fn run_gas_pass(ctx: &Context, module: &mut MLIRModule, options: GasOptions) -> Result<()> {
    let mut gas_pass = gas::GasPass { ctx, options };
    gas_pass.run(module.as_operation())
}

/// Run the storage optimization pass on the given MLIR module
///
/// # Arguments
/// * `ctx` - The MLIR context containing dialect registry and configurations
/// * `module` - The MLIR module to be optimized
///
/// # Returns
/// * `Result<()>` - Ok if the pass succeeds, Err with error message if it fails
///
/// # Description
/// This function:
/// 1. Creates a new StoragePass instance which handles memory and storage optimizations
/// 2. Runs the pass on all operations in the module
/// 3. Transforms storage operations into more efficient forms
/// 4. Optimizes storage access patterns
pub fn run_storage_pass(ctx: &Context, module: &mut MLIRModule) -> Result<()> {
    let mut storage_pass = storage::StoragePass::new(ctx);
    storage_pass.run(module.as_operation())
}
