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
use std::mem::offset_of;

impl<'c> ConversionPass<'c> {
    pub(crate) fn chainid(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let ptr_type = rewriter.ptr_ty();
        let result_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::CHAINID),
            &[syscall_ctx.into()],
            &[ptr_type],
            location,
        ))?;
        // We don't need to check for errors here, as no errors will be returned.
        let chainid = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<u64>, value),
            rewriter.intrinsics.i64_ty,
        )?;
        rewriter.make(arith::extui(chainid, rewriter.intrinsics.i256_ty, location))?;
        Ok(())
    }

    pub(crate) fn coinbase(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);
        let uint160 = IntegerType::new(context, 160).into();

        let ptr_type = rewriter.ptr_ty();
        let result_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::COINBASE),
            &[syscall_ctx.into()],
            &[ptr_type],
            location,
        ))?;
        // We don't need to check for errors here, as no errors will be returned.
        let coinbase_ptr = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<*mut u8>, value),
            ptr_type,
        )?;
        let coinbase = rewriter.make(rewriter.load(coinbase_ptr, uint160))?;
        let coinbase = if cfg!(target_endian = "little") {
            rewriter.make(llvm::intr_bswap(coinbase, uint160, location))?
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
            symbols::BLOCK_NUMBER,
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
            symbols::PREVRANDAO,
            &[prevrandao_ptr],
            rewriter.intrinsics.i256_ty,
            location
        );
        Ok(())
    }

    pub(crate) fn gaslimit(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let ptr_type = rewriter.ptr_ty();
        let result_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::GASLIMIT),
            &[syscall_ctx.into()],
            &[ptr_type],
            location,
        ))?;
        // We don't need to check for errors here, as no errors will be returned.
        let gaslimit = rewriter.get_field_value(
            result_ptr,
            offset_of!(dora_runtime::context::RuntimeResult<u64>, value),
            rewriter.intrinsics.i64_ty,
        )?;
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
            symbols::ORIGIN,
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
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::BLOB_HASH,
            &[index_ptr],
            [],
            index_ptr,
            rewriter.intrinsics.i256_ty,
            location
        );
        Ok(())
    }

    pub(crate) fn blobbasefee(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        syscall_ctx!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let blobbasefee_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::STORE_IN_BLOBBASEFEE_PTR,
            &[blobbasefee_ptr],
            [],
            blobbasefee_ptr,
            rewriter.intrinsics.i256_ty,
            location
        );
        Ok(())
    }
}
