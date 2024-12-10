#![allow(missing_docs)]

use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
};
use dora_bench::benches::{get_benches, Bench};
use dora_compiler::evm::{CompileOptions, Program};
use dora_compiler::{dora, evm, pass, Compiler, Context, EVMCompiler};
use dora_primitives::{spec::SpecId, Bytes};
use dora_primitives::{Address, Bytecode};
use dora_runtime::context::{Contract, RuntimeContext};
use dora_runtime::env::Env;
use dora_runtime::executor::Executor;
use dora_runtime::host::DummyHost;
use std::hint::black_box;
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
    let spec_id = SpecId::CANCUN;
    let context = Context::new();
    let compiler = EVMCompiler::new(&context);
    let program = Program::from_opcodes(bytecode, spec_id);
    let mut module = compiler
        .compile(
            &program,
            &(),
            &CompileOptions {
                spec_id,
                stack_bound_checks: false,
                gas_metering: false,
            },
        )
        .unwrap();
    // Lowering the EVM dialect to MLIR builtin dialects.
    evm::pass::run(&context.mlir_context, &mut module.mlir_module).unwrap();
    dora::pass::run(
        &context.mlir_context,
        &mut module.mlir_module,
        &dora::pass::PassOptions {
            program_code_size: program.code_size,
            spec_id,
            ..Default::default()
        },
    )
    .unwrap();
    pass::run(&context.mlir_context, &mut module.mlir_module).unwrap();
    debug_assert!(module.mlir_module.as_operation().verify());
    let gas_limit = 2_000_000;
    // New the EVM run environment.
    let mut env: Env = Default::default();
    env.tx.gas_limit = gas_limit;
    env.tx.data = Bytes::from(calldata.to_vec());
    env.tx.transact_to = Address::left_padding_from(&[40]);
    let contract = Contract::new_with_env(&env, Bytecode::from(program.to_opcode()), None);
    let mut host = DummyHost::new(env);
    let mut context = RuntimeContext::new(contract, &mut host, SpecId::CANCUN);
    let executor = Executor::new(module.module(), &context, Default::default());
    let func = executor.get_main_entrypoint();
    let ctx = black_box(&mut context);
    let gas = black_box(gas_limit);

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
