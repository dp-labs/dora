use crate::conversion::builder::OpBuilder;
use dora_runtime::symbols;
use melior::{
    dialect::func,
    dialect::llvm::attributes::Linkage,
    ir::{
        attribute::TypeAttribute, r#type::FunctionType, Identifier, Module as MLIRModule, Region,
        Type,
    },
    Context as MLIRContext,
};

pub(crate) fn declare_symbols(context: &MLIRContext, module: &MLIRModule) {
    let block = module.body();
    let builder = OpBuilder::new_with_block(context, block);
    let location = builder.get_insert_location();

    let ptr_type = builder.intrinsics.ptr_ty;
    let uint8 = builder.intrinsics.i8_ty;
    let uint32 = builder.intrinsics.i32_ty;
    let uint64 = builder.intrinsics.i64_ty;
    let attributes = &[(
        Identifier::new(context, "sym_visibility"),
        builder.str_attr("private").into(),
    )];

    // Globals declaration
    builder.create(builder.global(symbols::CTX_IS_STATIC, ptr_type, Linkage::External));

    let function_signatures: &[(&str, &[Type<'_>], &[Type<'_>])] = &[
        (symbols::DEBUG_PRINT, &[uint32], &[]),
        (
            symbols::WRITE_RESULT,
            &[ptr_type, uint64, uint64, uint64, uint8],
            &[],
        ),
        (
            symbols::KECCAK256_HASHER,
            &[ptr_type, uint64, uint64, ptr_type],
            &[],
        ),
        (symbols::GET_CALLDATA_PTR, &[ptr_type], &[ptr_type]),
        (symbols::GET_CALLDATA_SIZE, &[ptr_type], &[uint64]),
        (symbols::GET_CHAINID, &[ptr_type], &[uint64]),
        (symbols::STORE_IN_CALLVALUE_PTR, &[ptr_type, ptr_type], &[]),
        (symbols::STORE_IN_CALLER_PTR, &[ptr_type, ptr_type], &[]),
        (symbols::STORE_IN_GASPRICE_PTR, &[ptr_type, ptr_type], &[]),
        (
            symbols::STORE_IN_SELFBALANCE_PTR,
            &[ptr_type, ptr_type],
            &[],
        ),
        (
            symbols::STORE_IN_BLOBBASEFEE_PTR,
            &[ptr_type, ptr_type],
            &[],
        ),
        (symbols::GET_GASLIMIT, &[ptr_type], &[uint64]),
        (symbols::EXTEND_MEMORY, &[ptr_type, uint64], &[ptr_type]),
        (
            symbols::COPY_CODE_TO_MEMORY,
            &[ptr_type, uint64, uint64, uint64],
            &[],
        ),
        (symbols::STORAGE_READ, &[ptr_type, ptr_type, ptr_type], &[]),
        (
            symbols::STORAGE_WRITE,
            &[ptr_type, ptr_type, ptr_type],
            &[uint64],
        ),
        (symbols::APPEND_LOG, &[ptr_type, uint64, uint64], &[]),
        (
            symbols::APPEND_LOG_ONE_TOPIC,
            &[ptr_type, uint64, uint64, ptr_type],
            &[],
        ),
        (
            symbols::APPEND_LOG_TWO_TOPICS,
            &[ptr_type, uint64, uint64, ptr_type, ptr_type],
            &[],
        ),
        (
            symbols::APPEND_LOG_THREE_TOPICS,
            &[ptr_type, uint64, uint64, ptr_type, ptr_type, ptr_type],
            &[],
        ),
        (
            symbols::APPEND_LOG_FOUR_TOPICS,
            &[
                ptr_type, uint64, uint64, ptr_type, ptr_type, ptr_type, ptr_type,
            ],
            &[],
        ),
        (symbols::GET_ORIGIN, &[ptr_type, ptr_type], &[]),
        (symbols::GET_COINBASE_PTR, &[ptr_type], &[ptr_type]),
        (symbols::GET_BLOCK_NUMBER, &[ptr_type, ptr_type], &[]),
        (
            symbols::GET_CODESIZE_FROM_ADDRESS,
            &[ptr_type, ptr_type],
            &[uint64],
        ),
        (symbols::GET_ADDRESS_PTR, &[ptr_type], &[ptr_type]),
        (symbols::GET_PREVRANDAO, &[ptr_type, ptr_type], &[]),
        (symbols::STORE_IN_TIMESTAMP_PTR, &[ptr_type, ptr_type], &[]),
        (symbols::STORE_IN_BASEFEE_PTR, &[ptr_type, ptr_type], &[]),
        (
            symbols::CALL,
            &[
                ptr_type, uint64, ptr_type, ptr_type, uint64, uint64, uint64, uint64, uint64,
                ptr_type, uint8,
            ],
            &[uint8],
        ),
        (
            symbols::STORE_IN_BALANCE,
            &[ptr_type, ptr_type, ptr_type],
            &[uint64],
        ),
        (
            symbols::COPY_EXT_CODE_TO_MEMORY,
            &[ptr_type, ptr_type, uint64, uint64, uint64],
            &[],
        ),
        (
            symbols::GET_BLOB_HASH_AT_INDEX,
            &[ptr_type, ptr_type, ptr_type],
            &[],
        ),
        (symbols::GET_BLOCK_HASH, &[ptr_type, ptr_type], &[]),
        (symbols::GET_CODE_HASH, &[ptr_type, ptr_type], &[]),
        (
            symbols::CREATE,
            &[ptr_type, uint64, uint64, ptr_type, ptr_type],
            &[uint8],
        ),
        (
            symbols::CREATE2,
            &[ptr_type, uint64, uint64, ptr_type, ptr_type, ptr_type],
            &[uint8],
        ),
        (symbols::GET_RETURN_DATA_SIZE, &[ptr_type], &[uint64]),
        (
            symbols::COPY_RETURN_DATA_INTO_MEMORY,
            &[ptr_type, uint64, uint64, uint64],
            &[],
        ),
        (symbols::SELFDESTRUCT, &[ptr_type, ptr_type], &[uint64]),
        (
            symbols::TRANSIENT_STORAGE_READ,
            &[ptr_type, ptr_type, ptr_type],
            &[],
        ),
        (
            symbols::TRANSIENT_STORAGE_WRITE,
            &[ptr_type, ptr_type, ptr_type],
            &[],
        ),
    ];

    for (name, input_types, output_types) in function_signatures.iter() {
        builder.create(func::func(
            context,
            builder.str_attr(name),
            TypeAttribute::new(FunctionType::new(context, input_types, output_types).into()),
            Region::new(),
            attributes,
            location,
        ));
    }
}
