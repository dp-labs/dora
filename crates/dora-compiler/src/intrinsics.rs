use crate::context::Context;
use melior::dialect::llvm::r#type::{pointer, void};
use melior::ir::{
    r#type::{IntegerType, Type},
    Location,
};
use melior::Context as MLIRContext;

/// The `Intrinsics` struct defines a collection of fundamental types and locations used
/// within an MLIR context. These intrinsic types are commonly used in MLIR operations and
/// represent various integer, floating-point, and other low-level types, as well as a generic
/// location for operations.
///
/// This struct helps ensure consistency across the codebase by providing a single source
/// for intrinsic types.
#[derive(Debug)]
pub struct Intrinsics<'ctx> {
    pub i1_ty: Type<'ctx>,
    pub i2_ty: Type<'ctx>,
    pub i4_ty: Type<'ctx>,
    pub i8_ty: Type<'ctx>,
    pub i16_ty: Type<'ctx>,
    pub i32_ty: Type<'ctx>,
    pub i64_ty: Type<'ctx>,
    pub i128_ty: Type<'ctx>,
    pub i256_ty: Type<'ctx>,
    pub i257_ty: Type<'ctx>,
    pub isize_ty: Type<'ctx>,
    pub f32_ty: Type<'ctx>,
    pub f64_ty: Type<'ctx>,
    pub index_ty: Type<'ctx>,
    pub ptr_ty: Type<'ctx>,
    pub void_ty: Type<'ctx>,
    pub unknown_loc: Location<'ctx>,
}

impl<'ctx> Intrinsics<'ctx> {
    /// Create an [`Intrinsics`] for the given MLIR [`Context`].
    pub fn declare(context: &'ctx Context) -> Self {
        Self::declare_with_mlir_context(&context.mlir_context)
    }

    /// Create an [`Intrinsics`] for the given MLIR [`Context`].
    pub fn declare_with_mlir_context(context: &'ctx MLIRContext) -> Self {
        let i1_ty = IntegerType::new(context, 1).into();
        let i2_ty = IntegerType::new(context, 2).into();
        let i4_ty = IntegerType::new(context, 4).into();
        let i8_ty = IntegerType::new(context, 8).into();
        let i16_ty = IntegerType::new(context, 16).into();
        let i32_ty = IntegerType::new(context, 32).into();
        let i64_ty = IntegerType::new(context, 64).into();
        let i128_ty = IntegerType::new(context, 128).into();
        let i256_ty = IntegerType::new(context, 256).into();
        let i257_ty = IntegerType::new(context, 257).into();
        let isize_ty = IntegerType::new(context, std::mem::size_of::<isize>() as u32 * 8).into();
        let f32_ty = Type::float32(context);
        let f64_ty = Type::float64(context);
        let index_ty = Type::index(context);
        let ptr_ty = pointer(context, 0);
        let void_ty = void(context);
        let unknown_loc = Location::unknown(context);
        Self {
            i1_ty,
            i2_ty,
            i4_ty,
            i8_ty,
            i16_ty,
            i32_ty,
            i64_ty,
            i128_ty,
            i256_ty,
            i257_ty,
            isize_ty,
            f32_ty,
            f64_ty,
            index_ty,
            ptr_ty,
            void_ty,
            unknown_loc,
        }
    }
}

pub fn is_f32_arithmetic(bits: u32) -> bool {
    // Mask off sign bit.
    let bits = bits & 0x7FFF_FFFF;
    bits < 0x7FC0_0000
}

pub fn is_f64_arithmetic(bits: u64) -> bool {
    // Mask off sign bit.
    let bits = bits & 0x7FFF_FFFF_FFFF_FFFF;
    bits < 0x7FF8_0000_0000_0000
}
