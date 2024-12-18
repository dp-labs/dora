//! This macro file defines a series of macros primarily focused on operations involving operands, rewriting contexts, and handling system calls.

/// Extracts operands from an operation.
///
/// [`operands!`] supports from 1 to 4 operands.
/// Here are examples:
///
/// ```ignore
/// operands!(op, input1);
/// operands!(op, input1, input2);
/// operands!(op, input1, input2, input3);
/// operands!(op, input1, input2, input3, input4);
/// ```
#[macro_export]
macro_rules! operands {
    ($op:expr, $o1:ident) => {
        let $o1 = $op.operand(0)?;
    };
    ($op:expr, $o1:ident, $o2:ident) => {
        let ($o1, $o2) = ($op.operand(0)?, $op.operand(1)?);
    };
    ($op:expr, $o1:ident, $o2:ident, $o3:ident) => {
        let ($o1, $o2, $o3) = ($op.operand(0)?, $op.operand(1)?, $op.operand(2)?);
    };
    ($op:expr, $o1:ident, $o2:ident, $o3:ident, $o4:ident) => {
        let ($o1, $o2, $o3, $o4) = (
            $op.operand(0)?,
            $op.operand(1)?,
            $op.operand(2)?,
            $op.operand(3)?,
        );
    };
}

/// Creates a rewriter context for operations, depending on deferred option.
///
/// [`rewrite_ctx!`] creates deffered or non-deffered rewirter depending on `NoDefer` option.
/// There are two forms of this macro:
///
/// - Creates a [DeferredRewriter](crate::conversion::rewriter::DeferredRewriter) context.
/// ```ignore
/// rewrite_ctx!(conext, op, rewriter, location);
/// ```
///
/// - Crates a [Rewriter](crate::conversion::rewriter::Rewriter) context.
/// ```ignore
/// rewrite_ctx!(conext, op, rewriter, location, NoDefer);
/// ```
#[macro_export]
macro_rules! rewrite_ctx {
    ($context:expr, $op:expr, $rewriter:ident, $loc:ident) => {
        let r = $crate::conversion::rewriter::DeferredRewriter::new_with_op($context, *$op);
        let l = r.get_insert_location();

        let ($rewriter, $loc) = (r, l);

        scopeguard::defer! {
            $rewriter.remove();
        }
    };
    ($context:expr, $op:expr, $rewriter:ident, $loc:ident, NoDefer) => {
        let r = $crate::conversion::rewriter::Rewriter::new_with_op($context, *$op);
        let l = r.get_insert_location();

        let ($rewriter, $loc) = (r, l);
    };
}

/// Extracts arguments from the first block of a parent operation.
///
/// [`block_argument!`] serves as a general macro for every MLIR/LLVM code.
///
/// There are two forms of this macro:
///
/// - Extracts `syscall_ctx` which is the first argument from the first block.
///
/// ```ignore
/// block_argument!(op, syscall_ctx);
/// ```
///
/// - Extracts `syscall_ctx` and `gas_counter_ptr` which is the send argument from the first block
///   and converted to [Value](melior::ir::Value) type.
///
/// ```ignore
/// block_argument!(op, syscall_ctx, gas_counter_ptr);
/// ```
#[macro_export]
macro_rules! block_argument {
    ($op:expr, $syscall_ctx:ident) => {
        let func_op = $op.parent_operation().unwrap();
        let region = func_op.region(0).unwrap();
        let func_block = region.first_block().unwrap();
        let $syscall_ctx = func_block.argument(0).unwrap();
    };
    ($op:expr, $syscall_ctx:ident, $gas_counter_ptr:ident) => {
        use melior::ir::{Value, ValueLike};
        let func_op = $op.parent_operation().unwrap();
        let region = func_op.region(0).unwrap();
        let func_block = region.first_block().unwrap();
        let $syscall_ctx = func_block.argument(0).unwrap();
        let $gas_counter_ptr: Value<'_, '_> = func_block.argument(1).unwrap().into();
        let $gas_counter_ptr: Value<'_, '_> = unsafe { Value::from_raw($gas_counter_ptr.to_raw()) };
    };
}

/// Creates an arithmetic constant.
#[macro_export]
macro_rules! arith_constant {
    ($rewriter:expr, $context:expr, $ty:expr, $value:expr, $location:expr) => {
        arith::constant(
            $context,
            melior::ir::attribute::IntegerAttribute::new($ty.into(), $value).into(),
            $location,
        )
    };
}

/// Macro to create a variable with default size of 1.
#[macro_export(local_inner_macros)]
macro_rules! create_var {
    ($rewriter:expr, $context:expr, $location:expr) => {{
        use melior::dialect::llvm::{self, AllocaOptions};
        let array_size = $rewriter.make(arith_constant!(
            $rewriter,
            $context,
            $rewriter.intrinsics.i256_ty,
            1_i64,
            $location
        ))?;
        $rewriter.make(llvm::alloca(
            $context,
            array_size,
            $rewriter.ptr_ty(),
            $location,
            AllocaOptions::new().elem_type(Some(TypeAttribute::new(
                $rewriter.intrinsics.i256_ty.into(),
            ))),
        ))?
    }};
    ($rewriter:expr, $context:expr, $elm_type:expr, $location:expr) => {{
        use melior::dialect::llvm::{self, AllocaOptions};
        let array_size = $rewriter.make(arith_constant!(
            $rewriter, $context, $elm_type, 1_i64, $location
        ))?;
        $rewriter.make(llvm::alloca(
            $context,
            array_size,
            $rewriter.ptr_ty(),
            $location,
            AllocaOptions::new().elem_type(Some(TypeAttribute::new($elm_type))),
        ))?
    }};
}

/// Macro to load a variable from memory.
#[macro_export]
macro_rules! load_var {
    ($rewriter:expr, $context:expr, $arg:expr, $ltype:expr, $location:expr) => {{
        use melior::dialect::llvm::{self, LoadStoreOptions};
        $rewriter.make(llvm::load(
            $context,
            $arg,
            $ltype,
            $location,
            LoadStoreOptions::default(),
        ))?
    }};
    ($rewriter:expr, $context:expr, $syscall_ctx:expr, $symbol:expr, $args:expr, $ltype:expr, $location:expr) => {{
        use melior::dialect::{
            func,
            llvm::{self, LoadStoreOptions},
        };
        let ptr_type = $rewriter.ptr_ty();
        let args = [&[$syscall_ctx.into()][..], &$args[..]].concat();
        $rewriter.create(func::call(
            $context,
            melior::ir::attribute::FlatSymbolRefAttribute::new($context, $symbol),
            &args,
            &[ptr_type],
            $location,
        ));
        $rewriter.make(llvm::load(
            $context,
            $args[0],
            $ltype,
            $location,
            LoadStoreOptions::default(),
        ))?
    }};
    ($rewriter:expr, $context:expr, $syscall_ctx:expr, $symbol:expr, $args:expr, $laddr:expr, $ltype:expr, $location:expr) => {{
        use melior::dialect::{
            func,
            llvm::{self, LoadStoreOptions},
        };
        let ptr_type = $rewriter.ptr_ty();
        let args = [&[$syscall_ctx.into()][..], &$args[..]].concat();
        $rewriter.create(func::call(
            $context,
            melior::ir::attribute::FlatSymbolRefAttribute::new($context, $symbol),
            &args,
            &[ptr_type],
            $location,
        ));
        $rewriter.make(llvm::load(
            $context,
            $laddr,
            $ltype,
            $location,
            LoadStoreOptions::default(),
        ))?
    }};
    ($rewriter:expr, $context:expr, $syscall_ctx:expr, $symbol:expr, $args:expr, $rtn_types:expr, $laddr:expr, $ltype:expr, $location:expr) => {{
        use melior::dialect::{
            func,
            llvm::{self, LoadStoreOptions},
        };
        let args = [&[$syscall_ctx.into()][..], &$args[..]].concat();
        $rewriter.create(func::call(
            $context,
            melior::ir::attribute::FlatSymbolRefAttribute::new($context, $symbol),
            &args,
            &$rtn_types,
            $location,
        ));
        $rewriter.make(llvm::load(
            $context,
            $laddr,
            $ltype,
            $location,
            LoadStoreOptions::default(),
        ))?
    }};
    ($rewriter:expr, $context:expr, $syscall_ctx:expr, $symbol:expr, $args:expr, $rtn_types:expr, $laddr:expr, $ltype:expr, $location:expr, $extra_options:expr) => {{
        use melior::dialect::{
            func,
            llvm::{self, LoadStoreOptions},
        };
        let args = [&[$syscall_ctx.into()][..], &$args[..]].concat();
        $rewriter.create(func::call(
            $context,
            melior::ir::attribute::FlatSymbolRefAttribute::new($context, $symbol),
            &args,
            &rtn_types,
            $location,
        ));
        $rewriter.make(llvm::load(
            $context,
            $laddr,
            $ltype,
            $location,
            $extra_options,
        ))?
    }};
}

/// Macro to store a value at a specified address in memory.
#[macro_export]
macro_rules! store_var {
    ($rewriter:expr, $context:expr, $value:expr, $addr:expr, $location:expr) => {{
        use melior::dialect::llvm::{self, LoadStoreOptions};
        llvm::store(
            $context,
            $value,
            $addr,
            $location,
            LoadStoreOptions::default(),
        )
    }};
    ($rewriter:expr, $context:expr, $value:expr, $addr:expr, $location:expr, $extra_options:expr) => {{
        melior::dialect::llvm::store($context, $value, $addr, $location, $extra_options)
    }};
}

/// Macro to implement conditional branching to revert execution based on specified conditions.
///
/// ```ignore
/// use dora_runtime::ExitStatusCode;
///
/// maybe_revert_here!(op, rewriter, revert_flag, ExitStatus::Return);
/// ```
#[macro_export]
macro_rules! maybe_revert_here {
    ($op:expr, $rewriter:expr, $cond:expr, ExitStatusCode::$variant:ident) => {
        if let Some(block) = $op.block() {
            if let Some(region) = block.parent_region() {
                if let Some(setup_block) = region.first_block() {
                    if let Some(revert_block) = setup_block.next_in_region() {
                        if let Some(insert_point) = $rewriter.get_insert_point() {
                            let next_block = $rewriter.split_block(block, Some(insert_point))?;
                            let builder = $crate::conversion::builder::OpBuilder::new_with_block(
                                $rewriter.context(),
                                block,
                            );
                            builder.create(melior::dialect::cf::cond_br(
                                $rewriter.context(),
                                $cond,
                                &revert_block,
                                &next_block,
                                &[],
                                &[builder.make(
                                    builder.iconst_8(ExitStatusCode::$variant.to_u8() as i8),
                                )?],
                                $rewriter.get_insert_location(),
                            ));
                        }
                    }
                }
            }
        }
    };
    ($op:expr, $rewriter:expr, $cond:expr, $code:expr) => {
        if let Some(block) = $op.block() {
            if let Some(region) = block.parent_region() {
                if let Some(setup_block) = region.first_block() {
                    if let Some(revert_block) = setup_block.next_in_region() {
                        if let Some(insert_point) = $rewriter.get_insert_point() {
                            let next_block = $rewriter.split_block(block, Some(insert_point))?;
                            let builder = $crate::conversion::builder::OpBuilder::new_with_block(
                                $rewriter.context(),
                                block,
                            );
                            builder.create(melior::dialect::cf::cond_br(
                                $rewriter.context(),
                                $cond,
                                &revert_block,
                                &next_block,
                                &[],
                                &[$code],
                                $rewriter.get_insert_location(),
                            ));
                        }
                    }
                }
            }
        }
    };
}

/// Macro to check for memory offset errors during operations and triggers a revert if an out-of-gas condition is detected.
///
/// ```ignore
/// use dora_runtime::ExitSatusCode;
///
/// check_op_oog!(op, rewriter, size);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! check_op_oog {
    ($op:expr, $rewriter:ident, $size:expr) => {
        // Check the memory offset halt error
        let zero = $rewriter.make($rewriter.iconst_64(0))?;
        let overflow =
            $rewriter.make($rewriter.icmp($crate::backend::IntCC::SignedLessThan, $size, zero))?;
        maybe_revert_here!($op, $rewriter, overflow, ExitStatusCode::InvalidOperandOOG);
        let $rewriter =
            $crate::conversion::rewriter::Rewriter::new_with_op($rewriter.context(), *$op);
    };
}

/// Macro to convert a 256-bit unsigned integer to a 64-bit representation while checking for overflow conditions.
///
/// ```ignore
/// use dora_runtime::ExitStatusCode;
///
/// u256_u64!(op, rewriter, size);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! u256_to_u64 {
    ($op:expr, $rewriter:ident, $size:ident) => {
        let overflow = $rewriter.make($rewriter.icmp_imm(
            $crate::backend::IntCC::SignedGreaterThan,
            $size,
            u64::MAX as i64,
        )?)?;
        maybe_revert_here!($op, $rewriter, overflow, ExitStatusCode::InvalidOperandOOG);
        let $rewriter =
            $crate::conversion::rewriter::Rewriter::new_with_op($rewriter.context(), *$op);
        let $size = $rewriter.make(melior::dialect::arith::trunci(
            $size,
            $rewriter.intrinsics.i64_ty,
            $rewriter.get_insert_location(),
        ))?;
        check_op_oog!($op, $rewriter, $size)
    };
}

/// Macro to validate runtime errors and triggers reversion if an error condition is met, ensuring robust error management.
#[macro_export(local_inner_macros)]
macro_rules! check_runtime_error {
    ($op:expr, $rewriter:ident, $error:expr) => {
        // Check the runtime halt error
        let zero = $rewriter.make($rewriter.iconst_8(0))?;
        let has_error =
            $rewriter.make($rewriter.icmp($crate::backend::IntCC::NotEqual, $error, zero))?;
        maybe_revert_here!($op, $rewriter, has_error, $error);
    };
}

/// Macro to verify that the current operation is not within a static call context, reverting if it is.
///
/// ```ignore
/// use dora_runtime::ExitStatusCode;
///
/// ensure_non_staticcall!(op, rewriter, syscall_ctx);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! ensure_non_staticcall {
    ($op:expr, $rewriter:ident, $syscall_ctx: ident) => {
        use dora_runtime::symbols;
        let context = $rewriter.context();
        let location = $rewriter.get_insert_location();
        let ctx_is_static_u8 = $rewriter.make(func::call(
            context,
            melior::ir::attribute::FlatSymbolRefAttribute::new(context, symbols::CTX_IS_STATIC),
            &[$syscall_ctx.into()],
            &[$rewriter.intrinsics.i8_ty],
            location,
        ))?;
        let ctx_is_static = $rewriter.make($rewriter.icmp_imm(
            $crate::backend::IntCC::NotEqual,
            ctx_is_static_u8,
            0,
        )?)?;
        maybe_revert_here!(
            $op,
            $rewriter,
            ctx_is_static,
            ExitStatusCode::StateChangeDuringStaticcall
        );
    };
}

/// Macro to check if there is enough gas available for an operation and manages gas consumption, reverting if out of gas.
///
/// ```ignore
/// use dora_runtime::ExitStatusCode;
///
/// gas_or_fail!(op, rewriter, gas, gas_counter_ptr);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! gas_or_fail {
    ($op:expr, $rewriter:ident, $gas_value:expr, $gas_counter_ptr:expr) => {
        let gas_counter =
            $rewriter.make($rewriter.load($gas_counter_ptr.into(), $rewriter.intrinsics.i64_ty))?;
        let out_of_gas = $rewriter.make(melior::dialect::arith::cmpi(
            $rewriter.context(),
            melior::dialect::arith::CmpiPredicate::Ult,
            gas_counter,
            $gas_value,
            $rewriter.get_insert_location(),
        ))?;
        $rewriter.create(melior::dialect::scf::r#if(
            out_of_gas,
            &[],
            {
                let region = melior::ir::Region::new();
                let block = region.append_block(Block::new(&[]));
                let rewriter = $crate::conversion::rewriter::Rewriter::new_with_block(
                    $rewriter.context(),
                    block,
                );
                rewriter.create(melior::dialect::scf::r#yield(
                    &[],
                    rewriter.get_insert_location(),
                ));
                region
            },
            {
                use melior::dialect::llvm::{self, LoadStoreOptions};
                let region = melior::ir::Region::new();
                let block = region.append_block(Block::new(&[]));
                let rewriter = $crate::conversion::rewriter::Rewriter::new_with_block(
                    $rewriter.context(),
                    block,
                );
                let new_gas_counter = rewriter.make(melior::dialect::arith::subi(
                    gas_counter,
                    $gas_value,
                    rewriter.get_insert_location(),
                ))?;
                rewriter.create(llvm::store(
                    rewriter.context(),
                    new_gas_counter,
                    $gas_counter_ptr,
                    rewriter.get_insert_location(),
                    LoadStoreOptions::default(),
                ));
                rewriter.create(melior::dialect::scf::r#yield(
                    &[],
                    rewriter.get_insert_location(),
                ));
                region
            },
            $rewriter.get_insert_location(),
        ));
        maybe_revert_here!($op, $rewriter, out_of_gas, ExitStatusCode::OutOfGas);
    };
}

/// Macro to facilitate conditional execution by creating branches based on specified conditions within the operation's context.
#[macro_export]
macro_rules! if_here {
    ($op:ident, $rewriter:ident, $cond:expr, $block:expr) => {
        if let Some(block) = $op.block() {
            if let Some(region) = block.parent_region() {
                if let Some(insert_point) = $rewriter.get_insert_point() {
                    use melior::dialect::cf;
                    use $crate::conversion::rewriter::Rewriter;
                    let if_block = region.append_block(Block::new(&[]));
                    let next_block = $rewriter.split_block(block, Some(insert_point))?;
                    let rewriter = Rewriter::new_with_block($rewriter.context(), block);
                    rewriter.create(cf::cond_br(
                        $rewriter.context(),
                        $cond,
                        &if_block,
                        &next_block,
                        &[],
                        &[],
                        $rewriter.get_insert_location(),
                    ));
                    let $op = &if_block.append_operation(cf::br(
                        &next_block,
                        &[],
                        $rewriter.get_insert_location(),
                    ));
                    let $rewriter = Rewriter::new_with_op(rewriter.context(), *$op);
                    $block
                }
            }
        }
    };
}
