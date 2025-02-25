use melior::{
    Context, Error,
    ir::Module,
    pass::{self, PassManager},
};

/// Executes a series of optimization and conversion passes on the given [`MLIR`][melior] module.
///
/// This function creates a [`PassManager`] for the provided [`Context`] and adds a set of pre-defined
/// passes that transform and lower the MLIR module into a more optimized and compatible form,
/// targeting LLVM and other relevant dialects.
///
/// # Arguments
///
/// * `context` - A reference to the [`Context`] that holds the MLIR environment.
/// * `module` - A mutable reference to the [`Module`] that will have the passes applied.
///
/// # Returns
///
/// A [`Result<(), Error>`] indicating whether the pass execution was successful (`Ok`) or if it encountered
/// an error ([`Err`]).
///
/// # Passes Applied
///
/// The following passes are applied in sequence:
///
/// * [`transform::create_canonicalizer`][pass::transform::create_canonicalizer] - Applies canonicalization to simplify and optimize the module.
/// * [`conversion::create_scf_to_control_flow`][pass::conversion::create_scf_to_control_flow] - Converts structured control flow (SCF) operations into a more generic control flow form.
/// * [`conversion::create_arith_to_llvm`][pass::conversion::create_arith_to_llvm] - Lowers arithmetic operations to their LLVM counterparts.
/// * [`conversion::create_math_to_llvm`][pass::conversion::create_math_to_llvm] - Converts mathematical operations into LLVM operations.
/// * [`conversion::create_math_to_funcs`][pass::conversion::create_math_to_funcs] - Converts mathematical operations into function calls.
/// * [`conversion::create_control_flow_to_llvm`][pass::conversion::create_control_flow_to_llvm] - Converts control flow operations to LLVM.
/// * [`conversion::create_index_to_llvm`][pass::conversion::create_index_to_llvm] - Converts index types to their LLVM equivalents.
/// * [`conversion::create_finalize_mem_ref_to_llvm`][pass::conversion::create_finalize_mem_ref_to_llvm] - Finalizes memory references for LLVM compatibility.
/// * [`conversion::create_func_to_llvm`][pass::conversion::create_func_to_llvm] - Lowers functions into LLVM form.
/// * [`conversion::create_reconcile_unrealized_casts`][pass::conversion::create_reconcile_unrealized_casts] - Reconciles unrealized cast operations.
///
/// # Example
///
/// ```ignore
/// let context = Context::new();
/// let mut module = Module::empty(&context);
/// run(&context, &mut module)?;
/// ```
pub fn run(context: &Context, module: &mut Module) -> Result<(), Error> {
    let pass_manager = PassManager::new(context);
    pass_manager.enable_verifier(true);
    pass_manager.add_pass(pass::transform::create_canonicalizer());
    pass_manager.add_pass(pass::conversion::create_scf_to_control_flow());
    pass_manager.add_pass(pass::conversion::create_arith_to_llvm());
    pass_manager.add_pass(pass::conversion::create_math_to_llvm());
    pass_manager.add_pass(pass::conversion::create_math_to_funcs());
    pass_manager.add_pass(pass::conversion::create_control_flow_to_llvm());
    pass_manager.add_pass(pass::conversion::create_index_to_llvm());
    pass_manager.add_pass(pass::conversion::create_finalize_mem_ref_to_llvm());
    pass_manager.add_pass(pass::conversion::create_func_to_llvm());
    pass_manager.add_pass(pass::conversion::create_reconcile_unrealized_casts());
    pass_manager.run(module)
}
