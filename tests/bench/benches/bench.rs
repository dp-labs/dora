#![allow(missing_docs)]

use ::dora::EVMTransaction;
use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
};
use dora_bench::benches::{get_benches, Bench};
use dora_compiler::evm::Program;
use dora_compiler::{dora, evm, pass, Compiler, Context, EVMCompiler};
use dora_primitives::Bytes;
use dora_primitives::{db::MemoryDB, Address, Bytecode};
use dora_runtime::context::RuntimeContext;
use dora_runtime::executor::Executor;
use dora_runtime::host::DummyHost;
use dora_runtime::{context::CallFrame, env::Env};
use std::hint::black_box;
use std::sync::{Arc, RwLock};
use std::time::Duration;

fn bench(c: &mut Criterion) {
    for bench in &get_benches() {
        run_bench(c, bench);
    }
}

fn run_bench(c: &mut Criterion, bench: &Bench) {
    let Bench {
        name,
        bytecode,
        calldata,
        native,
    } = bench;

    let mut g = mk_group(c, name);

    let context = Context::new();
    let compiler = EVMCompiler::new(&context);
    let program = Program::from_opcode(bytecode);
    let mut module = compiler.compile(&program, &()).unwrap();
    // Lowering the EVM dialect to MLIR builtin dialects.
    evm::pass::run(&context.mlir_context, &mut module.mlir_module).unwrap();
    dora::pass::run(
        &context.mlir_context,
        &mut module.mlir_module,
        &dora::pass::PassOptions {
            program_code_size: program.code_size,
        },
    )
    .unwrap();
    pass::run(&context.mlir_context, &mut module.mlir_module).unwrap();
    debug_assert!(module.mlir_module.as_operation().verify());
    let (address, bytecode) = (
        Address::from_low_u64_be(40),
        Bytecode::from(program.to_opcode()),
    );
    // New the EVM run environment.
    let mut env: Env = Default::default();
    env.tx.gas_limit = 999_999;
    env.tx.data = Bytes::from(calldata.to_vec());
    env.tx.transact_to = address;
    let mut context = RuntimeContext::new(
        Arc::new(RwLock::new(
            MemoryDB::default().with_contract(address, bytecode),
        )),
        CallFrame::new(Address::from_low_u64_le(10000)),
        Arc::new(EVMTransaction::<MemoryDB>::new()),
        Arc::new(RwLock::new(DummyHost::new(env))),
    );
    let executor = Executor::new(module.module(), &context, Default::default());
    let func = executor.get_main_entrypoint();
    let ctx = black_box(&mut context);
    let gas = black_box(999_999);

    g.bench_function("dora", |b| b.iter(|| func(ctx, gas)));

    if let Some(native) = *native {
        g.bench_function("native", |b| b.iter(native));
    }

    g.finish();
}

fn mk_group<'a>(c: &'a mut Criterion, name: &str) -> BenchmarkGroup<'a, WallTime> {
    let mut g = c.benchmark_group(name);
    g.sample_size(20);
    g.warm_up_time(Duration::from_secs(2));
    g.measurement_time(Duration::from_secs(5));
    g
}

criterion_group!(benches, bench);
criterion_main!(benches);
