use crate::conversion::builder::OpBuilder;
use dora_runtime::symbols;
use melior::{
    Context as MLIRContext,
    dialect::func,
    ir::{
        Identifier, Module as MLIRModule, Region, Type, attribute::TypeAttribute,
        r#type::FunctionType,
    },
};

pub(crate) fn declare_symbols(context: &MLIRContext, module: &MLIRModule) {
    let block = module.body();
    let builder = OpBuilder::new_with_block(context, block);
    let location = builder.get_insert_location();

    let uint8 = builder.i8_ty();
    let uint64 = builder.i64_ty();
    let size_type = builder.isize_ty();
    let ptr_type = builder.ptr_ty();

    let attributes = &[(
        Identifier::new(context, "sym_visibility"),
        builder.str_attr("private").into(),
    )];
    let function_signatures: &[(&str, &[Type<'_>], &[Type<'_>])] = &[
        (symbols::NOP, &[], &[]),
        (
            symbols::TRACING,
            &[ptr_type, uint8, uint64, ptr_type, ptr_type],
            &[],
        ),
        (
            symbols::WRITE_RESULT,
            &[ptr_type, uint64, uint64, uint64, uint8],
            &[],
        ),
        (symbols::CTX_IS_STATIC, &[ptr_type], &[uint8]),
        (symbols::EXP, &[ptr_type, ptr_type, ptr_type], &[]),
        (
            symbols::KECCAK256_HASHER,
            &[ptr_type, uint64, uint64, ptr_type],
            &[],
        ),
        (symbols::CALLDATA, &[ptr_type], &[ptr_type]),
        (symbols::CALLDATA_SIZE, &[ptr_type], &[uint64]),
        (
            symbols::CALLDATA_COPY,
            &[ptr_type, uint64, ptr_type, uint64],
            &[],
        ),
        (symbols::DATA_LOAD, &[ptr_type, ptr_type], &[]),
        (symbols::DATA_SECTION, &[ptr_type], &[ptr_type]),
        (symbols::DATA_SECTION_SIZE, &[ptr_type], &[uint64]),
        (
            symbols::DATA_SECTION_COPY,
            &[ptr_type, uint64, ptr_type, uint64],
            &[],
        ),
        (symbols::CHAINID, &[ptr_type], &[uint64]),
        (symbols::CALLVALUE, &[ptr_type, ptr_type], &[]),
        (symbols::CALLER, &[ptr_type, ptr_type], &[]),
        (symbols::STORE_IN_GASPRICE_PTR, &[ptr_type, ptr_type], &[]),
        (
            symbols::STORE_IN_SELFBALANCE_PTR,
            &[ptr_type, ptr_type],
            &[ptr_type],
        ),
        (
            symbols::STORE_IN_BLOBBASEFEE_PTR,
            &[ptr_type, ptr_type],
            &[],
        ),
        (symbols::STORE_IN_GASLIMIT_PTR, &[ptr_type, ptr_type], &[]),
        (symbols::EXTEND_MEMORY, &[ptr_type, uint64], &[ptr_type]),
        (symbols::MEMORY_PTR, &[ptr_type], &[ptr_type]),
        (symbols::MEMORY_SIZE, &[ptr_type], &[uint64]),
        (
            symbols::CODE_COPY,
            &[ptr_type, uint64, ptr_type, uint64],
            &[],
        ),
        (symbols::SLOAD, &[ptr_type, ptr_type, ptr_type], &[ptr_type]),
        (
            symbols::SSTORE,
            &[ptr_type, ptr_type, ptr_type, uint64],
            &[ptr_type],
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
        (symbols::ORIGIN, &[ptr_type, ptr_type], &[]),
        (symbols::COINBASE, &[ptr_type], &[ptr_type]),
        (symbols::BLOCK_NUMBER, &[ptr_type, ptr_type], &[]),
        (symbols::EXT_CODE_SIZE, &[ptr_type, ptr_type], &[ptr_type]),
        (symbols::ADDRESS, &[ptr_type], &[ptr_type]),
        (symbols::PREVRANDAO, &[ptr_type, ptr_type], &[]),
        (symbols::STORE_IN_TIMESTAMP_PTR, &[ptr_type, ptr_type], &[]),
        (symbols::STORE_IN_BASEFEE_PTR, &[ptr_type, ptr_type], &[]),
        (
            symbols::STORE_IN_BALANCE,
            &[ptr_type, ptr_type],
            &[ptr_type],
        ),
        (
            symbols::EXT_CODE_COPY,
            &[ptr_type, ptr_type, ptr_type, uint64, uint64],
            &[ptr_type],
        ),
        (symbols::BLOB_HASH, &[ptr_type, ptr_type], &[]),
        (symbols::BLOCK_HASH, &[ptr_type, ptr_type], &[ptr_type]),
        (symbols::EXT_CODE_HASH, &[ptr_type, ptr_type], &[ptr_type]),
        (
            symbols::EOFCREATE,
            &[ptr_type, uint8, uint64, uint64, ptr_type, uint64, ptr_type],
            &[ptr_type],
        ),
        (
            symbols::RETURNCONTRACT,
            &[ptr_type, uint8, uint64, uint64, size_type, uint64, uint8],
            &[ptr_type],
        ),
        (
            symbols::CALL,
            &[
                ptr_type, ptr_type, ptr_type, ptr_type, uint64, uint64, uint64, uint64, uint64,
                uint8,
            ],
            &[ptr_type],
        ),
        (
            symbols::CREATE,
            &[ptr_type, uint64, uint64, ptr_type, uint64],
            &[ptr_type],
        ),
        (
            symbols::CREATE2,
            &[ptr_type, uint64, uint64, ptr_type, uint64, ptr_type],
            &[ptr_type],
        ),
        (
            symbols::EXTCALL_ADDR_VALIDATE,
            &[ptr_type, ptr_type],
            &[uint8],
        ),
        (
            symbols::EXTCALL,
            &[ptr_type, ptr_type, ptr_type, uint64, uint64, uint64, uint8],
            &[ptr_type],
        ),
        (symbols::RETURNDATA, &[ptr_type], &[ptr_type]),
        (symbols::RETURNDATA_SIZE, &[ptr_type], &[uint64]),
        (
            symbols::RETURNDATA_COPY,
            &[ptr_type, uint64, ptr_type, uint64],
            &[ptr_type],
        ),
        (symbols::SELFDESTRUCT, &[ptr_type, ptr_type], &[ptr_type]),
        (symbols::TLOAD, &[ptr_type, ptr_type, ptr_type], &[]),
        (symbols::TSTORE, &[ptr_type, ptr_type, ptr_type], &[]),
        (
            symbols::FUNC_STACK_PUSH,
            &[ptr_type, size_type, size_type],
            &[uint8],
        ),
        (symbols::FUNC_STACK_POP, &[ptr_type], &[size_type]),
        (symbols::FUNC_STACK_GROW, &[ptr_type], &[]),
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
