#![allow(missing_docs)]

use ::dora::{build_artifact, run_evm};
use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
};
use dora_bench::benches::{get_benches, Bench};
use dora_bench::contract::erc20::ERC20Contract;
use dora_bench::contract::uniswapv3;
use dora_compiler::evm::{CompileOptions, Program};
use dora_compiler::{dora, evm, pass, Compiler, Context, EVMCompiler};
use dora_primitives::{address, fixed_bytes, uint, Address, Bytecode, B256, U256};
use dora_primitives::{spec::SpecId, Bytes};
use dora_runtime::context::{Contract, RuntimeContext, Stack};
use dora_runtime::db::{Database, MemoryDB};
use dora_runtime::env::{Env, TxEnv, TxKind};
use dora_runtime::executor::Executor;
use dora_runtime::host::DummyHost;
use rustc_hash::FxHashMap;
use std::hint::black_box;
use std::time::Duration;

fn bench(c: &mut Criterion) {
    for bench in &get_benches() {
        run_bench(c, bench);
    }
    run_uniswapv3_bench(c);
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
                ..Default::default()
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
    env.tx.transact_to = TxKind::Call(Address::left_padding_from(&[40]));
    env.tx.caller = address!("6666000000000000000000000000000000000000");
    let contract = Contract::new_with_env(&env, Bytecode::from(program.to_opcode()), None);
    let mut host = DummyHost::new(env);
    let mut context = RuntimeContext::new(contract, 1, false, false, &mut host, SpecId::CANCUN);
    let executor = Executor::new(module.module(), Default::default());
    let func = executor.get_main_entrypoint();
    let ctx = black_box(&mut context);

    g.bench_function("dora", |b| {
        b.iter(|| {
            let mut gas = black_box(gas_limit);
            func(ctx, &mut gas, &mut Stack::new(), &mut 0);
            assert!(ctx.status().is_ok());
        })
    });
    if let Some(native) = *native {
        g.bench_function("native", |b| b.iter(native));
    }

    g.finish();
}

fn run_uniswapv3_bench(c: &mut Criterion) {
    let addresses = vec![Address::new(rand::random()); 5];
    let seller = addresses[0];

    let (dai_address, usdc_address) = {
        let x = Address::new(rand::random());
        let y = Address::new(rand::random());
        (std::cmp::min(x, y), std::cmp::max(x, y))
    };
    let pool_init_code_hash = B256::new(rand::random());
    let router_address = Address::new(rand::random());
    let swap_address = Address::new(rand::random());
    let weth_address = Address::new(rand::random());
    let owner = Address::new(rand::random());
    let factory_address = Address::new(rand::random());
    let nonfungible_position_manager_address = Address::new(rand::random());
    let pool_address = uniswapv3::PoolContract::new(dai_address, usdc_address, factory_address)
        .pool_address(factory_address, pool_init_code_hash);

    let weth_account = uniswapv3::WethContract::new().to_db_account();

    let dai_account = ERC20Contract::new("DAI", "DAI", 18, 222_222_000_000_000_000_000_000u128)
        .add_balances(&[pool_address], uint!(111_111_000_000_000_000_000_000_U256))
        .add_balances(&addresses, uint!(1_000_000_000_000_000_000_U256))
        .add_allowances(
            &addresses,
            swap_address,
            uint!(1_000_000_000_000_000_000_U256),
        )
        .to_db_account();

    let usdc_account = ERC20Contract::new("USDC", "USDC", 18, 222_222_000_000_000_000_000_000u128)
        .add_balances(&[pool_address], uint!(111_111_000_000_000_000_000_000_U256))
        .add_balances(&addresses, uint!(1_000_000_000_000_000_000_U256))
        .add_allowances(
            &addresses,
            swap_address,
            uint!(1_000_000_000_000_000_000_U256),
        )
        .to_db_account();

    let factory_account = uniswapv3::FactoryContract::new(owner)
        .add_pool(dai_address, usdc_address, pool_address)
        .to_db_account(factory_address);

    let pool_account = uniswapv3::PoolContract::new(dai_address, usdc_address, factory_address)
        .add_position(
            nonfungible_position_manager_address,
            -600000,
            600000,
            [
                uint!(0x00000000000000000000000000000000000000000000178756e190b388651605_U256),
                uint!(0x0000000000000000000000000000000000000000000000000000000000000000_U256),
                uint!(0x0000000000000000000000000000000000000000000000000000000000000000_U256),
                uint!(0x0000000000000000000000000000000000000000000000000000000000000000_U256),
            ],
        )
        .add_tick(
            -600000,
            [
                uint!(0x000000000000178756e190b388651605000000000000178756e190b388651605_U256),
                uint!(0x0000000000000000000000000000000000000000000000000000000000000000_U256),
                uint!(0x0000000000000000000000000000000000000000000000000000000000000000_U256),
                uint!(0x0100000001000000000000000000000000000000000000000000000000000000_U256),
            ],
        )
        .add_tick(
            600000,
            [
                uint!(0xffffffffffffe878a91e6f4c779ae9fb000000000000178756e190b388651605_U256),
                uint!(0x0000000000000000000000000000000000000000000000000000000000000000_U256),
                uint!(0x0000000000000000000000000000000000000000000000000000000000000000_U256),
                uint!(0x0100000000000000000000000000000000000000000000000000000000000000_U256),
            ],
        )
        .to_db_account(pool_address);

    let router_account =
        uniswapv3::RouterContract::new(weth_address, factory_address, pool_init_code_hash)
            .to_db_account();

    let swap_account =
        uniswapv3::SwapContract::new(router_address, dai_address, usdc_address).to_db_account();

    let state = [
        (weth_address, weth_account),
        (dai_address, dai_account),
        (usdc_address, usdc_account),
        (factory_address, factory_account),
        (pool_address, pool_account),
        (router_address, router_account),
        (swap_address, swap_account),
    ];

    let mut db = MemoryDB::new();
    for (address, info) in state.clone() {
        let artifact = build_artifact::<MemoryDB>(&info.0, SpecId::CANCUN).unwrap();
        db.set_artifact(info.1.bytecode_hash, artifact);
        db = db.with_contract(address, info.0.into());
        db.set_account(address, info.1.nonce, info.1.balance, info.1.storage);
    }

    for address in addresses.clone() {
        db.set_account(
            address,
            1,
            uint!(4_567_000_000_000_000_000_000_U256),
            FxHashMap::default(),
        );
    }

    let env = Env {
        tx: TxEnv {
            caller: seller,
            gas_limit: 2_000_000,
            gas_price: U256::from(0xb2d05e07u64),
            transact_to: TxKind::Call(swap_address),
            data: [
                &fixed_bytes!("c92b0891")[..],
                &B256::from(U256::from(2000))[..],
            ]
            .concat()
            .into(),
            nonce: 1,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut g = mk_group(c, "uniswapv3");
    g.bench_function("dora", |b| {
        b.iter(|| {
            let result = run_evm(env.clone(), db.clone(), SpecId::CANCUN).unwrap();
            assert!(result.result.is_success());
        })
    });

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
