use attribute::{StringAttribute, TypeAttribute};
use melior::dialect::func;
use melior::ir::r#type::IntegerType;
use melior::{Context, dialect::DialectRegistry, ir::*, utility::register_all_dialects};
use r#type::FunctionType;

fn init_registry() -> Context {
    // Initialize the DialectRegistry and register all dialects
    let registry = DialectRegistry::new();
    register_all_dialects(&registry);
    // Create the MLIR context and append the registry
    let context = Context::new();
    context.append_dialect_registry(&registry);
    context.load_all_available_dialects();
    context.set_allow_unregistered_dialects(true);

    context
}

fn test_module<'c>(context: &'c Context, ty: Type<'c>, block: Block) -> Module<'c> {
    let location = Location::unknown(context);
    let module = Module::new(location);
    module.body().append_operation(func::func(
        context,
        StringAttribute::new(context, "test_func"),
        TypeAttribute::new(FunctionType::new(context, &[ty], &[]).into()),
        {
            let region = Region::new();
            region.append_block(block);
            region
        },
        &[],
        location,
    ));
    module
}

macro_rules! wasm_dialect_test_snapshot {
    ($name:ident, $op:ident, => i32) => {
        #[test]
        fn $name() {
            let context = init_registry();
            let location = Location::unknown(&context);
            let i32_type = IntegerType::new(&context, 32).into();
            let block: Block = Block::new(&[(i32_type, location)]);
            block.append_operation(crate::wasm::$op(&context, i32_type, location).into());
            insta::assert_snapshot!(test_module(&context, i32_type, block).as_operation());
        }
    };
    ($name:ident, $op:ident, i32) => {
        #[test]
        fn $name() {
            let context = init_registry();
            let location = Location::unknown(&context);
            let i32_type = IntegerType::new(&context, 32).into();
            let block: Block = Block::new(&[(i32_type, location)]);
            let constant = block.argument(0).unwrap();
            block.append_operation(
                crate::wasm::$op(&context, i32_type, constant.into(), location).into(),
            );
            insta::assert_snapshot!(test_module(&context, i32_type, block).as_operation());
        }
    };
    ($name:ident, $op:ident, i64) => {
        #[test]
        fn $name() {
            let context = init_registry();
            let location = Location::unknown(&context);
            let i64_type = IntegerType::new(&context, 64).into();
            let block: Block = Block::new(&[(i64_type, location)]);
            let constant = block.argument(0).unwrap();
            block.append_operation(
                crate::wasm::$op(&context, i64_type, constant.into(), location).into(),
            );
            insta::assert_snapshot!(test_module(&context, i64_type, block).as_operation());
        }
    };
    ($name:ident, $op:ident, i32, i32) => {
        #[test]
        fn $name() {
            let context = init_registry();
            let location = Location::unknown(&context);
            let i32_type = IntegerType::new(&context, 32).into();
            let block: Block = Block::new(&[(i32_type, location), (i32_type, location)]);
            let constant0 = block.argument(0).unwrap();
            let constant1 = block.argument(1).unwrap();
            block.append_operation(
                crate::wasm::$op(&context, constant0.into(), constant1.into(), location).into(),
            );
            insta::assert_snapshot!(test_module(&context, i32_type, block).as_operation());
        }
    };
    ($name:ident, $op:ident, i32, i32 => i32) => {
        #[test]
        fn $name() {
            let context = init_registry();
            let location = Location::unknown(&context);
            let i32_type = IntegerType::new(&context, 32).into();
            let block: Block = Block::new(&[(i32_type, location), (i32_type, location)]);
            let constant0 = block.argument(0).unwrap();
            let constant1 = block.argument(1).unwrap();
            block.append_operation(
                crate::wasm::$op(
                    &context,
                    i32_type,
                    constant0.into(),
                    constant1.into(),
                    location,
                )
                .into(),
            );
            insta::assert_snapshot!(test_module(&context, i32_type, block).as_operation());
        }
    };
    ($name:ident, $op:ident, i64, i64 => i64) => {
        #[test]
        fn $name() {
            let context = init_registry();
            let location = Location::unknown(&context);
            let i64_type = IntegerType::new(&context, 64).into();
            let block: Block = Block::new(&[(i64_type, location), (i64_type, location)]);
            let constant0 = block.argument(0).unwrap();
            let constant1 = block.argument(1).unwrap();
            block.append_operation(
                crate::wasm::$op(
                    &context,
                    i64_type,
                    constant0.into(),
                    constant1.into(),
                    location,
                )
                .into(),
            );
            insta::assert_snapshot!(test_module(&context, i64_type, block).as_operation());
        }
    };
    ($name:ident, $op:ident, i64, i64 => i32) => {
        #[test]
        fn $name() {
            let context = init_registry();
            let location = Location::unknown(&context);
            let i32_type = IntegerType::new(&context, 32).into();
            let i64_type = IntegerType::new(&context, 64).into();
            let block: Block = Block::new(&[(i64_type, location), (i64_type, location)]);
            let constant0 = block.argument(0).unwrap();
            let constant1 = block.argument(1).unwrap();
            block.append_operation(
                crate::wasm::$op(
                    &context,
                    i32_type,
                    constant0.into(),
                    constant1.into(),
                    location,
                )
                .into(),
            );
            insta::assert_snapshot!(test_module(&context, i64_type, block).as_operation());
        }
    };
}

wasm_dialect_test_snapshot! {
    local_get_i32,
    local_get,
    i32
}

wasm_dialect_test_snapshot! {
    local_set_i32,
    local_set,
    i32, i32
}

wasm_dialect_test_snapshot! {
    global_get_i32,
    global_get,
    i32
}

wasm_dialect_test_snapshot! {
    et_global_i32,
    global_set,
    i32, i32
}

wasm_dialect_test_snapshot! {
    load_i32,
    load,
    i32
}

wasm_dialect_test_snapshot! {
    load_i64,
    load,
    i64
}

wasm_dialect_test_snapshot! {
    store_i32,
    store,
    i32, i32
}

wasm_dialect_test_snapshot! {
    add_i32,
    add,
    i32, i32 => i32
}

wasm_dialect_test_snapshot! {
    add_i64,
    add,
    i64, i64 => i64
}

wasm_dialect_test_snapshot! {
    sub_i32,
    sub,
    i32, i32 => i32
}

wasm_dialect_test_snapshot! {
    sub_i64,
    sub,
    i64, i64 => i64
}

wasm_dialect_test_snapshot! {
    mul_i32,
    mul,
    i32, i32 => i32
}

wasm_dialect_test_snapshot! {
    and_i32,
    and,
    i32, i32 => i32
}

wasm_dialect_test_snapshot! {
    and_i64,
    and,
    i64, i64 => i64
}

wasm_dialect_test_snapshot! {
    or_i32,
    or,
    i32, i32 => i32
}

wasm_dialect_test_snapshot! {
    or_i64,
    or,
    i64, i64 => i64
}

wasm_dialect_test_snapshot! {
    xor_i32,
    xor,
    i32, i32 => i32
}

wasm_dialect_test_snapshot! {
    xor_i64,
    xor,
    i64, i64 => i64
}

wasm_dialect_test_snapshot! {
    eq_i32,
    eq,
    i32, i32 => i32
}

wasm_dialect_test_snapshot! {
    eq_i64,
    eq,
    i64, i64 => i32
}
