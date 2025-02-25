use crate::context::Context;

use super::intrinsics::WASMIntrinsics;
use melior::{
    Context as MLIRContext,
    ir::{
        Attribute,
        attribute::{FloatAttribute, IntegerAttribute, TypeAttribute},
    },
};
use wasmer_types::{FunctionType, Type};

/// Converts a WebAssembly function signature to an MLIR function type.
///
/// This function maps a WebAssembly function signature (`sig`) to its corresponding
/// MLIR function type. The resulting function type includes the WebAssembly function
/// parameters, a context pointer, and the appropriate return types.
///
/// # Parameters
///
/// - `context`: The MLIR context in which the function type is created.
/// - `intrinsics`: The WebAssembly intrinsics that provide predefined MLIR types.
/// - `sig`: The WebAssembly function signature to be converted.
///
/// # Returns
///
/// - `melior::ir::r#type::FunctionType<'c>`: The MLIR function type corresponding to the WebAssembly signature.
///
/// # Example
///
/// ```no_check
/// let mlir_func_type = func_type_to_mlir(&context, &wasm_intrinsics, &wasm_sig);
/// ```
pub fn func_type_to_mlir<'c>(
    context: &'c Context,
    intrinsics: &WASMIntrinsics<'c>,
    sig: &FunctionType,
) -> melior::ir::r#type::FunctionType<'c> {
    let param_types = sig.params().iter().map(|ty| type_to_mlir(intrinsics, ty));
    // Add the WASM vm context pointer as the first parameter
    let param_types = std::iter::once(intrinsics.ptr_ty).chain(param_types);
    let return_types = sig.results().iter().map(|ty| type_to_mlir(intrinsics, ty));
    melior::ir::r#type::FunctionType::new(
        &context.mlir_context,
        &param_types.collect::<Vec<melior::ir::Type>>(),
        &return_types.collect::<Vec<melior::ir::Type>>(),
    )
}

/// Converts a WebAssembly type to its corresponding MLIR type.
///
/// This function maps a WebAssembly type (`ty`) to its corresponding MLIR type
/// using the provided `WASMIntrinsics`.
///
/// # Parameters
///
/// - `intrinsics`: The WebAssembly intrinsics that provide predefined MLIR types.
/// - `ty`: The WebAssembly type to be converted.
///
/// # Returns
///
/// - `melior::ir::r#type::Type<'c>`: The MLIR type corresponding to the WebAssembly type.
///
/// # Example
///
/// ```no_check
/// let mlir_type = type_to_mlir(&wasm_intrinsics, Type::I32);
/// ```
#[inline]
pub fn type_to_mlir<'c>(
    intrinsics: &WASMIntrinsics<'c>,
    ty: &Type,
) -> melior::ir::r#type::Type<'c> {
    match ty {
        Type::I32 => intrinsics.i32_ty,
        Type::I64 => intrinsics.i64_ty,
        Type::F32 => intrinsics.f32_ty,
        Type::F64 => intrinsics.f64_ty,
        Type::V128 => intrinsics.i128_ty,
        Type::FuncRef => intrinsics.func_ref_ty,
        Type::ExternRef => intrinsics.extern_ref_ty,
    }
}

/// Creates an MLIR zero attribute for a given WebAssembly type.
///
/// This function generates an MLIR zero attribute for the provided WebAssembly
/// type (`ty`). It assigns the zero value based on the type's nature (e.g., integer, float).
///
/// # Parameters
///
/// - `context`: The MLIR context in which the attribute is created.
/// - `intrinsics`: The WebAssembly intrinsics that provide predefined MLIR types.
/// - `ty`: The WebAssembly type for which to generate the zero attribute.
///
/// # Returns
///
/// - `Attribute<'c>`: The MLIR zero attribute corresponding to the WebAssembly type.
///
/// # Example
///
/// ```no_check
/// let zero_attr = type_to_mlir_zero_attribute(&mlir_context, &wasm_intrinsics, Type::I32);
/// ```
pub fn type_to_mlir_zero_attribute<'c>(
    context: &'c MLIRContext,
    intrinsics: &WASMIntrinsics<'c>,
    ty: Type,
) -> Attribute<'c> {
    match ty {
        Type::I32 => IntegerAttribute::new(intrinsics.i32_ty, 0).into(),
        Type::I64 => IntegerAttribute::new(intrinsics.i64_ty, 0).into(),
        Type::F32 => FloatAttribute::new(context, intrinsics.f32_ty, 0.0).into(),
        Type::F64 => FloatAttribute::new(context, intrinsics.f64_ty, 0.0).into(),
        Type::V128 => IntegerAttribute::new(intrinsics.i128_ty, 0).into(),
        Type::FuncRef => TypeAttribute::new(intrinsics.func_ref_ty).into(),
        Type::ExternRef => TypeAttribute::new(intrinsics.extern_ref_ty).into(),
    }
}
