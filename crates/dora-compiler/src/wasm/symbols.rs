use crate::conversion::builder::OpBuilder;
use dora_runtime::symbols;
use melior::{
    dialect::func,
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

    let uint32 = builder.i32_ty();
    let uint64 = builder.i64_ty();
    let ptr_type = builder.ptr_ty();

    let attributes = &[(
        Identifier::new(context, "sym_visibility"),
        builder.str_attr("private").into(),
    )];
    let function_signatures: &[(&str, &[Type<'_>], &[Type<'_>])] = &[
        (
            symbols::wasm::TABLE_INIT,
            &[ptr_type, uint32, uint32, uint32, uint32, uint32],
            &[],
        ),
        (
            symbols::wasm::TABLE_COPY,
            &[ptr_type, uint32, uint32, uint32, uint32, uint32],
            &[],
        ),
        (
            symbols::wasm::TABLE_FILL,
            &[ptr_type, uint32, uint32, ptr_type, uint32],
            &[],
        ),
        (symbols::wasm::TABLE_SIZE, &[ptr_type, uint32], &[uint32]),
        (
            symbols::wasm::TABLE_GET,
            &[ptr_type, uint32, uint32],
            &[ptr_type],
        ),
        (
            symbols::wasm::TABLE_SET,
            &[ptr_type, uint32, uint32, ptr_type],
            &[],
        ),
        (
            symbols::wasm::TABLE_GROW,
            &[ptr_type, ptr_type, uint32, uint32],
            &[uint32],
        ),
        (
            symbols::wasm::IMPORTED_TABLE_SIZE,
            &[ptr_type, uint32],
            &[uint32],
        ),
        (
            symbols::wasm::IMPORTED_TABLE_GET,
            &[ptr_type, uint32, uint32],
            &[ptr_type],
        ),
        (
            symbols::wasm::IMPORTED_TABLE_GROW,
            &[ptr_type, ptr_type, uint32, uint32],
            &[uint32],
        ),
        (
            symbols::wasm::MEMORY_INIT,
            &[ptr_type, uint32, uint32, uint32, uint32, uint32],
            &[],
        ),
        (symbols::wasm::MEMORY_SIZE, &[ptr_type, uint32], &[uint32]),
        (
            symbols::wasm::MEMORY_GROW,
            &[ptr_type, uint32, uint32],
            &[uint32],
        ),
        (
            symbols::wasm::MEMORY_COPY,
            &[ptr_type, uint32, uint32, uint32, uint32],
            &[],
        ),
        (
            symbols::wasm::MEMORY_FILL,
            &[ptr_type, uint32, uint32, uint32, uint32],
            &[],
        ),
        (
            symbols::wasm::MEMORY_NOTIFY,
            &[ptr_type, uint32, uint32, uint32],
            &[uint32],
        ),
        (
            symbols::wasm::MEMORY_WAIT32,
            &[ptr_type, uint32, uint32, uint32, uint64],
            &[uint32],
        ),
        (
            symbols::wasm::MEMORY_WAIT64,
            &[ptr_type, uint32, uint32, uint64, uint64],
            &[uint32],
        ),
        (
            symbols::wasm::IMPORTED_MEMORY_SIZE,
            &[ptr_type, uint32],
            &[uint32],
        ),
        (
            symbols::wasm::IMPORTED_MEMORY_GROW,
            &[ptr_type, uint32, uint32],
            &[uint32],
        ),
        (
            symbols::wasm::IMPORTED_MEMORY_COPY,
            &[ptr_type, uint32, uint32, uint32, uint32],
            &[],
        ),
        (
            symbols::wasm::IMPORTED_MEMORY_FILL,
            &[ptr_type, uint32, uint32, uint32, uint32],
            &[],
        ),
        (
            symbols::wasm::IMPORTED_MEMORY_NOTIFY,
            &[ptr_type, uint32, uint32, uint32],
            &[uint32],
        ),
        (
            symbols::wasm::IMPORTED_MEMORY_WAIT32,
            &[ptr_type, uint32, uint32, uint32, uint64],
            &[uint32],
        ),
        (
            symbols::wasm::IMPORTED_MEMORY_WAIT64,
            &[ptr_type, uint32, uint32, uint64, uint64],
            &[uint32],
        ),
        (symbols::wasm::FUNC_REF, &[ptr_type, uint32], &[ptr_type]),
        (symbols::wasm::DATA_DROP, &[ptr_type, uint32], &[]),
        (symbols::wasm::ELEM_DROP, &[ptr_type, uint32], &[]),
        (symbols::wasm::RAISE_TRAP, &[uint32], &[]),
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
