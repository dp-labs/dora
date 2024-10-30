use crate::context::Context;

use super::intrinsics::WASMIntrinsics;
use melior::{
    dialect::llvm::r#type::r#struct,
    ir::{
        attribute::{FloatAttribute, IntegerAttribute, TypeAttribute},
        Attribute,
    },
    Context as MLIRContext,
};
use wasmer_types::{FunctionType, Type};

/// Returns the MLIR pointer type for a given WebAssembly type.
///
/// This function takes a WebAssembly type (`_ty`) and returns a corresponding
/// MLIR pointer type using the provided `WASMIntrinsics`.
///
/// # Parameters
///
/// - `intrinsics`: The WebAssembly intrinsics that provide predefined MLIR types.
/// - `_ty`: The WebAssembly type to be mapped to an MLIR pointer type.
///
/// # Returns
///
/// - `melior::ir::Type<'c>`: The corresponding MLIR pointer type for the given WebAssembly type.
///
/// # Example
///
/// ```no_check
/// let ptr_type = type_to_mlir_ptr(&wasm_intrinsics, Type::I32);
/// ```
#[inline]
pub fn type_to_mlir_ptr<'c>(intrinsics: &WASMIntrinsics<'c>, _ty: Type) -> melior::ir::Type<'c> {
    intrinsics.ptr_ty
}

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
    let user_param_types = sig.params().iter().map(|&ty| type_to_mlir(intrinsics, ty));
    let param_types = std::iter::once(intrinsics.ctx_ptr_ty).chain(user_param_types);
    let sig_returns_bit_widths = sig
        .results()
        .iter()
        .map(|ty| match ty {
            Type::I32 | Type::F32 => 32,
            Type::I64 | Type::F64 => 64,
            Type::V128 => 128,
            Type::ExternRef | Type::FuncRef => 64, /* pointer */
        })
        .collect::<Vec<i32>>();
    match sig_returns_bit_widths.as_slice() {
        [] => melior::ir::r#type::FunctionType::new(
            &context.mlir_context,
            &param_types.collect::<Vec<melior::ir::Type>>(),
            &[intrinsics.void_ty],
        ),
        [_] => {
            let single_value = sig.results()[0];
            melior::ir::r#type::FunctionType::new(
                &context.mlir_context,
                &param_types.collect::<Vec<melior::ir::Type>>(),
                &[type_to_mlir(intrinsics, single_value)],
            )
        }
        [32, 64] | [64, 32] | [64, 64] => {
            let basic_types: Vec<_> = sig
                .results()
                .iter()
                .map(|&ty| type_to_mlir(intrinsics, ty))
                .collect::<_>();
            let result_ty = r#struct(&context.mlir_context, &basic_types, false);
            melior::ir::r#type::FunctionType::new(
                &context.mlir_context,
                &param_types.collect::<Vec<melior::ir::Type>>(),
                &[result_ty],
            )
        }
        [32, 32] if sig.results()[0] == Type::F32 && sig.results()[1] == Type::F32 => {
            melior::ir::r#type::FunctionType::new(
                &context.mlir_context,
                &param_types.collect::<Vec<melior::ir::Type>>(),
                &[intrinsics.f32_ty, intrinsics.f32_ty],
            )
        }
        [32, 32] => melior::ir::r#type::FunctionType::new(
            &context.mlir_context,
            &param_types.collect::<Vec<melior::ir::Type>>(),
            &[intrinsics.f64_ty],
        ),
        [32, 32, _] if sig.results()[0] == Type::F32 && sig.results()[1] == Type::F32 => {
            melior::ir::r#type::FunctionType::new(
                &context.mlir_context,
                &param_types.collect::<Vec<melior::ir::Type>>(),
                &[
                    intrinsics.f32_ty,
                    intrinsics.f32_ty,
                    type_to_mlir(intrinsics, sig.results()[2]),
                ],
            )
        }
        [32, 32, _] => melior::ir::r#type::FunctionType::new(
            &context.mlir_context,
            &param_types.collect::<Vec<melior::ir::Type>>(),
            &[
                intrinsics.i64_ty,
                type_to_mlir(intrinsics, sig.results()[2]),
            ],
        ),
        [64, 32, 32] if sig.results()[1] == Type::F32 && sig.results()[2] == Type::F32 => {
            melior::ir::r#type::FunctionType::new(
                &context.mlir_context,
                &param_types.collect::<Vec<melior::ir::Type>>(),
                &[
                    type_to_mlir(intrinsics, sig.results()[0]),
                    intrinsics.f32_ty,
                    intrinsics.f32_ty,
                ],
            )
        }
        [64, 32, 32] => melior::ir::r#type::FunctionType::new(
            &context.mlir_context,
            &param_types.collect::<Vec<melior::ir::Type>>(),
            &[
                type_to_mlir(intrinsics, sig.results()[0]),
                intrinsics.i64_ty,
            ],
        ),
        [32, 32, 32, 32] => melior::ir::r#type::FunctionType::new(
            &context.mlir_context,
            &param_types.collect::<Vec<melior::ir::Type>>(),
            &[
                if sig.results()[0] == Type::F32 && sig.results()[1] == Type::F32 {
                    intrinsics.f64_ty
                } else {
                    intrinsics.i64_ty
                },
                if sig.results()[2] == Type::F32 && sig.results()[3] == Type::F32 {
                    intrinsics.f64_ty
                } else {
                    intrinsics.i64_ty
                },
            ],
        ),
        _ => {
            let sret_ptr = intrinsics.ptr_ty;
            let param_types = std::iter::once(sret_ptr).chain(param_types);
            melior::ir::r#type::FunctionType::new(
                &context.mlir_context,
                &param_types.collect::<Vec<melior::ir::Type>>(),
                &[intrinsics.void_ty],
            )
        }
    }
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
pub fn type_to_mlir<'c>(intrinsics: &WASMIntrinsics<'c>, ty: Type) -> melior::ir::r#type::Type<'c> {
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
