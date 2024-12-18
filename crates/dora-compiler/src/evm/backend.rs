use crate::backend::{IntCC, TypeMethods};
use crate::conversion::builder::OpBuilder;
use crate::errors::{CompileError, Result};
use crate::value::ToContextValue;
use dora_ir::IRTypes;
use dora_runtime::constants::MAX_STACK_SIZE;
use melior::dialect::arith::CmpiPredicate;
use melior::dialect::llvm::LoadStoreOptions;
use melior::dialect::{arith, cf, func, llvm};
use melior::ir::attribute::IntegerAttribute;
use melior::ir::attribute::{DenseI32ArrayAttribute, FlatSymbolRefAttribute, FloatAttribute};
use melior::ir::{Attribute, Block, BlockRef, Location, OperationRef, Region, ValueLike};
use melior::ir::{Type, Value};
use melior::Context as MLIRContext;
use num_bigint::BigUint;

use super::CtxType;

/// The `EVMBuilder` struct provides a specialized builder for generating and managing EVM (Ethereum Virtual Machine)
/// related MLIR operations. It combines the MLIR context, operation builder, and optional configurations such as
/// using a static stack. This struct facilitates the construction and manipulation of EVM-specific MLIR operations
/// in a controlled and flexible manner.
///
/// # Fields:
/// - `ctx`: A mutable reference to the context of type `CtxType` which represents the environment or context
///   in which the EVM operations are being generated. The context lifetime `'c` is tied to the EVM's lifetime.
/// - `builder`: An instance of `OpBuilder` used to generate MLIR operations within the given block and insertion point.
/// - `use_static_stack`: A boolean flag indicating whether to use a static stack for the EVM's execution model.
///   If set to `true`, the EVM will operate with a fixed stack size. This option can be useful for optimizing performance
///   in certain scenarios, such as when the maximum stack size is known ahead of time.
///
/// # Example Usage:
/// ```no_check
/// let mut evm_builder = EVMBuilder {
///     ctx: &mut evm_context,
///     builder: OpBuilder {
///         ctx: &mlir_context,
///         intrinsics: intrinsics,
///         block: Some(block_ref),
///         insert_point: Some(operation_ref),
///     },
///     use_static_stack: true,
/// };
/// // Generate EVM-specific MLIR operations using the `evm_builder`.
/// ```
///
/// # Notes:
/// - The `EVMBuilder` provides a flexible interface for working with EVM operations in MLIR, allowing users to configure
///   the execution context and stack model as needed.
/// - The `use_static_stack` option allows developers to optimize the execution model when a known stack size is required,
///   making it useful in environments where performance is critical and the stack behavior is predictable.
#[derive(Debug)]
pub struct EVMBuilder<'a, 'c> {
    /// The execution context for EVM operations, represented by `CtxType`. This holds the environment and settings
    /// used during the generation of EVM MLIR operations.
    pub ctx: &'a mut CtxType<'c>,

    /// The `OpBuilder` responsible for generating MLIR operations. This is used to create operations within
    /// a specific block and insertion point.
    pub builder: OpBuilder<'c, 'a>,

    /// A flag indicating whether to use a static stack for EVM execution. If set to `true`, the EVM will operate
    /// with a fixed stack size, which can improve performance when the stack size is known ahead of time.
    pub use_static_stack: bool,
}

impl<'a, 'c> std::ops::Deref for EVMBuilder<'a, 'c> {
    type Target = OpBuilder<'c, 'a>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}

impl std::ops::DerefMut for EVMBuilder<'_, '_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.builder
    }
}

impl<'a> IRTypes for EVMBuilder<'a, '_> {
    type Type = Type<'a>;
    type Value = Value<'a, 'a>;
    type Region = Region<'a>;
    type BasicBlock = BlockRef<'a, 'a>;
    type Operation = OperationRef<'a, 'a>;
}

impl TypeMethods for EVMBuilder<'_, '_> {
    fn type_ptr(&self) -> Self::Type {
        self.builder.ptr_ty()
    }

    fn type_int(&self, bits: u32) -> Self::Type {
        self.builder.type_int(bits)
    }
}

impl crate::backend::Builder for EVMBuilder<'_, '_> {
    fn bool_const(&mut self, value: bool) -> Result<Self::Value> {
        let ty = self.intrinsics.i1_ty;
        let op = self.builder.create(arith::constant(
            self.context(),
            IntegerAttribute::new(ty, if value { 1 } else { 0 }).into(),
            self.location(),
        ));
        Ok(op.result(0)?.to_ctx_value())
    }

    fn iconst(&mut self, ty: Self::Type, value: i64) -> Result<Self::Value> {
        let op = self.builder.create(arith::constant(
            self.context(),
            IntegerAttribute::new(ty, value).into(),
            self.location(),
        ));
        Ok(op.result(0)?.to_ctx_value())
    }

    fn uconst(&mut self, ty: Self::Type, value: u64) -> Result<Self::Value> {
        let op = self.builder.create(arith::constant(
            self.context(),
            IntegerAttribute::new(ty, value as i64).into(),
            self.location(),
        ));
        Ok(op.result(0)?.to_ctx_value())
    }

    fn iconst_32(&mut self, value: i32) -> Result<Self::Value> {
        self.iconst(self.intrinsics.i32_ty, value as i64)
    }

    fn iconst_64(&mut self, value: i64) -> Result<Self::Value> {
        self.iconst(self.intrinsics.i64_ty, value)
    }

    fn iconst_256(&mut self, value: BigUint) -> Result<Self::Value> {
        let op = self.builder.create(arith::constant(
            self.context(),
            Attribute::parse(self.context(), &format!("{} : i256", value)).ok_or(
                CompileError::Codegen(format!("can't parse value {value} to i256")),
            )?,
            self.location(),
        ));
        Ok(op.result(0)?.to_ctx_value())
    }

    fn fconst(&mut self, ty: Self::Type, value: f64) -> Result<Self::Value> {
        let op = self.builder.create(arith::constant(
            self.context(),
            FloatAttribute::new(self.context(), ty, value).into(),
            self.location(),
        ));
        Ok(op.result(0)?.to_ctx_value())
    }

    fn fconst_32(&mut self, value: f32) -> Result<Self::Value> {
        self.fconst(self.intrinsics.f32_ty, value as f64)
    }

    fn fconst_64(&mut self, value: f64) -> Result<Self::Value> {
        self.fconst(self.intrinsics.f64_ty, value)
    }

    fn stack_push(&mut self, value: Self::Value) -> Result<()> {
        let value = unsafe { Value::from_raw(value.to_raw()) };
        let builder = &self.builder;
        // Load stack pointer
        let stack_ptr =
            builder.make(builder.load(self.ctx.values.stack_top_ptr, builder.ptr_ty()))?;
        builder.create(builder.store(value, stack_ptr));
        // Increment stack pointer
        let new_stack_ptr = builder.make(llvm::get_element_ptr(
            builder.context(),
            stack_ptr,
            DenseI32ArrayAttribute::new(builder.context(), &[1]),
            builder.intrinsics.i256_ty,
            builder.ptr_ty(),
            builder.get_insert_location(),
        ))?;
        builder.create(builder.store(new_stack_ptr, self.ctx.values.stack_top_ptr));
        Ok(())
    }

    fn stack_pop(&mut self) -> Result<Self::Value> {
        let builder = &self.builder;
        // Load stack pointer
        let stack_ptr =
            builder.make(builder.load(self.ctx.values.stack_top_ptr, builder.ptr_ty()))?;
        let old_stack_ptr = builder.make(llvm::get_element_ptr(
            builder.context(),
            stack_ptr,
            DenseI32ArrayAttribute::new(builder.context(), &[-1]),
            builder.intrinsics.i256_ty,
            builder.ptr_ty(),
            builder.get_insert_location(),
        ))?;
        let value = builder.make(builder.load(old_stack_ptr, builder.intrinsics.i256_ty))?;
        builder.create(builder.store(old_stack_ptr, self.ctx.values.stack_top_ptr));
        Ok(unsafe { Value::from_raw(value.to_raw()) })
    }

    #[inline]
    fn stack_peek(&mut self) -> Result<Self::Value> {
        self.stack_peek_nth(1)
    }

    fn stack_peek_nth(&mut self, n: usize) -> Result<Self::Value> {
        debug_assert!(n < MAX_STACK_SIZE);
        let builder = &self.builder;
        // Load stack pointer
        let stack_ptr =
            builder.make(builder.load(self.ctx.values.stack_top_ptr, builder.ptr_ty()))?;
        // n-th stack pointer
        let nth_stack_ptr = builder.make(llvm::get_element_ptr(
            builder.context(),
            stack_ptr,
            DenseI32ArrayAttribute::new(builder.context(), &[-(n as i32)]),
            builder.intrinsics.i256_ty,
            builder.ptr_ty(),
            builder.get_insert_location(),
        ))?;
        let value = builder.make(builder.load(nth_stack_ptr, builder.intrinsics.i256_ty))?;
        Ok(unsafe { Value::from_raw(value.to_raw()) })
    }

    fn stack_exchange(&mut self, n: usize, m: usize) -> Result<()> {
        let n = n + 1;
        let m = m + 1;
        let builder = &self.builder;
        // Load stack pointer
        let stack_ptr =
            builder.make(builder.load(self.ctx.values.stack_top_ptr, builder.ptr_ty()))?;
        // n-th stack pointer
        let nth_stack_ptr = builder.make(llvm::get_element_ptr(
            builder.context(),
            stack_ptr,
            DenseI32ArrayAttribute::new(builder.context(), &[-(n as i32)]),
            builder.intrinsics.i256_ty,
            builder.ptr_ty(),
            builder.get_insert_location(),
        ))?;
        // m-th stack pointer
        let mth_stack_ptr = builder.make(llvm::get_element_ptr(
            builder.context(),
            stack_ptr,
            DenseI32ArrayAttribute::new(builder.context(), &[-(m as i32)]),
            builder.intrinsics.i256_ty,
            builder.ptr_ty(),
            builder.get_insert_location(),
        ))?;
        let n_value = builder.make(builder.load(nth_stack_ptr, builder.intrinsics.i256_ty))?;
        let m_value = builder.make(builder.load(mth_stack_ptr, builder.intrinsics.i256_ty))?;
        builder.create(builder.store(n_value, mth_stack_ptr));
        builder.create(builder.store(m_value, nth_stack_ptr));
        Ok(())
    }

    fn load(&mut self, ty: Self::Type, ptr: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(llvm::load(
            self.context(),
            ptr,
            ty,
            self.location(),
            LoadStoreOptions::default(),
        ));
        Ok(op.result(0)?.to_ctx_value())
    }

    fn store(&mut self, value: Self::Value, ptr: Self::Value) {
        self.builder.create(llvm::store(
            self.context(),
            value,
            ptr,
            self.location(),
            LoadStoreOptions::default(),
        ));
    }

    fn nop(&mut self) {
        // Nothing to do
    }

    fn ret(&mut self, values: &[Self::Value]) {
        self.builder.create(func::r#return(values, self.location()));
    }

    fn icmp(
        &mut self,
        cond: crate::backend::IntCC,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> Result<Self::Value> {
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

        let op = self
            .builder
            .create(arith::cmpi(self.context(), pred, lhs, rhs, self.location()));
        Ok(op.result(0)?.to_ctx_value())
    }

    fn icmp_imm(
        &mut self,
        cond: crate::backend::IntCC,
        lhs: Self::Value,
        rhs: i64,
    ) -> Result<Self::Value> {
        let ty = lhs.r#type();
        let rhs = self.iconst(ty, rhs)?;
        self.icmp(cond, lhs, rhs)
    }

    fn is_null(&mut self, ptr: Self::Value) -> Result<Self::Value> {
        let zero = self.iconst(ptr.r#type(), 0)?;
        self.icmp(IntCC::Equal, ptr, zero)
    }

    fn is_not_null(&mut self, ptr: Self::Value) -> Result<Self::Value> {
        let zero = self.iconst(ptr.r#type(), 0)?;
        self.icmp(IntCC::NotEqual, ptr, zero)
    }

    fn br(&mut self, dest: Self::BasicBlock) {
        self.builder.create(cf::br(&dest, &[], self.location()));
    }

    fn brif(
        &mut self,
        cond: Self::Value,
        then_block: Self::BasicBlock,
        else_block: Self::BasicBlock,
        then_values: &[Self::Value],
        else_values: &[Self::Value],
    ) {
        self.builder.create(cf::cond_br(
            self.context(),
            cond,
            &then_block,
            &else_block,
            then_values,
            else_values,
            self.location(),
        ));
    }

    fn switch(
        &mut self,
        default: Self::BasicBlock,
        targets: &[(u64, Self::BasicBlock)],
        flag: Self::Value,
        flag_type: Self::Type,
        _default_is_cold: bool,
    ) -> Result<()> {
        let mut case_values = Vec::new();
        let mut case_destinations = Vec::new();
        for (value, block) in targets {
            case_values.push(*value as i64);
            case_destinations.push(*block);
        }
        let case_destinations: Vec<_> = case_destinations
            .iter()
            .map(|b| {
                let x: (&Block, &[Self::Value]) = (b, &[]);
                x
            })
            .collect();
        let op = cf::switch(
            self.context(),
            &case_values,
            flag,
            flag_type,
            (&default, &[]),
            &case_destinations,
            self.location(),
        )?;

        self.builder.create(op);
        Ok(())
    }

    fn br_indirect(&mut self, successor: Self::BasicBlock) {
        self.builder
            .create(cf::br(&successor, &[], self.location()));
    }

    fn select(
        &mut self,
        cond: Self::Value,
        then_value: Self::Value,
        else_value: Self::Value,
    ) -> Result<Self::Value> {
        let op = self
            .builder
            .create(arith::select(cond, then_value, else_value, self.location()));
        Ok(op.result(0)?.to_ctx_value())
    }

    fn iadd(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::add(self.context(), self.uint256_ty(), lhs, rhs, self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn isub(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::sub(self.context(), self.uint256_ty(), lhs, rhs, self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn imul(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::mul(self.context(), self.uint256_ty(), lhs, rhs, self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn udiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::div(self.context(), self.uint256_ty(), lhs, rhs, self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn sdiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::sdiv(self.context(), self.uint256_ty(), lhs, rhs, self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn umod(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::r#mod(self.context(), self.uint256_ty(), lhs, rhs, self.location())
                .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn smod(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::smod(self.context(), self.uint256_ty(), lhs, rhs, self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn addmod(
        &mut self,
        lhs: Self::Value,
        rhs: Self::Value,
        modulus: Self::Value,
    ) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::addmod(
                self.context(),
                self.uint256_ty(),
                lhs,
                rhs,
                modulus,
                self.location(),
            )
            .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn mulmod(
        &mut self,
        lhs: Self::Value,
        rhs: Self::Value,
        modulus: Self::Value,
    ) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::mulmod(
                self.context(),
                self.uint256_ty(),
                lhs,
                rhs,
                modulus,
                self.location(),
            )
            .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn exp(&mut self, base: Self::Value, exponent: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::exp(
                self.context(),
                self.uint256_ty(),
                base,
                exponent,
                self.location(),
            )
            .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn signextend(&mut self, byte: Self::Value, value: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::signextend(
                self.context(),
                self.uint256_ty(),
                byte,
                value,
                self.location(),
            )
            .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn lt(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::lt(self.context(), self.uint256_ty(), lhs, rhs, self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn gt(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::gt(self.context(), self.uint256_ty(), lhs, rhs, self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn slt(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::slt(self.context(), self.uint256_ty(), lhs, rhs, self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn sgt(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::sgt(self.context(), self.uint256_ty(), lhs, rhs, self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn eq(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::eq(self.context(), self.uint256_ty(), lhs, rhs, self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn iszero(&mut self, value: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::iszero(self.context(), self.uint256_ty(), value, self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn and(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::and(self.context(), self.uint256_ty(), lhs, rhs, self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn or(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::or(self.context(), self.uint256_ty(), lhs, rhs, self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn xor(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::xor(self.context(), self.uint256_ty(), lhs, rhs, self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn not(&mut self, value: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::not(self.context(), self.uint256_ty(), value, self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn byte(&mut self, index: Self::Value, value: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::byte(
                self.context(),
                self.uint256_ty(),
                index,
                value,
                self.location(),
            )
            .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn shl(&mut self, shift: Self::Value, value: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::shl(
                self.context(),
                self.uint256_ty(),
                shift,
                value,
                self.location(),
            )
            .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn shr(&mut self, shift: Self::Value, value: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::shr(
                self.context(),
                self.uint256_ty(),
                shift,
                value,
                self.location(),
            )
            .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn sar(&mut self, shift: Self::Value, value: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::sar(
                self.context(),
                self.uint256_ty(),
                shift,
                value,
                self.location(),
            )
            .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn urem(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value> {
        let op = arith::remui(lhs, rhs, self.location());
        Ok(op.result(0)?.to_ctx_value())
    }

    fn srem(&mut self, lhs: Self::Value, rhs: Self::Value) -> Result<Self::Value> {
        let op = arith::remsi(lhs, rhs, self.location());
        Ok(op.result(0)?.to_ctx_value())
    }

    fn iadd_imm(&mut self, lhs: Self::Value, rhs: i64) -> Result<Self::Value> {
        let ty = lhs.r#type();
        let rhs = self.iconst(ty, rhs)?;
        self.iadd(lhs, rhs)
    }

    fn isub_imm(&mut self, lhs: Self::Value, rhs: i64) -> Result<Self::Value> {
        let ty = lhs.r#type();
        let rhs = self.iconst(ty, rhs)?;
        self.isub(lhs, rhs)
    }

    fn imul_imm(&mut self, lhs: Self::Value, rhs: i64) -> Result<Self::Value> {
        let ty = lhs.r#type();
        let rhs = self.iconst(ty, rhs)?;
        self.imul(lhs, rhs)
    }

    fn ireduce(&mut self, to: Self::Type, value: Self::Value) -> Result<Self::Value> {
        let op = arith::trunci(value, to, self.location());
        Ok(op.result(0)?.to_ctx_value())
    }

    fn gep(
        &mut self,
        ptr: Self::Value,
        indices: &[i32],
        element_ty: Self::Type,
        result_ty: Self::Type,
    ) -> Result<Self::Value> {
        let op = llvm::get_element_ptr(
            self.context(),
            ptr,
            DenseI32ArrayAttribute::new(self.context(), indices),
            element_ty,
            result_ty,
            self.location(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn memcpy(&mut self, dst: Self::Value, src: Self::Value, len: Self::Value) -> Result<()> {
        let inline = false;
        let volatile = self.bool_const(false)?;
        let name = format!(
            "llvm.memcpy{}.p0.p0.{}",
            if inline { ".inline" } else { "" },
            len.r#type(),
        );

        self.builder.create(func::call(
            self.context(),
            FlatSymbolRefAttribute::new(self.context(), &name),
            &[dst, src, len, volatile],
            &[],
            self.location(),
        ));
        Ok(())
    }

    fn unreachable(&mut self) {
        self.builder.create(llvm::unreachable(self.location()));
    }
}

impl crate::backend::EVMBuilder for EVMBuilder<'_, '_> {
    fn keccak256(&mut self, start: Self::Value, length: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::keccak_256(
                self.context(),
                self.uint256_ty(),
                start,
                length,
                self.location(),
            )
            .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn address(&mut self) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::address(self.context(), self.uint256_ty(), self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn caller(&mut self) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::caller(self.context(), self.uint256_ty(), self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn callvalue(&mut self) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::callvalue(self.context(), self.uint256_ty(), self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn calldataload(&mut self, offset: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::calldataload(self.context(), self.uint256_ty(), offset, self.location())
                .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn calldatasize(&mut self) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::calldatasize(self.context(), self.uint256_ty(), self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn calldatacopy(
        &mut self,
        dest_offset: Self::Value,
        data_offset: Self::Value,
        length: Self::Value,
    ) {
        self.builder.create(
            dora_ir::evm::calldatacopy(
                self.context(),
                dest_offset,
                data_offset,
                length,
                self.location(),
            )
            .into(),
        );
    }

    fn codesize(&mut self) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::codesize(self.context(), self.uint256_ty(), self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn codecopy(
        &mut self,
        dest_offset: Self::Value,
        data_offset: Self::Value,
        length: Self::Value,
    ) {
        self.builder.create(
            dora_ir::evm::codecopy(
                self.context(),
                dest_offset,
                data_offset,
                length,
                self.location(),
            )
            .into(),
        );
    }

    fn returndatasize(&mut self) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::returndatasize(self.context(), self.uint256_ty(), self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn returndatacopy(
        &mut self,
        dest_offset: Self::Value,
        data_offset: Self::Value,
        length: Self::Value,
    ) {
        self.builder.create(
            dora_ir::evm::returndatacopy(
                self.context(),
                dest_offset,
                data_offset,
                length,
                self.location(),
            )
            .into(),
        );
    }

    fn returndataload(&mut self, offset: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::returndataload(
                self.context(),
                self.uint256_ty(),
                offset,
                self.location(),
            )
            .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn gas(&mut self) -> Result<Self::Value> {
        let op = self
            .builder
            .create(dora_ir::evm::gas(self.context(), self.uint256_ty(), self.location()).into());
        Ok(op.result(0)?.to_ctx_value())
    }

    fn balance(&mut self, account: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::balance(self.context(), self.uint256_ty(), account, self.location())
                .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn selfbalance(&mut self) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::selfbalance(self.context(), self.uint256_ty(), self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn extcodesize(&mut self, account: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::extcodesize(self.context(), self.uint256_ty(), account, self.location())
                .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn extcodehash(&mut self, account: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::extcodehash(self.context(), self.uint256_ty(), account, self.location())
                .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn extcodecopy(
        &mut self,
        account: Self::Value,
        dest_offset: Self::Value,
        code_offset: Self::Value,
        length: Self::Value,
    ) {
        self.builder.create(
            dora_ir::evm::extcodecopy(
                self.context(),
                account,
                dest_offset,
                code_offset,
                length,
                self.location(),
            )
            .into(),
        );
    }

    fn blockhash(&mut self, block_number: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::blockhash(
                self.context(),
                self.uint256_ty(),
                block_number,
                self.location(),
            )
            .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn sload(&mut self, key: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::sload(self.context(), self.uint256_ty(), key, self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn sstore(&mut self, key: Self::Value, value: Self::Value) {
        self.builder
            .create(dora_ir::evm::sstore(self.context(), key, value, self.location()).into());
    }

    fn tstore(&mut self, key: Self::Value, value: Self::Value) {
        self.builder
            .create(dora_ir::evm::tstore(self.context(), key, value, self.location()).into());
    }

    fn tload(&mut self, key: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::tload(self.context(), self.uint256_ty(), key, self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn log0(&mut self, offset: Self::Value, length: Self::Value) {
        self.builder
            .create(dora_ir::evm::log_0(self.context(), offset, length, self.location()).into());
    }

    fn log1(&mut self, offset: Self::Value, length: Self::Value, topic: Self::Value) {
        self.builder.create(
            dora_ir::evm::log_1(self.context(), offset, length, topic, self.location()).into(),
        );
    }

    fn log2(
        &mut self,
        offset: Self::Value,
        length: Self::Value,
        topic1: Self::Value,
        topic2: Self::Value,
    ) {
        self.builder.create(
            dora_ir::evm::log_2(
                self.context(),
                offset,
                length,
                topic1,
                topic2,
                self.location(),
            )
            .into(),
        );
    }

    fn log3(
        &mut self,
        offset: Self::Value,
        length: Self::Value,
        topic1: Self::Value,
        topic2: Self::Value,
        topic3: Self::Value,
    ) {
        self.builder.create(
            dora_ir::evm::log_3(
                self.context(),
                offset,
                length,
                topic1,
                topic2,
                topic3,
                self.location(),
            )
            .into(),
        );
    }

    fn log4(
        &mut self,
        offset: Self::Value,
        length: Self::Value,
        topic1: Self::Value,
        topic2: Self::Value,
        topic3: Self::Value,
        topic4: Self::Value,
    ) {
        self.builder.create(
            dora_ir::evm::log_4(
                self.context(),
                offset,
                length,
                topic1,
                topic2,
                topic3,
                topic4,
                self.location(),
            )
            .into(),
        );
    }

    fn dataload(&mut self, offset: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::dataload(self.context(), self.uint256_ty(), offset, self.location())
                .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn dataloadn(&mut self, offset: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::dataloadn(self.context(), self.uint256_ty(), offset, self.location())
                .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn datasize(&mut self) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::datasize(self.context(), self.uint256_ty(), self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn datacopy(&mut self, mem_offset: Self::Value, offset: Self::Value, size: Self::Value) {
        self.builder.create(
            dora_ir::evm::datacopy(self.context(), mem_offset, offset, size, self.location())
                .into(),
        );
    }

    fn selfdestruct(&mut self, recipient: Self::Value) {
        self.builder
            .create(dora_ir::evm::selfdestruct(self.context(), recipient, self.location()).into());
    }

    fn chainid(&mut self) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::chainid(self.context(), self.uint256_ty(), self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn coinbase(&mut self) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::coinbase(self.context(), self.uint256_ty(), self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn timestamp(&mut self) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::timestamp(self.context(), self.uint256_ty(), self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn number(&mut self) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::number(self.context(), self.uint256_ty(), self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn prevrandao(&mut self) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::prevrandao(self.context(), self.uint256_ty(), self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn gaslimit(&mut self) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::gaslimit(self.context(), self.uint256_ty(), self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn gasprice(&mut self) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::gasprice(self.context(), self.uint256_ty(), self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn basefee(&mut self) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::basefee(self.context(), self.uint256_ty(), self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn origin(&mut self) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::origin(self.context(), self.uint256_ty(), self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn blobhash(&mut self, index: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::blobhash(self.context(), self.uint256_ty(), index, self.location())
                .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn blobbasefee(&mut self) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::blobbasefee(self.context(), self.uint256_ty(), self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn mload(&mut self, offset: Self::Value) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::mload(self.context(), self.uint256_ty(), offset, self.location()).into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn mstore(&mut self, offset: Self::Value, data: Self::Value) {
        self.builder
            .create(dora_ir::evm::mstore(self.context(), offset, data, self.location()).into());
    }

    fn mstore8(&mut self, offset: Self::Value, data: Self::Value) {
        self.builder
            .create(dora_ir::evm::mstore_8(self.context(), offset, data, self.location()).into());
    }

    fn msize(&mut self) -> Result<Self::Value> {
        let op = self
            .builder
            .create(dora_ir::evm::msize(self.context(), self.uint256_ty(), self.location()).into());
        Ok(op.result(0)?.to_ctx_value())
    }

    fn mcopy(&mut self, dest_offset: Self::Value, src_offset: Self::Value, length: Self::Value) {
        self.builder.create(
            dora_ir::evm::mcopy(
                self.context(),
                dest_offset,
                src_offset,
                length,
                self.location(),
            )
            .into(),
        );
    }

    fn create(
        &mut self,
        value: Self::Value,
        offset: Self::Value,
        length: Self::Value,
    ) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::create(
                self.context(),
                self.uint256_ty(),
                value,
                offset,
                length,
                self.location(),
            )
            .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn create2(
        &mut self,
        value: Self::Value,
        offset: Self::Value,
        length: Self::Value,
        salt: Self::Value,
    ) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::create_2(
                self.context(),
                self.uint256_ty(),
                value,
                offset,
                length,
                salt,
                self.location(),
            )
            .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn eofcreate(
        &mut self,
        initcontainer_index: Self::Value,
        value: Self::Value,
        salt: Self::Value,
        input_offset: Self::Value,
        input_size: Self::Value,
    ) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::eofcreate(
                self.context(),
                self.uint256_ty(),
                initcontainer_index,
                value,
                salt,
                input_offset,
                input_size,
                self.location(),
            )
            .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn returncontract(
        &mut self,
        deploy_container_index: Self::Value,
        aux_data_offset: Self::Value,
        aux_data_size: Self::Value,
    ) {
        self.builder.create(
            dora_ir::evm::returncontract(
                self.context(),
                deploy_container_index,
                aux_data_offset,
                aux_data_size,
                self.location(),
            )
            .into(),
        );
    }

    fn call(
        &mut self,
        gas: Self::Value,
        address: Self::Value,
        value: Self::Value,
        input_offset: Self::Value,
        input_length: Self::Value,
        output_offset: Self::Value,
        output_length: Self::Value,
    ) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::call(
                self.context(),
                self.uint256_ty(),
                gas,
                address,
                value,
                input_offset,
                input_length,
                output_offset,
                output_length,
                self.location(),
            )
            .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn callf(&mut self, target_section_index: Self::Value) {
        self.builder.create(
            dora_ir::evm::callf(self.context(), target_section_index, self.location()).into(),
        );
    }

    fn retf(&mut self) {
        self.builder
            .create(dora_ir::evm::retf(self.context(), self.location()).into());
    }

    fn callcode(
        &mut self,
        gas: Self::Value,
        address: Self::Value,
        value: Self::Value,
        input_offset: Self::Value,
        input_length: Self::Value,
        output_offset: Self::Value,
        output_length: Self::Value,
    ) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::callcode(
                self.context(),
                self.uint256_ty(),
                gas,
                address,
                value,
                input_offset,
                input_length,
                output_offset,
                output_length,
                self.location(),
            )
            .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn delegatecall(
        &mut self,
        gas: Self::Value,
        address: Self::Value,
        input_offset: Self::Value,
        input_length: Self::Value,
        output_offset: Self::Value,
        output_length: Self::Value,
    ) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::delegatecall(
                self.context(),
                self.uint256_ty(),
                gas,
                address,
                input_offset,
                input_length,
                output_offset,
                output_length,
                self.location(),
            )
            .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn extcall(
        &mut self,
        target_address: Self::Value,
        input_offset: Self::Value,
        input_size: Self::Value,
        value: Self::Value,
    ) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::extcall(
                self.context(),
                self.uint256_ty(),
                target_address,
                input_offset,
                input_size,
                value,
                self.location(),
            )
            .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn extdelegatecall(
        &mut self,
        target_address: Self::Value,
        input_offset: Self::Value,
        input_size: Self::Value,
    ) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::extdelegatecall(
                self.context(),
                self.uint256_ty(),
                target_address,
                input_offset,
                input_size,
                self.location(),
            )
            .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn staticcall(
        &mut self,
        gas: Self::Value,
        address: Self::Value,
        input_offset: Self::Value,
        input_length: Self::Value,
        output_offset: Self::Value,
        output_length: Self::Value,
    ) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::staticcall(
                self.context(),
                self.uint256_ty(),
                gas,
                address,
                input_offset,
                input_length,
                output_offset,
                output_length,
                self.location(),
            )
            .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn creturn(&mut self, offset: Self::Value, length: Self::Value) {
        self.builder
            .create(dora_ir::evm::r#return(self.context(), offset, length, self.location()).into());
    }

    fn extstaticcall(
        &mut self,
        target_address: Self::Value,
        input_offset: Self::Value,
        input_size: Self::Value,
    ) -> Result<Self::Value> {
        let op = self.builder.create(
            dora_ir::evm::extstaticcall(
                self.context(),
                self.uint256_ty(),
                target_address,
                input_offset,
                input_size,
                self.location(),
            )
            .into(),
        );
        Ok(op.result(0)?.to_ctx_value())
    }

    fn revert(&mut self, offset: Self::Value, length: Self::Value) {
        self.builder
            .create(dora_ir::evm::revert(self.context(), offset, length, self.location()).into());
    }

    fn stop(&mut self) {
        self.builder
            .create(dora_ir::evm::stop(self.context(), self.location()).into());
    }

    fn invalid(&mut self) {
        self.builder
            .create(dora_ir::evm::invalid(self.context(), self.location()).into());
    }
}

impl<'a, 'c> EVMBuilder<'a, 'c> {
    /// Returns a reference to the MLIR context associated with the builder.
    ///
    /// # Returns
    /// A reference to the `MLIRContext` that this builder uses.
    #[inline]
    pub fn context(&self) -> &'a MLIRContext {
        self.ctx.context
    }

    /// Retrieves the unknown location intrinsic for operations.
    ///
    /// This method provides a default location that can be used for operations where
    /// the actual location is not specified.
    ///
    /// # Returns
    /// A `Location` representing the unknown location.
    #[inline]
    pub fn location(&self) -> Location<'c> {
        self.intrinsics.unknown_loc
    }

    /// Gets the type representing a 256-bit unsigned integer.
    ///
    /// This method returns the type used for `uint256` operations in the EVM.
    ///
    /// # Returns
    /// A `Type` representing a 256-bit unsigned integer.
    #[inline]
    pub fn uint256_ty(&self) -> Type<'c> {
        self.intrinsics.i256_ty
    }

    /// Retrieves the pointer type used in the EVM context.
    ///
    /// This method returns the type used for pointers in the EVM.
    ///
    /// # Returns
    /// A `Type` representing the pointer type.
    #[inline]
    pub fn ptr_ty(&self) -> Type<'c> {
        self.intrinsics.ptr_ty
    }
}
