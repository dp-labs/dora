use crate::backend::{AtomicOrdering, IntCC, LoadStoreOptions, TypeMethods};
use crate::errors::{CompileError, Result};
use crate::intrinsics::Intrinsics;
use crate::value::{IntoContextOperation, ToContextValue};
use anyhow::bail;
use dora_ir::IRTypes;
use melior::Context as MLIRContext;
use melior::dialect::arith::CmpiPredicate;
use melior::dialect::llvm::AllocaOptions;
use melior::dialect::llvm::attributes::Linkage;
use melior::dialect::llvm::r#type::r#struct;
use melior::dialect::{arith, func, llvm};
use melior::ir::attribute::{
    DenseI32ArrayAttribute, FlatSymbolRefAttribute, FloatAttribute, IntegerAttribute,
    StringAttribute, TypeAttribute,
};
use melior::ir::operation::{OperationBuilder, OperationRefMut};
use melior::ir::r#type::IntegerType;
use melior::ir::{
    Attribute, BlockRef, Identifier, Location, Operation, OperationRef, Region, Type, TypeLike,
    Value, ValueLike,
};
use mlir_sys::{mlirIntegerTypeGetWidth, mlirOperationGetBlock, mlirOperationGetLocation};
use num_bigint::{BigInt, BigUint};

/// The `OpBuilder` struct is responsible for constructing operations within an MLIR context.
/// It manages the current insertion point within a block, allowing the user to define and insert
/// MLIR operations programmatically. The `OpBuilder` provides a flexible API for generating
/// MLIR code, with access to predefined intrinsics and contextual information.
///
/// # Fields:
/// - `ctx`: A reference to the MLIR context (`MLIRContext`). This context holds the global state
///   required for managing types, operations, and other resources in MLIR.
/// - `intrinsics`: Provides access to a set of predefined intrinsic types (`Intrinsics`). These types
///   represent common data types and locations used in MLIR operations.
/// - `block`: An optional `BlockRef` representing the current block in which the operations will be
///   inserted. If present, operations will be created and inserted into this block.
/// - `insert_point`: An optional `OperationRef` representing the current insertion point within
///   the block. New operations will be inserted after this point, if specified.
///
/// # Example Usage:
/// ```no_check
/// let builder = OpBuilder::new(&mlir_context);
/// // Use the builder to insert operations into the specified block
/// ```
///
/// # Purpose:
/// The `OpBuilder` struct is designed to simplify the creation and insertion of MLIR operations.
/// It provides essential contextual information (`ctx`, `intrinsics`) and manages the current insertion
/// point within a block of operations. This allows for flexible and controlled operation construction
/// in an MLIR-based environment.
///
/// # Fields:
/// - `ctx`: A reference to the MLIR context, which holds global state and is required for constructing
///   operations and managing types.
/// - `intrinsics`: A collection of intrinsic types and locations used for constructing common operations
///   in MLIR, such as integer types, floating-point types, and index types.
/// - `block`: An optional reference to a block within which new operations will be inserted. If set,
///   the block determines the scope of operation insertion.
/// - `insert_point`: An optional reference to the current insertion point in the block. This allows
///   operations to be inserted at a specific point within the block.
///
/// # Notes:
/// - The `block` field is optional and, if not set, the `OpBuilder` may be used to construct operations
///   without immediate insertion into a block.
/// - The `insert_point` field determines where new operations are inserted within the block, if provided.
///   If not set, operations may be appended to the end of the block.
#[derive(Debug)]
pub struct OpBuilder<'c, 'a> {
    /// A reference to the MLIR context, which manages global state and resources required for MLIR operations.
    pub ctx: &'c MLIRContext,

    /// A collection of intrinsic types used for constructing common operations in MLIR.
    pub intrinsics: Intrinsics<'c>,

    /// An optional reference to the current block where operations are being inserted.
    /// If set, the block represents the scope of the operation insertion.
    block: Option<BlockRef<'c, 'a>>,

    /// An optional reference to the current insertion point within the block.
    /// If set, new operations will be inserted after this point.
    insert_point: Option<OperationRef<'c, 'a>>,
}

impl<'c, 'a> OpBuilder<'c, 'a> {
    /// Creates a new `OpBuilder` without an insert point or block set.
    ///
    /// This builder can be used to create MLIR operations, but operations
    /// won't be inserted into any block until an insert point or block is set.
    ///
    /// # Parameters
    /// - `ctx`: A reference to the MLIR context.
    ///
    /// # Returns
    /// A new `OpBuilder` instance.
    pub fn new(ctx: &'c MLIRContext) -> Self {
        Self {
            ctx,
            intrinsics: Intrinsics::declare_with_mlir_context(ctx),
            insert_point: None,
            block: None,
        }
    }

    /// Creates a new `OpBuilder` and sets the insertion point to the end of the specified block.
    ///
    /// Any operations created by this builder will be inserted at the end of the given block.
    ///
    /// # Parameters
    /// - `ctx`: A reference to the MLIR context.
    /// - `block`: A reference to the block where new operations will be inserted.
    ///
    /// # Returns
    /// A new `OpBuilder` instance with the insertion point set to the end of the provided block.
    pub fn new_with_block(ctx: &'c MLIRContext, block: BlockRef<'c, 'a>) -> Self {
        Self {
            ctx,
            intrinsics: Intrinsics::declare_with_mlir_context(ctx),
            insert_point: None,
            block: Some(block),
        }
    }

    /// Creates a new `OpBuilder` and sets the insertion point to right before the specified operation.
    ///
    /// Any operations created by this builder will be inserted immediately before the specified operation.
    ///
    /// # Parameters
    /// - `ctx`: A reference to the MLIR context.
    /// - `op`: A reference to the operation before which new operations will be inserted.
    ///
    /// # Returns
    /// A new `OpBuilder` instance with the insertion point set to before the specified operation.
    pub fn new_with_op(ctx: &'c MLIRContext, op: OperationRef<'c, 'a>) -> Self {
        let op = op.to_raw();
        let block = unsafe { BlockRef::from_option_raw(mlirOperationGetBlock(op)) };
        Self {
            ctx,
            intrinsics: Intrinsics::declare_with_mlir_context(ctx),
            insert_point: unsafe { Some(OperationRef::from_raw(op)) },
            block,
        }
    }

    /// Creates a new `OpBuilder` and sets the insertion point to right before the specified mutable operation.
    ///
    /// Any operations created by this builder will be inserted immediately before the specified mutable operation.
    ///
    /// # Parameters
    /// - `ctx`: A reference to the MLIR context.
    /// - `op`: A mutable reference to the operation before which new operations will be inserted.
    ///
    /// # Returns
    /// A new `OpBuilder` instance with the insertion point set to before the specified mutable operation.
    pub fn new_with_op_mut(ctx: &'c MLIRContext, op: OperationRefMut<'c, 'a>) -> Self {
        let op = op.to_raw();
        let block = unsafe { BlockRef::from_option_raw(mlirOperationGetBlock(op)) };
        Self {
            ctx,
            intrinsics: Intrinsics::declare_with_mlir_context(ctx),
            insert_point: unsafe { Some(OperationRef::from_raw(op)) },
            block,
        }
    }
}

impl<'c> IRTypes for OpBuilder<'c, '_> {
    type Type = Type<'c>;
    type Value = Value<'c, 'c>;
    type Region = Region<'c>;
    type BasicBlock = BlockRef<'c, 'c>;
    type Operation = Operation<'c>;
}

impl<'c, 'a> TypeMethods for OpBuilder<'c, 'a> {
    fn type_ptr(&self) -> Ty<'c, 'a> {
        self.intrinsics.ptr_ty
    }

    fn type_int(&self, bits: u32) -> Ty<'c, 'a> {
        IntegerType::new(self.ctx, bits).into()
    }
}

type Op<'c, 'a> = <OpBuilder<'c, 'a> as IRTypes>::Operation;
type Ty<'c, 'a> = <OpBuilder<'c, 'a> as IRTypes>::Type;
type Val<'c, 'a> = <OpBuilder<'c, 'a> as IRTypes>::Value;

impl<'c, 'a> OpBuilder<'c, 'a> {
    /// Returns a reference to the MLIR context associated with the builder.
    #[inline]
    pub fn context(&self) -> &'c MLIRContext {
        self.ctx
    }

    /// Returns the `boolean` type intrinsic, representing a 1-bit unsigned integer.
    #[inline]
    pub fn i1_ty(&self) -> Type<'c> {
        self.intrinsics.i1_ty
    }

    /// Returns the `i2` type intrinsic, representing a 2-bit unsigned integer.
    #[inline]
    pub fn i2_ty(&self) -> Type<'c> {
        self.intrinsics.i2_ty
    }

    /// Returns the `i4` type intrinsic, representing a 4-bit unsigned integer.
    #[inline]
    pub fn i4_ty(&self) -> Type<'c> {
        self.intrinsics.i4_ty
    }

    /// Returns the `i8` integer type intrinsic, representing a 8-bit unsigned integer.
    #[inline]
    pub fn i8_ty(&self) -> Type<'c> {
        self.intrinsics.i8_ty
    }

    /// Returns the `i16` integer type intrinsic, representing a 16-bit unsigned integer.
    #[inline]
    pub fn i16_ty(&self) -> Type<'c> {
        self.intrinsics.i16_ty
    }

    /// Returns the `i32` integer type intrinsic, representing a 32-bit unsigned integer.
    #[inline]
    pub fn i32_ty(&self) -> Type<'c> {
        self.intrinsics.i32_ty
    }

    /// Returns the `i64` integer type intrinsic, representing a 64-bit unsigned integer.
    #[inline]
    pub fn i64_ty(&self) -> Type<'c> {
        self.intrinsics.i64_ty
    }

    /// Returns the `i128` integer type intrinsic, representing a 128-bit unsigned integer.
    #[inline]
    pub fn i128_ty(&self) -> Type<'c> {
        self.intrinsics.i128_ty
    }

    /// Returns the `i256` integer type intrinsic, representing a 256-bit unsigned integer.
    #[inline]
    pub fn i256_ty(&self) -> Type<'c> {
        self.intrinsics.i256_ty
    }

    /// Returns the `i257` integer type intrinsic, representing a 257-bit unsigned integer.
    #[inline]
    pub fn i257_ty(&self) -> Type<'c> {
        self.intrinsics.i257_ty
    }

    /// Returns the index type intrinsic with the fixed 64-bit widih.
    #[inline]
    pub fn index_ty(&self) -> Type<'c> {
        self.intrinsics.index_ty
    }

    /// Returns the isize type intrinsic (The pointer-sized signed integer type).
    /// The size of this primitive is how many bytes it takes to reference any location in memory. For example, on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes.
    #[inline]
    pub fn usize_ty(&self) -> Type<'c> {
        self.intrinsics.isize_ty
    }

    /// Returns the isize type intrinsic (The pointer-sized signed integer type).
    /// The size of this primitive is how many bytes it takes to reference any location in memory. For example, on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes.
    #[inline]
    pub fn isize_ty(&self) -> Type<'c> {
        self.intrinsics.isize_ty
    }

    /// Returns the f32 type intrinsic.
    #[inline]
    pub fn f32_ty(&self) -> Type<'c> {
        self.intrinsics.f32_ty
    }

    /// Returns the f64 type intrinsic.
    #[inline]
    pub fn f64_ty(&self) -> Type<'c> {
        self.intrinsics.f64_ty
    }

    /// Returns the pointer type intrinsic.
    #[inline]
    pub fn ptr_ty(&self) -> Type<'c> {
        self.intrinsics.ptr_ty
    }

    /// Returns the unknown location intrinsic, used when no specific source location is available.
    #[inline]
    pub fn unknown_loc(&self) -> Location<'c> {
        self.intrinsics.unknown_loc
    }

    /// Returns the width of the specified integer type. If the type is not an integer, an error is returned.
    ///
    /// # Parameters
    /// - `ty`: The integer type for which to retrieve the width.
    ///
    /// # Returns
    /// The width of the integer type as a `Result<u32>`.
    pub fn int_ty_width(&self, ty: Type<'c>) -> Result<u32> {
        if ty.is_integer() {
            Ok(unsafe { mlirIntegerTypeGetWidth(ty.to_raw()) } as u32)
        } else {
            bail!(CompileError::Codegen(
                "type is not a integer type".to_string(),
            ))
        }
    }

    /// Returns the specified integer type with bits.
    #[inline]
    pub fn int_ty(&self, bits: u32) -> Ty<'c, '_> {
        IntegerType::new(self.context(), bits).into()
    }

    /// Creates an `i32` integer attribute with the specified value.
    ///
    /// # Parameters
    /// - `value`: The `i32` value to wrap in an attribute.
    ///
    /// # Returns
    /// A new integer attribute of type `i32`.
    #[inline]
    pub fn i32_attr(&self, value: i32) -> Attribute {
        IntegerAttribute::new(self.intrinsics.i32_ty, value as i64).into()
    }

    /// Creates an `i64` integer attribute with the specified value.
    ///
    /// # Parameters
    /// - `value`: The `i64` value to wrap in an attribute.
    ///
    /// # Returns
    /// A new integer attribute of type `i64`.
    #[inline]
    pub fn i64_attr(&self, value: i64) -> Attribute {
        IntegerAttribute::new(self.intrinsics.i64_ty, value).into()
    }

    /// Creates an `f32` float attribute with the specified value.
    ///
    /// # Parameters
    /// - `value`: The `f32` value to wrap in an attribute.
    ///
    /// # Returns
    /// A new float attribute of type `f32`.
    #[inline]
    pub fn f32_attr(&self, value: f32) -> Attribute {
        FloatAttribute::new(self.context(), self.intrinsics.f32_ty, value as f64).into()
    }

    /// Creates an `f64` float attribute with the specified value.
    ///
    /// # Parameters
    /// - `value`: The `f64` value to wrap in an attribute.
    ///
    /// # Returns
    /// A new float attribute of type `f64`.
    #[inline]
    pub fn f64_attr(&self, value: f64) -> Attribute {
        FloatAttribute::new(self.context(), self.intrinsics.i64_ty, value).into()
    }

    /// Creates a string attribute with the specified string value.
    ///
    /// # Parameters
    /// - `val`: The string value to wrap in an attribute.
    ///
    /// # Returns
    /// A new string attribute.
    #[inline]
    pub fn str_attr(&self, val: &str) -> StringAttribute {
        StringAttribute::new(self.context(), val)
    }

    /// Creates a boolean constant operation with the specified value.
    ///
    /// # Parameters
    /// - `value`: The boolean value to use (`true` or `false`).
    ///
    /// # Returns
    /// A boolean constant operation.
    #[inline]
    pub fn bool_const(&self, value: bool) -> Op<'c, '_> {
        let ty = self.intrinsics.i1_ty;
        arith::constant(
            self.context(),
            IntegerAttribute::new(ty, if value { 1 } else { 0 }).into(),
            self.intrinsics.unknown_loc,
        )
    }

    /// Creates a index constant operation with the specified value.
    ///
    /// # Parameters
    /// - `value`: The index value.
    ///
    /// # Returns
    /// An integer constant operation of type `i64`.
    #[inline]
    pub fn index(&self, value: usize) -> Op<'c, '_> {
        self.iconst(self.intrinsics.index_ty, value as i64)
    }

    /// Creates an integer constant operation with the specified type and value.
    ///
    /// # Parameters
    /// - `ty`: The type of the integer.
    /// - `value`: The integer value.
    ///
    /// # Returns
    /// An integer constant operation.
    #[inline]
    pub fn iconst(&self, ty: Ty<'c, 'a>, value: i64) -> Op<'c, '_> {
        arith::constant(
            self.context(),
            IntegerAttribute::new(ty, value).into(),
            self.intrinsics.unknown_loc,
        )
    }

    /// Creates an integer constant operation with the specified type and biguint value.
    ///
    /// # Parameters
    /// - `ty`: The type of the integer.
    /// - `value`: The integer value.
    ///
    /// # Returns
    /// An integer constant operation.
    #[inline]
    pub fn iconst_biguint(&self, ty: Ty<'c, 'a>, value: BigUint) -> Result<Op<'c, '_>> {
        Ok(arith::constant(
            self.context(),
            Attribute::parse(self.context(), &format!("{value} : {ty}")).ok_or(
                CompileError::Codegen(format!("can't parse value {value} to {ty}")),
            )?,
            self.intrinsics.unknown_loc,
        ))
    }

    /// Creates an integer constant operation with the specified type and bigint value.
    ///
    /// # Parameters
    /// - `ty`: The type of the integer.
    /// - `value`: The integer value.
    ///
    /// # Returns
    /// An integer constant operation.
    #[inline]
    pub fn iconst_bigint(&self, ty: Ty<'c, 'a>, value: BigInt) -> Result<Op<'c, '_>> {
        Ok(arith::constant(
            self.context(),
            Attribute::parse(self.context(), &format!("{value} : {ty}")).ok_or(
                CompileError::Codegen(format!("can't parse value {value} to {ty}")),
            )?,
            self.intrinsics.unknown_loc,
        ))
    }

    /// Creates an unsigned integer constant operation with the specified type and value.
    ///
    /// # Parameters
    /// - `ty`: The type of the integer.
    /// - `value`: The unsigned integer value.
    ///
    /// # Returns
    /// An unsigned integer constant operation.
    #[inline]
    pub fn uconst(&self, ty: Ty<'c, 'a>, value: u64) -> Op<'c, '_> {
        arith::constant(
            self.context(),
            IntegerAttribute::new(ty, value as i64).into(),
            self.intrinsics.unknown_loc,
        )
    }

    /// Creates an 8-bit integer constant operation with the specified value.
    ///
    /// # Parameters
    /// - `value`: The 8-bit integer value.
    ///
    /// # Returns
    /// An integer constant operation of type `i8`.
    #[inline]
    pub fn iconst_8(&self, value: i8) -> Op<'c, '_> {
        self.iconst(self.intrinsics.i8_ty, value as i64)
    }

    /// Creates an 16-bit integer constant operation with the specified value.
    ///
    /// # Parameters
    /// - `value`: The 16-bit integer value.
    ///
    /// # Returns
    /// An integer constant operation of type `i16`.
    #[inline]
    pub fn iconst_16(&self, value: i16) -> Op<'c, '_> {
        self.iconst(self.intrinsics.i16_ty, value as i64)
    }

    /// Creates a 32-bit integer constant operation with the specified value.
    ///
    /// # Parameters
    /// - `value`: The 32-bit integer value.
    ///
    /// # Returns
    /// An integer constant operation of type `i32`.
    #[inline]
    pub fn iconst_32(&self, value: i32) -> Op<'c, '_> {
        self.iconst(self.intrinsics.i32_ty, value as i64)
    }

    /// Creates a 64-bit integer constant operation with the specified value.
    ///
    /// # Parameters
    /// - `value`: The 64-bit integer value.
    ///
    /// # Returns
    /// An integer constant operation of type `i64`.
    #[inline]
    pub fn iconst_64(&self, value: i64) -> Op<'c, '_> {
        self.iconst(self.intrinsics.i64_ty, value)
    }

    /// Creates a 256-bit integer constant operation with the specified `BigUint` value.
    ///
    /// # Parameters
    /// - `value`: The 256-bit integer value.
    ///
    /// # Returns
    /// A result containing a 256-bit integer constant operation or an error.
    #[inline]
    pub fn iconst_256(&self, value: BigUint) -> Result<Op<'c, '_>> {
        Ok(arith::constant(
            self.context(),
            Attribute::parse(self.context(), &format!("{} : i256", value)).ok_or(
                CompileError::Codegen(format!("can't parse value {value} to i256")),
            )?,
            self.intrinsics.unknown_loc,
        ))
    }

    /// Creates a 256-bit integer minimum constant operation with the specified `BigUint` value.
    ///
    /// # Returns
    /// A result containing a minimum 256-bit integer constant operation or an error.
    #[inline]
    pub fn iconst_256_min(&self) -> Result<Op<'c, '_>> {
        Ok(arith::constant(
            self.context(),
            Attribute::parse(self.context(), "-57896044618658097711785492504343953926634992332820282019728792003956564819968 : i256").ok_or(
                CompileError::Codegen("can't parse the i256 min value".to_string()),
            )?,
            self.intrinsics.unknown_loc,
        ))
    }

    /// Creates an 1-bit integer constant operation with the specified value.
    ///
    /// # Parameters
    /// - `value`: The bool value.
    ///
    /// # Returns
    /// An integer constant operation of type `ii`.
    #[inline]
    pub fn iconst_1_from_bool(&self, value: bool) -> Op<'c, '_> {
        self.iconst(self.intrinsics.i1_ty, if value { 1 } else { 0 })
    }

    /// Creates a 256-bit integer constant operation from a `u64` value.
    ///
    /// # Parameters
    /// - `value`: The `u64` value to convert.
    ///
    /// # Returns
    /// A result containing a 256-bit integer constant operation or an error.
    #[inline]
    pub fn iconst_256_from_u64(&self, value: u64) -> Result<Op<'c, '_>> {
        self.iconst_256(BigUint::from(value))
    }

    /// Creates a floating-point constant operation with the specified type and value.
    ///
    /// # Parameters
    /// - `ty`: The type of the floating-point number.
    /// - `value`: The floating-point value.
    ///
    /// # Returns
    /// A floating-point constant operation.
    #[inline]
    pub fn fconst(&self, ty: Ty<'c, 'a>, value: f64) -> Op<'c, '_> {
        arith::constant(
            self.context(),
            FloatAttribute::new(self.context(), ty, value).into(),
            self.intrinsics.unknown_loc,
        )
    }

    /// Creates a 32-bit floating-point constant operation with the specified value.
    ///
    /// # Parameters
    /// - `value`: The 32-bit floating-point value.
    ///
    /// # Returns
    /// A floating-point constant operation of type `f32`.
    #[inline]
    pub fn fconst_32(&self, value: f32) -> Op<'c, '_> {
        self.fconst(self.intrinsics.f32_ty, value as f64)
    }

    /// Creates a 64-bit floating-point constant operation with the specified value.
    ///
    /// # Parameters
    /// - `value`: The 64-bit floating-point value.
    ///
    /// # Returns
    /// A floating-point constant operation of type `f64`.
    #[inline]
    pub fn fconst_64(&self, value: f64) -> Op<'c, '_> {
        self.fconst(self.intrinsics.f64_ty, value)
    }

    /// Creates an LLVM struct type.
    /// # Parameters
    /// - `fields`: The type array in the struct type.
    ///
    /// # Returns
    /// A LLVM struct type.
    #[inline]
    pub fn struct_ty(&self, fields: &[Ty<'c, '_>]) -> Ty<'c, '_> {
        r#struct(self.context(), fields, false)
    }

    /// Returns an operation that represents the return of values from the current function.
    ///
    /// # Parameters
    /// - `values`: A slice of values to return from the function.
    ///
    /// # Returns
    /// An operation representing the return statement.
    #[inline]
    pub fn ret(&mut self, values: &[Val<'c, 'a>]) -> Op<'c, '_> {
        func::r#return(values, self.intrinsics.unknown_loc)
    }

    /// Compares two integer values based on the specified condition.
    ///
    /// # Parameters
    /// - `cond`: The comparison condition to apply.
    /// - `lhs`: The left-hand side value for the comparison.
    /// - `rhs`: The right-hand side value for the comparison.
    ///
    /// # Returns
    /// An operation representing the result of the integer comparison.
    pub fn icmp(
        &self,
        cond: crate::backend::IntCC,
        lhs: Val<'c, 'a>,
        rhs: Val<'c, 'a>,
    ) -> Op<'c, 'a> {
        let pred = match cond {
            IntCC::Equal => CmpiPredicate::Eq,
            IntCC::NotEqual => CmpiPredicate::Ne,
            IntCC::SignedGreaterThan => CmpiPredicate::Sgt,
            IntCC::SignedGreaterThanOrEqual => CmpiPredicate::Sge,
            IntCC::SignedLessThan => CmpiPredicate::Slt,
            IntCC::SignedLessThanOrEqual => CmpiPredicate::Sle,
            IntCC::UnsignedGreaterThan => CmpiPredicate::Ugt,
            IntCC::UnsignedGreaterThanOrEqual => CmpiPredicate::Uge,
            IntCC::UnsignedLessThan => CmpiPredicate::Ult,
            IntCC::UnsignedLessThanOrEqual => CmpiPredicate::Ule,
        };
        arith::cmpi(self.context(), pred, lhs, rhs, self.intrinsics.unknown_loc)
    }

    /// Compares an integer value against an immediate value based on the specified condition.
    ///
    /// # Parameters
    /// - `cond`: The comparison condition to apply.
    /// - `lhs`: The left-hand side value for the comparison.
    /// - `rhs`: The immediate integer value to compare against.
    ///
    /// # Returns
    /// A result containing an operation representing the result of the integer comparison.
    pub fn icmp_imm(
        &self,
        cond: crate::backend::IntCC,
        lhs: Val<'c, 'a>,
        rhs: i64,
    ) -> Result<Op<'c, 'a>> {
        let ty = lhs.r#type();
        let rhs = self.create(self.iconst(ty, rhs));
        Ok(self.icmp(cond, lhs, rhs.result(0)?.to_ctx_value()))
    }

    /// Check whether the pointer value is an null pointer
    pub fn is_null(&self, ptr: Val<'c, 'a>) -> Result<Op<'c, 'a>> {
        let ptr_to_int_op = OperationBuilder::new("llvm.ptrtoint", self.intrinsics.unknown_loc)
            .add_results(&[self.isize_ty()])
            .add_operands(&[ptr])
            .build()?;
        let ptr_int_val = self.make(ptr_to_int_op)?.to_ctx_value();
        self.icmp_imm(IntCC::Equal, ptr_int_val, 0)
    }

    /// Loads a value from the specified pointer.
    ///
    /// # Parameters
    /// - `ptr`: The pointer from which to load the value.
    /// - `ty`: The type of the value to load.
    ///
    /// # Returns
    /// An operation representing the load operation.
    #[inline]
    pub fn load(&self, ptr: Val<'c, 'a>, ty: Ty<'c, 'a>) -> Op<'c, '_> {
        llvm::load(
            self.context(),
            ptr,
            ty,
            self.intrinsics.unknown_loc,
            llvm::LoadStoreOptions::default(),
        )
    }

    /// Loads a value from the specified pointer with alignment.
    ///
    /// # Parameters
    /// - `ptr`: The pointer from which to load the value.
    /// - `ty`: The type of the value to load.
    /// - `align`: Sets an alignment.
    ///
    /// # Returns
    /// An operation representing the load operation.
    #[inline]
    pub fn load_with_align(&self, ptr: Val<'c, 'a>, ty: Ty<'c, 'a>, align: u64) -> Op<'c, '_> {
        llvm::load(
            self.context(),
            ptr,
            ty,
            self.intrinsics.unknown_loc,
            llvm::LoadStoreOptions::default()
                .align(Some(IntegerAttribute::new(self.i64_ty(), align as i64))),
        )
    }

    /// Loads a value from the specified pointer with alignment and set it volatile.
    ///
    /// # Parameters
    /// - `ptr`: The pointer from which to load the value.
    /// - `ty`: The type of the value to load.
    /// - `align`: Sets an alignment.
    /// - `volatile`: Whether the load/store operation is volatile.
    ///
    /// # Returns
    /// An operation representing the load operation.
    #[inline]
    pub fn load_with_align_and_volatile(
        &self,
        ptr: Val<'c, 'a>,
        ty: Ty<'c, 'a>,
        align: u64,
        volatile: bool,
    ) -> Op<'c, '_> {
        llvm::load(
            self.context(),
            ptr,
            ty,
            self.intrinsics.unknown_loc,
            llvm::LoadStoreOptions::default()
                .align(Some(IntegerAttribute::new(self.i64_ty(), align as i64)))
                .volatile(volatile),
        )
    }

    /// Loads a value from the specified pointer with the atomic ordering option.
    ///
    /// # Parameters
    /// - `ptr`: The pointer from which to load the value.
    /// - `ty`: The type of the value to load.
    /// - `ordering`: The atomic order of the load op.
    ///
    /// # Returns
    /// An operation representing the load operation.
    pub fn load_with_ordering(
        &self,
        ptr: Val<'c, 'a>,
        ty: Ty<'c, 'a>,
        ordering: AtomicOrdering,
    ) -> Op<'c, '_> {
        let mut load_op = llvm::load(
            self.context(),
            ptr,
            ty,
            self.intrinsics.unknown_loc,
            llvm::LoadStoreOptions::default(),
        );
        load_op.set_attribute(
            "ordering",
            StringAttribute::new(self.context(), &ordering.attr_str()).into(),
        );
        load_op
    }

    /// Loads a value from the specified pointer with the load operaion options.
    ///
    /// # Parameters
    /// - `ptr`: The pointer from which to load the value.
    /// - `ty`: The type of the value to load.
    /// - `opts`: The load operation options.
    ///
    /// # Returns
    /// An operation representing the load operation.
    pub fn load_with_opts(
        &self,
        ptr: Val<'c, 'a>,
        ty: Ty<'c, 'a>,
        opts: LoadStoreOptions,
    ) -> Op<'c, '_> {
        let llvm_load_store_opts = llvm::LoadStoreOptions::default();
        if opts.nontemporal {
            llvm_load_store_opts.nontemporal(opts.nontemporal);
        }
        if opts.volatile {
            llvm_load_store_opts.volatile(opts.volatile);
        }
        if let Some(align) = opts.align {
            llvm_load_store_opts.align(Some(IntegerAttribute::new(self.i64_ty(), align as i64)));
        }
        let mut load_op = llvm::load(
            self.context(),
            ptr,
            ty,
            self.intrinsics.unknown_loc,
            llvm_load_store_opts,
        );
        if let Some(ordering) = opts.ordering {
            load_op.set_attribute(
                "ordering",
                StringAttribute::new(self.context(), &ordering.attr_str()).into(),
            );
        }
        load_op
    }

    /// Stores a value at the specified pointer.
    ///
    /// # Parameters
    /// - `value`: The value to store.
    /// - `ptr`: The pointer where the value should be stored.
    ///
    /// # Returns
    /// An operation representing the store operation.
    #[inline]
    pub fn store(&self, value: Val<'c, 'a>, ptr: Val<'c, 'a>) -> Op<'c, '_> {
        llvm::store(
            self.context(),
            value,
            ptr,
            self.intrinsics.unknown_loc,
            llvm::LoadStoreOptions::default(),
        )
    }

    /// Stores a value at the specified pointer with alignment.
    ///
    /// # Parameters
    /// - `value`: The value to store.
    /// - `ptr`: The pointer where the value should be stored.
    /// - `align`: Sets an alignment.
    ///
    /// # Returns
    /// An operation representing the store operation.
    #[inline]
    pub fn store_with_align(&self, value: Val<'c, 'a>, ptr: Val<'c, 'a>, align: u64) -> Op<'c, '_> {
        llvm::store(
            self.context(),
            value,
            ptr,
            self.intrinsics.unknown_loc,
            llvm::LoadStoreOptions::default()
                .align(Some(IntegerAttribute::new(self.i64_ty(), align as i64))),
        )
    }

    /// Stores a value at the specified pointer with alignment and set it volatile.
    ///
    /// # Parameters
    /// - `value`: The value to store.
    /// - `ptr`: The pointer where the value should be stored.
    /// - `align`: Sets an alignment.
    /// - `volatile`: Whether the load/store operation is volatile.
    ///
    /// # Returns
    /// An operation representing the store operation.
    #[inline]
    pub fn store_with_align_and_volatile(
        &self,
        value: Val<'c, 'a>,
        ptr: Val<'c, 'a>,
        align: u64,
        volatile: bool,
    ) -> Op<'c, '_> {
        llvm::store(
            self.context(),
            value,
            ptr,
            self.intrinsics.unknown_loc,
            llvm::LoadStoreOptions::default()
                .align(Some(IntegerAttribute::new(self.i64_ty(), align as i64)))
                .volatile(volatile),
        )
    }

    /// Stores a value at the specified pointer with the atomic ordering option.
    ///
    /// # Parameters
    /// - `value`: The value to store.
    /// - `ptr`: The pointer where the value should be stored.
    /// - `ordering`: The atomic order of the load op.
    ///
    /// # Returns
    /// An operation representing the store operation.
    pub fn store_with_ordering(
        &self,
        value: Val<'c, 'a>,
        ptr: Val<'c, 'a>,
        ordering: AtomicOrdering,
    ) -> Op<'c, '_> {
        let mut store_op = llvm::store(
            self.context(),
            value,
            ptr,
            self.intrinsics.unknown_loc,
            llvm::LoadStoreOptions::default(),
        );
        store_op.set_attribute(
            "ordering",
            StringAttribute::new(self.context(), &ordering.attr_str()).into(),
        );
        store_op
    }

    /// Stores a value at the specified pointer with the store option.
    ///
    /// # Parameters
    /// - `value`: The value to store.
    /// - `ptr`: The pointer where the value should be stored.
    /// - `opts`: The store operation options.
    ///
    /// # Returns
    /// An operation representing the load operation.
    pub fn store_with_opts(
        &self,
        value: Val<'c, 'a>,
        ptr: Val<'c, 'a>,
        opts: LoadStoreOptions,
    ) -> Op<'c, '_> {
        let llvm_load_store_opts = llvm::LoadStoreOptions::default();
        if opts.nontemporal {
            llvm_load_store_opts.nontemporal(opts.nontemporal);
        }
        if opts.volatile {
            llvm_load_store_opts.volatile(opts.volatile);
        }
        if let Some(align) = opts.align {
            llvm_load_store_opts.align(Some(IntegerAttribute::new(self.i64_ty(), align as i64)));
        }
        let mut store_op = llvm::store(
            self.context(),
            value,
            ptr,
            self.intrinsics.unknown_loc,
            llvm_load_store_opts,
        );
        if let Some(ordering) = opts.ordering {
            store_op.set_attribute(
                "ordering",
                StringAttribute::new(self.context(), &ordering.attr_str()).into(),
            );
        }
        store_op
    }

    /// Creates an operation to allocate memory on the stack for a specified type.
    ///
    /// # Parameters
    /// - `ty`: The type of the memory block to allocate.
    ///
    /// # Returns
    /// An operation representing the memory allocation.
    pub fn alloca(&self, ty: Ty<'c, 'a>) -> Result<Op<'c, '_>> {
        let array_size = self.make(self.iconst_32(1))?;
        Ok(llvm::alloca(
            self.context(),
            array_size,
            self.ptr_ty(),
            self.unknown_loc(),
            AllocaOptions::new().elem_type(Some(TypeAttribute::new(ty))),
        ))
    }

    /// Creates a `llvm.getelementptr` operation.
    ///
    /// # Parameters
    /// - `ptr`: The base pointer (`Val`) from which the field's offset is computed.
    /// - `offset`: The byte offset of the field relative to the base pointer.
    /// - `element_type`: The element type of the field value due to the offset.
    /// - `result_type`: The expected type of the field value to be loaded.
    #[inline]
    pub fn gep(
        &self,
        ptr: Val<'c, 'a>,
        offset: usize,
        element_type: Type<'c>,
        result_type: Type<'c>,
    ) -> Op<'c, '_> {
        llvm::get_element_ptr(
            self.context(),
            ptr,
            DenseI32ArrayAttribute::new(self.context(), &[offset as i32]),
            element_type,
            result_type,
            self.get_insert_location(),
        )
    }

    /// Creates a `llvm.getelementptr` operation.
    ///
    /// # Parameters
    /// - `ptr`: The base pointer (`Val`) from which the field's offset is computed.
    /// - `indices`: The dynamic indices of the field relative to the base pointer.
    /// - `element_type`: The element type of the field value due to the offset.
    /// - `result_type`: The expected type of the field value to be loaded.
    #[inline]
    pub fn gep_dynamic<const N: usize>(
        &self,
        ptr: Val<'c, 'a>,
        indices: &[Val<'c, '_>; N],
        element_type: Type<'c>,
        result_type: Type<'c>,
    ) -> Op<'c, '_> {
        llvm::get_element_ptr_dynamic(
            self.context(),
            ptr,
            indices,
            element_type,
            result_type,
            self.get_insert_location(),
        )
    }

    /// Creates a `llvm.getelementptr` operation with the inbounds option.
    ///
    /// # Parameters
    /// - `ptr`: The base pointer (`Val`) from which the field's offset is computed.
    /// - `indices`: The dynamic indices of the field relative to the base pointer.
    /// - `element_type`: The element type of the field value due to the offset.
    /// - `result_type`: The expected type of the field value to be loaded.
    pub fn inbounds_gep_dynamic<const N: usize>(
        &self,
        ptr: Val<'c, 'a>,
        indices: &[Val<'c, '_>; N],
        element_type: Type<'c>,
        result_type: Type<'c>,
    ) -> Op<'c, '_> {
        let mut op = llvm::get_element_ptr_dynamic(
            self.context(),
            ptr,
            indices,
            element_type,
            result_type,
            self.get_insert_location(),
        );
        op.set_attribute("inbounds", Attribute::unit(self.context()));
        op
    }

    /// Retrieves the value of a field from a given pointer at a specified offset and interprets it as a specific type.
    ///
    /// This function calculates the address of a field within a structure or memory block by computing its offset
    /// relative to the base pointer. It performs two memory loads:
    /// 1. Loads a pointer to the target value from the computed offset.
    /// 2. Loads the actual value from the pointer, interpreting it as the specified type.
    ///
    /// # Parameters
    /// - `ptr`: The base pointer (`Val`) from which the field's offset is computed.
    /// - `offset`: The byte offset of the field relative to the base pointer.
    /// - `r#type`: The expected type of the field value to be loaded.
    ///
    /// # Returns
    /// A `Val` containing the loaded field value, interpreted as the specified type.
    ///
    /// # Errors
    /// Returns a `Result::Err` if any step in the pointer computation, memory load, or value casting fails.
    ///
    /// # Example
    /// ```no_check
    /// let value = context.get_field_value(base_ptr, 8, field_type)?;
    /// ```
    pub fn get_field_value(
        &self,
        ptr: Val<'c, 'a>,
        offset: usize,
        r#type: Type<'c>,
    ) -> Result<Val<'c, '_>> {
        let rtn_ptr = self.make(llvm::get_element_ptr(
            self.context(),
            ptr,
            DenseI32ArrayAttribute::new(self.context(), &[offset as i32]),
            self.intrinsics.i8_ty,
            self.intrinsics.ptr_ty,
            self.get_insert_location(),
        ))?;
        Ok(self
            .make(llvm::load(
                self.context(),
                rtn_ptr,
                r#type,
                self.get_insert_location(),
                llvm::LoadStoreOptions::default(),
            ))?
            .to_ctx_value())
    }

    /// Creates an operation to obtain the address of a global variable.
    ///
    /// # Parameters
    /// - `name`: The name of the global variable.
    /// - `ty`: The type of the global variable.
    ///
    /// # Returns
    /// An operation representing the address of the specified global variable.
    pub fn addressof(&self, name: &str, ty: Ty<'c, 'a>) -> Op<'c, '_> {
        let context = self.context();
        OperationBuilder::new("llvm.mlir.addressof", self.intrinsics.unknown_loc)
            .add_attributes(&[(
                Identifier::new(context, "global_name"),
                FlatSymbolRefAttribute::new(context, name).into(),
            )])
            .add_results(&[ty])
            .build()
            .expect("valid operation")
    }

    /// Creates an operation to perform an indirect function call.
    ///
    /// This function generates an MLIR operation (`llvm.call`) to call a function indirectly
    /// through a function pointer. The function type, function pointer, and arguments are
    /// provided as inputs.
    ///
    /// # Parameters
    /// - `func_ty`: The type of the function being called, including its argument and result types.
    /// - `func_ptr`: The function pointer (of type `!llvm.ptr`) used for the indirect call.
    /// - `args`: A slice of values representing the arguments to pass to the function.
    ///
    /// # Returns
    /// An operation representing the indirect function call.
    ///
    /// # Errors
    /// Returns an error if the operation cannot be built (e.g., due to invalid inputs).
    ///
    /// # Reference
    /// For more details, see the MLIR documentation on `llvm.call`:
    /// https://mlir.llvm.org/docs/Dialects/LLVM/#llvmcall-llvmcallop
    pub fn indirect_call(
        &self,
        ret_ty: Type<'c>,
        func_ptr: Val<'c, 'a>,
        args: &[Val<'c, 'a>],
    ) -> Result<Op<'c, '_>> {
        let context = self.context();
        // Reference: https://mlir.llvm.org/docs/Dialects/LLVM/#llvmcall-llvmcallop
        // For indirect calls, the callee is of !llvm.ptr type and is stored as the first value in callee_operands
        let args = std::iter::once(func_ptr)
            .chain(args.iter().copied())
            .collect::<Vec<Value<'_, '_>>>();
        Ok(
            OperationBuilder::new("llvm.call", self.intrinsics.unknown_loc)
                .add_attributes(&[(
                    Identifier::new(context, "var_callee_type"),
                    TypeAttribute::new(llvm::r#type::function(ret_ty, &[], true)).into(),
                )])
                .add_operands(&args)
                .add_results(&[ret_ty])
                .build()?,
        )
    }

    /// Creates a global variable with the specified name, type, and linkage.
    ///
    /// # Parameters
    /// - `name`: The name of the global variable.
    /// - `ty`: The type of the global variable.
    /// - `linkage`: The linkage of the global variable.
    ///
    /// # Returns
    /// An operation representing the global variable.
    pub fn global(
        &self,
        name: &str,
        ty: Ty<'c, 'a>,
        linkage: Linkage,
    ) -> melior::ir::Operation<'c> {
        let context = self.context();
        OperationBuilder::new("llvm.mlir.global", self.intrinsics.unknown_loc)
            .add_regions([Region::new()])
            .add_attributes(&[
                (
                    Identifier::new(context, "sym_name"),
                    StringAttribute::new(context, name).into(),
                ),
                (
                    Identifier::new(context, "global_type"),
                    TypeAttribute::new(ty).into(),
                ),
                (
                    Identifier::new(context, "linkage"),
                    llvm::attributes::linkage(context, linkage),
                ),
            ])
            .build()
            .expect("valid operation")
    }
}

impl<'c, 'a> OpBuilder<'c, 'a> {
    /// Sets the insertion point to a specified operation.
    ///
    /// # Parameters
    /// - `point`: A reference to the operation that will become the new insertion point.
    pub fn set_insert_point(&mut self, point: OperationRef<'c, '_>) {
        let point = point.to_ctx_operation_ref();
        self.insert_point = Some(point);
    }

    /// Sets the insertion point to the end of a specified block.
    ///
    /// # Parameters
    /// - `block`: A reference to the block where the insertion point will be set to the end.
    pub fn set_insert_point_to_block_end(&mut self, block: BlockRef<'c, 'a>) {
        self.block = Some(block);
        self.insert_point = None
    }

    /// Retrieves the current insertion point, if one is set.
    ///
    /// # Returns
    /// An `Option` containing a reference to the current insertion point operation, if available.
    pub fn get_insert_point(&self) -> Option<OperationRef<'c, '_>> {
        self.insert_point
    }

    /// Retrieves the current block, if one is set.
    ///
    /// # Returns
    /// An `Option` containing a reference to the current block, if available.
    pub fn get_insert_block(&self) -> Option<BlockRef<'c, '_>> {
        self.block
    }

    /// Gets the location of the insertion point.
    ///
    /// # Returns
    /// The location of the current insertion point, or an unknown location if no insertion point is set.
    #[inline]
    pub fn get_insert_location(&self) -> Location<'c> {
        if let Some(insert_point) = self.insert_point {
            unsafe { Location::from_raw(mlirOperationGetLocation(insert_point.to_raw())) }
        } else {
            self.intrinsics.unknown_loc
        }
    }

    /// Gets the name of the insertion point operation, if one is set.
    ///
    /// # Returns
    /// An `Option` containing the name of the insertion point operation as a `String`, if available.
    #[inline]
    pub fn get_insert_op_name(&self) -> Option<String> {
        self.insert_point.map(|insert_point| {
            insert_point
                .name()
                .as_string_ref()
                .as_str()
                .unwrap()
                .to_string()
        })
    }

    /// Inserts a new operation at a specified position relative to an existing operation.
    ///
    /// # Parameters
    /// - `one`: A reference operation at the insertion point.
    /// - `other`: The new operation to be inserted.
    ///
    /// # Returns
    /// A reference to the inserted operation.
    pub fn insert(&self, one: OperationRef<'c, '_>, other: Operation<'c>) -> OperationRef<'c, '_> {
        if let Some(block) = self.block {
            // If a block exists, insert the new operation before the specified operation
            let op = block.insert_operation_before(one, other);
            // Convert the inserted operation to a context operation reference
            op.to_ctx_operation_ref()
        } else {
            // If no block exists, directly convert the operation to a raw pointer and create an operation reference.
            unsafe { OperationRef::from_raw(other.into_raw()) }
        }
    }

    /// Creates and inserts a new operation at the current insertion point.
    ///
    /// # Parameters
    /// - `op`: The new operation to be created and inserted.
    ///
    /// # Returns
    /// A reference to the created and inserted operation.
    pub fn create(&self, op: Operation<'c>) -> OperationRef<'c, '_> {
        if let Some(block) = self.block {
            let op = if let Some(insert_point) = self.insert_point {
                block.insert_operation_before(insert_point, op)
            } else {
                block.append_operation(op)
            };
            op.to_ctx_operation_ref()
        } else {
            unsafe { OperationRef::from_raw(op.into_raw()) }
        }
    }

    /// Creates and inserts a new operation at the current insertion point, returning its first result.
    ///
    /// # Parameters
    /// - `op`: The new operation to be created and inserted.
    ///
    /// # Returns
    /// A result containing the first created and inserted operation result.
    pub fn make(&self, op: Operation<'c>) -> Result<Value<'c, '_>> {
        let op = self.create(op);
        Ok(op.result(0)?.into())
    }

    /// Creates and inserts a new operation at the current insertion point, returning its first `N` results.
    ///
    /// # Parameters
    /// - `op`: The new operation to be created and inserted.
    ///
    /// # Returns
    /// A result containing an array of the first `N` created and inserted operation results.
    pub fn make_n<const N: usize>(&self, op: Operation<'c>) -> Result<[Value<'c, '_>; N]> {
        debug_assert_ne!(N, 0);
        let op = self.create(op);
        Ok(std::array::from_fn(|i| op.result(i).unwrap().into()))
    }
}
