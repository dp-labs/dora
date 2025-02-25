use crate::{
    block_argument, create_var,
    dora::{conversion::ConversionPass, memory},
    errors::Result,
    load_var, operands, rewrite_ctx,
};
use dora_runtime::symbols;
use melior::{
    Context,
    dialect::{arith, func, llvm},
    ir::{
        attribute::{FlatSymbolRefAttribute, TypeAttribute},
        operation::OperationRef,
        r#type::IntegerType,
    },
};

impl ConversionPass<'_> {
    pub(crate) fn chainid(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint64 = rewriter.i64_ty();
        let uint256 = rewriter.i256_ty();

        let chainid = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::CHAINID),
            &[syscall_ctx.into()],
            &[uint64],
            location,
        ))?;
        rewriter.make(arith::extui(chainid, uint256, location))?;
        Ok(())
    }

    pub(crate) fn coinbase(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint160 = IntegerType::new(context, 160).into();
        let uint256 = rewriter.i256_ty();

        let coinbase_ptr = rewriter.make(func::call(
            context,
            FlatSymbolRefAttribute::new(context, symbols::COINBASE),
            &[syscall_ctx.into()],
            &[rewriter.ptr_ty()],
            location,
        ))?;
        let coinbase = rewriter.make(rewriter.load(coinbase_ptr, uint160))?;
        let coinbase = if cfg!(target_endian = "little") {
            rewriter.make(llvm::intr_bswap(coinbase, uint160, location))?
        } else {
            coinbase
        };

        rewriter.make(arith::extui(coinbase, uint256, location))?;
        Ok(())
    }

    pub(crate) fn timestamp(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint256 = rewriter.i256_ty();

        let timestamp_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::STORE_IN_TIMESTAMP_PTR,
            &[timestamp_ptr],
            [],
            timestamp_ptr,
            uint256,
            location
        );
        Ok(())
    }

    pub(crate) fn number(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint256 = rewriter.i256_ty();

        let number_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::BLOCK_NUMBER,
            &[number_ptr],
            [],
            number_ptr,
            uint256,
            location
        );
        Ok(())
    }

    pub(crate) fn prevrandao(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint256 = rewriter.i256_ty();

        let prevrandao_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::PREVRANDAO,
            &[prevrandao_ptr],
            [],
            prevrandao_ptr,
            uint256,
            location
        );
        Ok(())
    }

    pub(crate) fn gaslimit(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint256 = rewriter.i256_ty();

        let gaslimit_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::STORE_IN_GASLIMIT_PTR,
            &[gaslimit_ptr],
            [],
            gaslimit_ptr,
            uint256,
            location
        );
        Ok(())
    }

    pub(crate) fn gasprice(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint256 = rewriter.i256_ty();

        let gasprice_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::STORE_IN_GASPRICE_PTR,
            &[gasprice_ptr],
            [],
            gasprice_ptr,
            uint256,
            location
        );
        Ok(())
    }

    pub(crate) fn basefee(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint256 = rewriter.i256_ty();

        let basefee_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::STORE_IN_BASEFEE_PTR,
            &[basefee_ptr],
            [],
            basefee_ptr,
            uint256,
            location
        );
        Ok(())
    }

    pub(crate) fn origin(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint256 = rewriter.i256_ty();

        let origin_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::ORIGIN,
            &[origin_ptr],
            [],
            origin_ptr,
            uint256,
            location
        );
        Ok(())
    }

    pub(crate) fn blobhash(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        operands!(op, index);
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint256 = rewriter.i256_ty();

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
            uint256,
            location
        );
        Ok(())
    }

    pub(crate) fn blobbasefee(context: &Context, op: &OperationRef<'_, '_>) -> Result<()> {
        block_argument!(op, syscall_ctx);
        rewrite_ctx!(context, op, rewriter, location);

        let uint256 = rewriter.i256_ty();

        let blobbasefee_ptr = create_var!(rewriter, context, location);
        load_var!(
            rewriter,
            context,
            syscall_ctx,
            symbols::STORE_IN_BLOBBASEFEE_PTR,
            &[blobbasefee_ptr],
            [],
            blobbasefee_ptr,
            uint256,
            location
        );
        Ok(())
    }
}
