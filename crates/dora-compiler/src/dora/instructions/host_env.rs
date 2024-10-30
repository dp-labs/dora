use crate::{
    arith_constant,
    conversion::rewriter::DeferredRewriter,
    create_var,
    dora::{conversion::ConversionPass, memory},
    errors::Result,
    load_var, operands, rewrite_ctx, syscall_ctx,
};
use dora_runtime::symbols;
use melior::{
    dialect::{
        arith::{self},
        func,
        llvm::{self, AllocaOptions, LoadStoreOptions},
    },
    ir::{
        attribute::{FlatSymbolRefAttribute, IntegerAttribute, TypeAttribute},
        operation::OperationRef,
        r#type::IntegerType,
    },
    Context,
};

impl<'c> ConversionPass<'c> {
    pub(crate) fn chainid(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let chainid = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::GET_CHAINID),
            &[syscall_ctx.into()],
            &[rewriter.intrinsics.i64_ty],
            location,
        ))?;
        rewriter.make(arith::extui(chainid, rewriter.intrinsics.i256_ty, location))?;
        Ok(())
    }

    pub(crate) fn coinbase(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);
        let uint160 = IntegerType::new(context, 160);

        let coinbase_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::GET_COINBASE_PTR),
            &[syscall_ctx.into()],
            &[rewriter.ptr_ty()],
            location,
        ))?;
        let coinbase = rewriter.make(llvm::load(
            context,
            coinbase_ptr,
            uint160.into(),
            location,
            LoadStoreOptions::new()
                .align(IntegerAttribute::new(IntegerType::new(context, 64).into(), 1).into()),
        ))?;
        let coinbase = if cfg!(target_endian = "little") {
            rewriter.make(llvm::intr_bswap(coinbase, uint160.into(), location))?
        } else {
            coinbase
        };

        rewriter.make(arith::extui(
            coinbase,
            rewriter.intrinsics.i256_ty,
            location,
        ))?;
        Ok(())
    }

    pub(crate) fn timestamp(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let timestamp_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::STORE_IN_TIMESTAMP_PTR,
            &[timestamp_ptr],
            rewriter.intrinsics.i256_ty,
            location
        );
        Ok(())
    }

    pub(crate) fn number(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let number_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::GET_BLOCK_NUMBER,
            &[number_ptr],
            rewriter.intrinsics.i256_ty,
            location
        );
        Ok(())
    }

    pub(crate) fn prevrandao(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let prevrandao_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::GET_PREVRANDAO,
            &[prevrandao_ptr],
            rewriter.intrinsics.i256_ty,
            location
        );
        Ok(())
    }

    pub(crate) fn gaslimit(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let gaslimit = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::GET_GASLIMIT),
            &[syscall_ctx.into()],
            &[rewriter.intrinsics.i64_ty],
            location,
        ))?;
        rewriter.make(arith::extui(
            gaslimit,
            rewriter.intrinsics.i256_ty,
            location,
        ))?;
        Ok(())
    }

    pub(crate) fn gasprice(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let gasprice_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::STORE_IN_GASPRICE_PTR,
            &[gasprice_ptr],
            rewriter.intrinsics.i256_ty,
            location
        );
        Ok(())
    }

    pub(crate) fn basefee(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let basefee_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::STORE_IN_BASEFEE_PTR,
            &[basefee_ptr],
            rewriter.intrinsics.i256_ty,
            location
        );
        Ok(())
    }

    pub(crate) fn origin(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let origin_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::GET_ORIGIN,
            &[origin_ptr],
            rewriter.intrinsics.i256_ty,
            location
        );
        Ok(())
    }

    pub(crate) fn blobhash(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, index);
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let index_ptr =
            memory::allocate_u256_and_assign_value(context, &rewriter, index, location)?;
        let var_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::GET_BLOB_HASH_AT_INDEX,
            &[index_ptr, var_ptr],
            var_ptr,
            rewriter.intrinsics.i256_ty,
            location
        );
        Ok(())
    }

    pub(crate) fn blobbasefee(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let blobbasefee_ptr = create_var!(rewriter, context, location);
        let blobbasefee = load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::STORE_IN_BLOBBASEFEE_PTR,
            &[blobbasefee_ptr],
            rewriter.intrinsics.i128_ty,
            location
        );
        rewriter.make(arith::extui(
            blobbasefee,
            rewriter.intrinsics.i256_ty,
            location,
        ))?;
        Ok(())
    }
}
