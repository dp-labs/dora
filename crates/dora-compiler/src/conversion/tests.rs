use attribute::IntegerAttribute;
use melior::{
    Context, ContextRef,
    dialect::{DialectHandle, DialectRegistry, arith, func},
    ir::{
        Identifier, Module, OperationRef,
        attribute::{StringAttribute, TypeAttribute},
        r#type::TypeId,
        *,
    },
    pass::{ExternalPass, Pass, PassManager, RunExternalPass, create_external},
    utility::{register_all_dialects, register_all_llvm_translations},
};
use r#type::{FunctionType, IntegerType};

use crate::conversion::{
    builder::OpBuilder,
    rewriter::{DeferredRewriter, Replacer, Rewriter, replace_op},
    walker::walk_operation,
};

fn load_all_dialects(context: &Context) {
    let registry = DialectRegistry::new();
    register_all_dialects(&registry);
    context.append_dialect_registry(&registry);
    context.load_all_available_dialects();
}

fn create_test_context() -> Context {
    let context = Context::new();

    context.attach_diagnostic_handler(|diagnostic| {
        eprintln!("{}", diagnostic);
        true
    });

    load_all_dialects(&context);
    register_all_llvm_translations(&context);

    context
}

#[repr(align(8))]
struct PassId;

fn create_module(context: &Context) -> Module {
    let location = Location::unknown(context);
    let module = Module::new(location);
    let index_type = IntegerType::new(context, 32).into();

    module.body().append_operation(func::func(
        context,
        StringAttribute::new(context, "add"),
        TypeAttribute::new(
            FunctionType::new(context, &[index_type, index_type], &[index_type]).into(),
        ),
        {
            let block = Block::new(&[(index_type, location), (index_type, location)]);

            let sum = block.append_operation(arith::addi(
                block.argument(0).unwrap().into(),
                block.argument(1).unwrap().into(),
                location,
            ));
            let constant = block.append_operation(arith::constant(
                context,
                IntegerAttribute::new(index_type, 0).into(),
                location,
            ));
            let sum = block.append_operation(arith::subi(
                constant.result(0).unwrap().into(),
                sum.result(0).unwrap().into(),
                location,
            ));
            block.append_operation(func::r#return(&[sum.result(0).unwrap().into()], location));

            let region = Region::new();
            region.append_block(block);
            region
        },
        &[],
        location,
    ));
    module
}

#[test]
fn external_pass() {
    static TEST_PASS: PassId = PassId;

    #[derive(Clone, Debug)]
    struct TestPass<'c> {
        context: &'c Context,
        value: i32,
    }

    impl<'c> RunExternalPass<'c> for TestPass<'c> {
        fn construct(&mut self) {
            assert_eq!(self.value, 10);
        }

        fn destruct(&mut self) {
            assert_eq!(self.value, 30);
        }

        fn initialize(&mut self, _context: ContextRef<'c>) {
            assert_eq!(self.value, 10);
            self.value = 20;
        }

        fn run(&mut self, operation: OperationRef<'c, '_>, _pass: ExternalPass<'_>) {
            assert_eq!(self.value, 20);
            self.value = 30;
            assert!(operation.verify());
            assert!(
                operation
                    .region(0)
                    .expect("module has a body")
                    .first_block()
                    .expect("module has a body")
                    .first_operation()
                    .expect("body has a function")
                    .name()
                    == Identifier::new(self.context, "func.func")
            );
        }
    }

    impl TestPass<'_> {
        fn into_pass(self) -> Pass {
            create_external(
                self,
                TypeId::create(&TEST_PASS),
                "test pass",
                "test argument",
                "a test pass",
                "",
                &[DialectHandle::func()],
            )
        }
    }

    let context = create_test_context();
    let mut module = create_module(&context);
    let pass_manager = PassManager::new(&context);
    let test_pass = TestPass {
        context: &context,
        value: 10,
    };
    pass_manager.add_pass(test_pass.into_pass());
    pass_manager.run(&mut module).unwrap();
}

use super::walker::WalkFn;

#[test]
fn test_walker() {
    let context = create_test_context();
    let module = create_module(&context);
    let op = module.as_operation();
    assert!(op.verify());
    let walk_fn: WalkFn = Box::new(|op| {
        println!(
            "Operation name: {}",
            op.name().as_string_ref().as_str().unwrap()
        );
        Ok(())
    });
    walk_operation(op, walk_fn).unwrap();
    assert!(op.verify());
    insta::assert_snapshot!(op);
}

#[test]
fn test_op_builder() {
    // To ensure the op builder live long as the context in the walker function scope.
    let ctx = create_test_context();
    let builder = OpBuilder::new(&ctx);
    let op = builder.iconst_32(1);
    assert!(op.verify());
    insta::assert_snapshot!(op);
}

#[test]
fn test_walker_with_op_builder() {
    let context = create_test_context();
    let module = create_module(&context);
    let op = module.as_operation();
    assert!(op.verify());
    walk_operation(
        op,
        Box::new(|op| {
            if op.name().as_string_ref().as_str().unwrap() == "arith.addi" {
                let builder = OpBuilder::new_with_op(&context, op);
                let loc = builder.get_insert_location();
                let one = builder.create(builder.iconst_32(1));
                let two = builder.create(builder.iconst_32(2));
                let three = builder.create(builder.iconst_32(3));
                let mul = builder.create(arith::muli(
                    one.result(0)?.into(),
                    two.result(0)?.into(),
                    loc,
                ));
                // Insert new ops but not to replace the original op
                let _result = builder.create(arith::muli(
                    mul.result(0)?.into(),
                    three.result(0)?.into(),
                    loc,
                ));
            }
            Ok(())
        }),
    )
    .unwrap();
    assert!(op.verify());
    insta::assert_snapshot!(op);
}

#[test]
fn test_walker_with_rewriter() {
    let context = create_test_context();
    let module = create_module(&context);
    let op = module.as_operation();
    assert!(op.verify());
    walk_operation(
        op,
        Box::new(|op| {
            let rewriter = Rewriter::new_with_op(&context, op);
            let loc = rewriter.get_insert_location();
            if let Some(name) = rewriter.get_insert_op_name() {
                match name.as_str() {
                    "arith.addi" => {
                        let one = rewriter.create(rewriter.iconst_32(1));
                        let two = rewriter.create(rewriter.iconst_32(2));
                        let three = rewriter.create(rewriter.iconst_32(3));
                        let mul = rewriter.create(arith::muli(
                            one.result(0)?.into(),
                            two.result(0)?.into(),
                            loc,
                        ));
                        rewriter.replace_op(
                            op,
                            arith::muli(mul.result(0)?.into(), three.result(0)?.into(), loc),
                        )?;
                    }
                    "arith.subi" => {
                        let one = rewriter.create(rewriter.iconst_32(1));
                        let two = rewriter.create(rewriter.iconst_32(2));
                        let three = rewriter.create(rewriter.iconst_32(3));
                        let divsi = rewriter.create(arith::divsi(
                            one.result(0)?.into(),
                            two.result(0)?.into(),
                            loc,
                        ));
                        rewriter.replace_op(
                            op,
                            arith::muli(divsi.result(0)?.into(), three.result(0)?.into(), loc),
                        )?;
                    }
                    _ => {}
                }
            }
            Ok(())
        }),
    )
    .unwrap();
    assert!(op.verify());
    insta::assert_snapshot!(op);
}

#[test]
fn test_walker_with_deferred_rewriter() {
    let context = create_test_context();
    let module = create_module(&context);
    let op = module.as_operation();
    assert!(op.verify());
    walk_operation(
        op,
        Box::new(|op| {
            let rewriter = DeferredRewriter::new_with_op(&context, op);
            let loc = rewriter.get_insert_location();
            if let Some(name) = rewriter.get_insert_op_name() {
                if name.as_str() == "arith.addi" {
                    let one = rewriter.create(rewriter.iconst_32(1));
                    let two = rewriter.create(rewriter.iconst_32(2));
                    let three = rewriter.create(rewriter.iconst_32(3));
                    let mul = rewriter.create(arith::muli(
                        one.result(0)?.into(),
                        two.result(0)?.into(),
                        loc,
                    ));
                    rewriter.create(arith::muli(
                        mul.result(0)?.into(),
                        three.result(0)?.into(),
                        loc,
                    ));
                }
            }
            Ok(())
        }),
    )
    .unwrap();
    assert!(op.verify());
    insta::assert_snapshot!(op);
}

#[test]
fn test_rewriter() {
    let context = create_test_context();
    let module = create_module(&context);
    let op = module.as_operation();
    op.verify();
    let walk_fn: WalkFn = Box::new(|op: OperationRef<'_, '_>| {
        // Replace "arith.addi" to "arith.subi"
        if op.name().as_string_ref().as_str().unwrap() == "arith.addi" {
            replace_op(
                op,
                arith::subi(op.operand(0)?, op.operand(1)?, op.location()),
            );
        }
        Ok(())
    });
    walk_operation(op, walk_fn).unwrap();
    assert!(op.verify());
    insta::assert_snapshot!(op);
}
