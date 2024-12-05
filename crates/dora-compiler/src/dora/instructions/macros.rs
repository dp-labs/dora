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

#[macro_export]
macro_rules! rewrite_ctx {
    ($context:expr, $op:expr, $rewriter:ident, $loc:ident) => {
        let r = DeferredRewriter::new_with_op($context, *$op);
        let l = r.get_insert_location();

        let ($rewriter, $loc) = (r, l);

        scopeguard::defer! {
            $rewriter.remove();
        }
    };
    ($context:expr, $op:expr, $rewriter:ident, $loc:ident, NoDefer) => {
        let r = Rewriter::new_with_op($context, *$op);
        let l = r.get_insert_location();

        let ($rewriter, $loc) = (r, l);
    };
}

#[macro_export]
macro_rules! syscall_ctx {
    ($op:expr, $syscall_ctx:ident) => {
        let func_op = $op.parent_operation().unwrap();
        let region = func_op.region(0).unwrap();
        let func_block = region.first_block().unwrap();
        let $syscall_ctx = func_block.argument(0).unwrap();
    };
}

#[macro_export]
macro_rules! arith_constant {
    ($rewriter:expr, $context:expr, $ty:expr, $value:expr, $location:expr) => {
        arith::constant(
            $context,
            IntegerAttribute::new($ty.into(), $value).into(),
            $location,
        )
    };
}

#[macro_export]
macro_rules! create_var {
    ($rewriter:expr, $context:expr, $location:expr) => {{
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

#[macro_export]
macro_rules! load_var {
    ($rewriter:expr, $context:expr, $arg:expr, $ltype:expr, $location:expr) => {{
        $rewriter.make(llvm::load(
            $context,
            $arg,
            $ltype,
            $location,
            LoadStoreOptions::default(),
        ))?
    }};
    ($rewriter:expr, $context:expr, $syscall_ctx:expr, $symbol:expr, $args:expr, $ltype:expr, $location:expr) => {{
        let ptr_type = $rewriter.ptr_ty();
        let args = [&[$syscall_ctx.into()][..], &$args[..]].concat();
        $rewriter.create(func::call(
            $context,
            FlatSymbolRefAttribute::new($context, $symbol),
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
        let ptr_type = $rewriter.ptr_ty();
        let args = [&[$syscall_ctx.into()][..], &$args[..]].concat();
        $rewriter.create(func::call(
            $context,
            FlatSymbolRefAttribute::new($context, $symbol),
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
        let args = [&[$syscall_ctx.into()][..], &$args[..]].concat();
        $rewriter.create(func::call(
            $context,
            FlatSymbolRefAttribute::new($context, $symbol),
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
        let args = [&[$syscall_ctx.into()][..], &$args[..]].concat();
        $rewriter.create(func::call(
            $context,
            FlatSymbolRefAttribute::new($context, $symbol),
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

#[macro_export]
macro_rules! load_by_addr {
    ($rewriter:expr, $name:expr, $rtn_type:expr) => {
        $rewriter.make($rewriter.load(
            $rewriter.make($rewriter.addressof($name, $rewriter.ptr_ty()))?,
            $rtn_type,
        ))?
    };
}

#[macro_export]
macro_rules! store_var {
    ($rewriter:expr, $context:expr, $value:expr, $addr:expr, $location:expr) => {{
        llvm::store(
            $context,
            $value,
            $addr,
            $location,
            LoadStoreOptions::default(),
        )
    }};
    ($rewriter:expr, $context:expr, $value:expr, $addr:expr, $location:expr, $extra_options:expr) => {{
        llvm::store($context, $value, $addr, $location, $extra_options)
    }};
}

#[macro_export]
macro_rules! maybe_revert_here {
    ($op:expr, $rewriter:expr, $cond:expr, ExitStatusCode::$variant:ident) => {
        if let Some(block) = $op.block() {
            if let Some(region) = block.parent_region() {
                if let Some(setup_block) = region.first_block() {
                    if let Some(revert_block) = setup_block.next_in_region() {
                        if let Some(insert_point) = $rewriter.get_insert_point() {
                            let next_block = $rewriter.split_block(block, Some(insert_point))?;
                            let builder = OpBuilder::new_with_block($rewriter.context(), block);
                            builder.create(cf::cond_br(
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
                            let builder = OpBuilder::new_with_block($rewriter.context(), block);
                            builder.create(cf::cond_br(
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

#[macro_export]
macro_rules! check_op_oog {
    ($op:expr, $rewriter:ident, $size:expr) => {
        // Check the memory offset halt error
        let zero = $rewriter.make($rewriter.iconst_64(0))?;
        let overflow = $rewriter.make($rewriter.icmp(IntCC::SignedLessThan, $size, zero))?;
        maybe_revert_here!($op, $rewriter, overflow, ExitStatusCode::InvalidOperandOOG);
        let $rewriter = Rewriter::new_with_op($rewriter.context(), *$op);
    };
}

#[macro_export]
macro_rules! u256_to_64 {
    ($op:expr, $rewriter:ident, $size:ident) => {
        let $size = $rewriter.make(arith::trunci(
            $size,
            $rewriter.intrinsics.i64_ty,
            $rewriter.get_insert_location(),
        ))?;
        check_op_oog!($op, $rewriter, $size)
    };
}

#[macro_export]
macro_rules! check_runtime_error {
    ($op:expr, $rewriter:ident, $error:expr) => {
        // Check the runtime halt error
        let zero = $rewriter.make($rewriter.iconst_8(0))?;
        let has_error = $rewriter.make($rewriter.icmp(IntCC::NotEqual, $error, zero))?;
        maybe_revert_here!($op, $rewriter, has_error, $error);
    };
}

#[macro_export]
macro_rules! ensure_non_staticcall {
    ($op:expr, $rewriter:ident) => {
        let ctx_is_static_ptr =
            $rewriter.make($rewriter.addressof(CTX_IS_STATIC, $rewriter.ptr_ty()))?;
        let ctx_is_static =
            $rewriter.make($rewriter.load(ctx_is_static_ptr, $rewriter.intrinsics.i1_ty))?;
        maybe_revert_here!(
            $op,
            $rewriter,
            ctx_is_static,
            ExitStatusCode::StateChangeDuringStaticCall
        );
    };
}

#[macro_export]
macro_rules! gas_or_fail {
    ($op:expr, $rewriter:ident, $gas_value:expr) => {
        let gas_counter_ptr =
            $rewriter.make($rewriter.addressof(GAS_COUNTER_GLOBAL, $rewriter.ptr_ty()))?;
        let gas_counter =
            $rewriter.make($rewriter.load(gas_counter_ptr, $rewriter.intrinsics.i64_ty))?;
        let out_of_gas = $rewriter.make(arith::cmpi(
            $rewriter.context(),
            arith::CmpiPredicate::Ult,
            gas_counter,
            $gas_value,
            $rewriter.get_insert_location(),
        ))?;
        $rewriter.create(scf::r#if(
            out_of_gas,
            &[],
            {
                let region = Region::new();
                let block = region.append_block(Block::new(&[]));
                let rewriter = Rewriter::new_with_block($rewriter.context(), block);
                rewriter.create(scf::r#yield(&[], rewriter.get_insert_location()));
                region
            },
            {
                let region = Region::new();
                let block = region.append_block(Block::new(&[]));
                let rewriter = Rewriter::new_with_block($rewriter.context(), block);
                let new_gas_counter = rewriter.make(arith::subi(
                    gas_counter,
                    $gas_value,
                    rewriter.get_insert_location(),
                ))?;
                rewriter.create(melior::dialect::llvm::store(
                    rewriter.context(),
                    new_gas_counter,
                    gas_counter_ptr,
                    rewriter.get_insert_location(),
                    melior::dialect::llvm::LoadStoreOptions::default(),
                ));
                rewriter.create(scf::r#yield(&[], rewriter.get_insert_location()));
                region
            },
            $rewriter.get_insert_location(),
        ));
        maybe_revert_here!($op, $rewriter, out_of_gas, ExitStatusCode::OutOfGas);
    };
}

#[macro_export]
macro_rules! if_here {
    ($op:ident, $rewriter:ident, $cond:expr, $block:expr) => {
        if let Some(block) = $op.block() {
            if let Some(region) = block.parent_region() {
                if let Some(insert_point) = $rewriter.get_insert_point() {
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
