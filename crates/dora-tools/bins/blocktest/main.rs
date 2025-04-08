//! Tools for running the historical blocks and transactions on the Ethereum mainnet.
//! e.g.,
//! ```shell
//! cargo install --path .
//! dora-blocktest run data/blocks
//! ```
use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use dora::Env;
use dora::compile_handler;
use dora_primitives::Bytes;
use dora_primitives::keccak256;
use dora_primitives::spec::SpecId;
use dora_primitives::{Address, B256, U256, as_u64_saturated};
use dora_runtime::context::VMContext;
use dora_runtime::db::{Database, MemoryDB};
use dora_runtime::executor::RUNTIME_STACK_SIZE;
use dora_runtime::vm::VM;
use dora_tools::find_all_json_tests;
use indicatif::{ProgressBar, ProgressDrawTarget};
use revm::ExecuteCommitEvm;
use revm::primitives::TxKind;
use revm::{MainBuilder, MainContext};
use serde::Deserialize;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use thiserror::Error;
use tracing::{error, info};

#[derive(Parser)]
#[command(author, version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run Dora block tests with given parameters
    Run(RunArgs),
}

#[derive(Args)]
struct RunArgs {
    path: Vec<PathBuf>,
}

pub type Test = HashMap<String, Suite>;
pub type WSet = HashMap<Address, HashMap<U256, U256>>;

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Suite {
    env: TestEnv,
    // The key denotes the tx index + 1
    transactions: HashMap<String, Transaction>,
    pre: HashMap<Address, Account>,
    // The key denotes the tx index
    logs: HashMap<String, HashMap<String, ExpectLog>>,
    // The key denotes the tx index
    wset: Option<HashMap<String, WSet>>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct TestEnv {
    pub block_number: U256,
    pub current_coinbase: Address,
    pub current_difficulty: U256,
    pub current_gas_limit: U256,
    pub current_timestamp: U256,
    pub previous_hash: B256,
    pub current_base_fee: Option<U256>,
}

#[derive(Debug, Default, PartialEq, Eq, Deserialize)]
struct Transaction {
    pub data: Bytes,
    pub gas_limit: U256,
    pub gas_price: Option<U256>,
    pub nonce: U256,
    #[serde(skip)]
    pub secret_key: B256,
    #[serde(default)]
    pub sender: Option<Address>,
    pub to: Option<Address>,
    pub value: U256,
    pub max_fee_per_gas: Option<U256>,
    pub max_priority_fee_per_gas: Option<U256>,
    pub max_fee_per_blob_gas: Option<U256>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct Account {
    pub balance: U256,
    pub code: Bytes,
    pub code_hash: Option<B256>,
    pub nonce: u64,
    pub storage: HashMap<U256, U256>,
}

#[derive(Debug, Error)]
#[error("Test {name} suite {suite_name:?} failed: {kind}")]
pub struct TestError {
    pub name: String,
    pub suite_name: Option<String>,
    pub kind: TestErrorKind,
}

#[derive(Debug, Error)]
pub enum TestErrorKind {
    #[error("logs root mismatch: got {got}, expected {expected}")]
    LogsRootMismatch { got: B256, expected: B256 },
    #[error("state root mismatch: got {got}, expected {expected}")]
    StateRootMismatch { got: B256, expected: B256 },
    #[error("unknown private key: {0:?}")]
    UnknownPrivateKey(B256),
    #[error(transparent)]
    SerdeDeserialize(#[from] serde_json::Error),
    #[error("unexpected execution error")]
    ExecutionError,
    #[error("unexpected output: got {got_output:?}, expected {expected_output:?}")]
    UnexpectedOutput {
        expected_output: Option<Bytes>,
        got_output: Option<Bytes>,
    },
    #[error("unexpected exception: got {got_exception:?}, expected {expected_exception:?}")]
    UnexpectedException {
        expected_exception: Option<String>,
        got_exception: Option<String>,
    },
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpectLog {
    pub address: Address,
    pub data: Bytes,
    pub topics: Vec<B256>,
    pub block_hash: B256,
    pub block_number: U256,
    pub transaction_hash: B256,
    pub transaction_index: U256,
    pub log_index: U256,
    pub removed: bool,
}

fn execute_test(path: &Path) -> Result<(), TestError> {
    let name = path.to_string_lossy().to_string();
    let s = std::fs::read_to_string(path).unwrap();
    let suites: Test = serde_json::from_str(&s).map_err(|e| TestError {
        name: name.clone(),
        suite_name: None,
        kind: e.into(),
    })?;
    // Create database and insert cache
    let mut db = MemoryDB::default();
    let mut cache_state = revm::database::CacheState::new(false);
    for (name, suite) in suites {
        for (address, info) in &suite.pre {
            let code_hash = keccak256(info.code.clone());
            if let Some(cc_code_hash) = info.code_hash {
                assert_eq!(cc_code_hash, code_hash);
            }
            let bytecode = revm::bytecode::Bytecode::new_raw(info.code.clone());
            let acc_info = revm::state::AccountInfo {
                balance: info.balance,
                code_hash,
                code: Some(bytecode),
                nonce: info.nonce,
            };
            cache_state.insert_account_with_storage(
                *address,
                acc_info,
                info.storage.iter().map(|(k, v)| (*k, *v)).collect(),
            );

            db = db.with_contract(
                address.to_owned(),
                dora_primitives::Bytecode::new(info.code.clone()),
            );
            db.set_account(
                address.to_owned(),
                info.nonce,
                info.balance,
                info.storage.iter().map(|(k, v)| (*k, *v)).collect(),
            );
        }
        let mut cache = cache_state.clone();
        cache.set_state_clear_flag(true);
        let mut state = revm::database::State::builder()
            .with_cached_prestate(cache)
            .with_bundle_update()
            .build();

        let mut vm = VM::new(VMContext::new(
            db.clone(),
            Env::default(),
            SpecId::default(),
            compile_handler(),
        ));

        // Sort transactions by id
        let mut transactions = Vec::with_capacity(suite.transactions.len());
        for _ in 0..suite.transactions.len() {
            transactions.push(Transaction::default());
        }
        for (id, tx) in suite.transactions {
            transactions[id.parse::<usize>().unwrap() - 1] = tx;
        }

        for (idx, tx) in transactions.iter().enumerate() {
            if let Ok(max_count) = std::env::var("DORA_BLOCKTEST_MAX_COUNT") {
                if max_count.parse::<usize>().unwrap() < idx {
                    break;
                }
            }
            let spec_id = get_block_spec(
                as_u64_saturated!(suite.env.current_timestamp),
                as_u64_saturated!(suite.env.block_number),
            );
            let mut env = Env::default();
            env.cfg.chain_id = 1;
            env.block.number = as_u64_saturated!(suite.env.block_number);
            env.block.beneficiary = suite.env.current_coinbase;
            env.block.gas_limit = as_u64_saturated!(suite.env.current_gas_limit);
            env.block.timestamp = as_u64_saturated!(suite.env.current_timestamp);
            env.block.difficulty = suite.env.current_difficulty;
            env.block.basefee = suite
                .env
                .current_base_fee
                .unwrap_or_default()
                .try_into()
                .unwrap_or(u64::MAX);
            env.tx.data = tx.data.clone();
            env.tx.data = tx.data.clone();
            env.tx.gas_limit = as_u64_saturated!(tx.gas_limit);
            env.tx.gas_price = tx
                .gas_price
                .or(tx.max_fee_per_gas)
                .unwrap_or_default()
                .try_into()
                .unwrap_or(u128::MAX);
            env.tx.nonce = as_u64_saturated!(tx.nonce);
            env.tx.caller = tx.sender.unwrap_or_default();
            env.tx.value = tx.value;
            env.tx.kind = match tx.to {
                Some(to) => TxKind::Call(to),
                None => TxKind::Create,
            };
            let gas_priority_fee = if spec_id.is_enabled_in(SpecId::LONDON) {
                Some(
                    tx.max_priority_fee_per_gas
                        .unwrap_or_default()
                        .try_into()
                        .unwrap_or(u128::MAX),
                )
            } else {
                None
            };
            env.tx.gas_priority_fee = gas_priority_fee;
            let _ = env.tx.derive_tx_type();
            vm.env = Box::new(env);
            vm.set_spec_id(spec_id);
            info!("testing name: {} tx idx {}", name, idx);
            let mut evm = revm::Context::mainnet()
                .with_db(&mut state)
                .modify_cfg_chained(|cfg| {
                    cfg.chain_id = 1;
                    cfg.spec = spec_id;
                })
                .modify_block_chained(|block| {
                    block.number = as_u64_saturated!(suite.env.block_number);
                    block.beneficiary = suite.env.current_coinbase;
                    block.gas_limit = as_u64_saturated!(suite.env.current_gas_limit);
                    block.timestamp = as_u64_saturated!(suite.env.current_timestamp);
                    block.difficulty = suite.env.current_difficulty;
                    block.basefee = suite
                        .env
                        .current_base_fee
                        .unwrap_or_default()
                        .try_into()
                        .unwrap_or(u64::MAX);
                })
                .modify_tx_chained(|etx| {
                    etx.data = tx.data.clone();
                    etx.gas_limit = as_u64_saturated!(tx.gas_limit);
                    etx.gas_price = tx
                        .gas_price
                        .or(tx.max_fee_per_gas)
                        .unwrap_or_default()
                        .try_into()
                        .unwrap_or(u128::MAX);
                    etx.nonce = as_u64_saturated!(tx.nonce);
                    etx.caller = tx.sender.unwrap_or_default();
                    etx.value = tx.value;
                    etx.kind = match tx.to {
                        Some(to) => TxKind::Call(to),
                        None => TxKind::Create,
                    };
                    etx.gas_priority_fee = gas_priority_fee;
                    let _ = etx.derive_tx_type();
                })
                .build_mainnet();
            // Run the revm and get the state result.
            let revm_res = evm.replay_commit().unwrap();
            // Run the dora VM and get the state result.
            let dora_res = vm.transact_commit().unwrap();
            info!(
                "testing name: {} tx idx {} result {:?}",
                name, idx, dora_res
            );
            assert_eq!(
                revm_res.logs(),
                dora_res.logs(),
                "name: {} tx idx {}",
                name,
                idx
            );
            assert_eq!(
                revm_res.gas_used(),
                dora_res.gas_used(),
                "name: {} tx idx {}",
                name,
                idx
            );
            let expect_logs = suite.logs.get(&idx.to_string()).unwrap();
            assert_eq!(
                expect_logs.len(),
                dora_res.logs().len(),
                "name: {} tx idx {}",
                name,
                idx
            );
            // Check expected logs
            for (i, log) in dora_res.logs().iter().enumerate() {
                let expect_log = expect_logs.get(&i.to_string()).unwrap();
                assert_eq!(
                    expect_log.data, log.data.data,
                    "name: {} tx idx {} log idx {}",
                    name, idx, i
                );
                assert_eq!(
                    expect_log.topics,
                    log.topics(),
                    "name: {} tx idx {} log idx {}",
                    name,
                    idx,
                    i
                );
                assert_eq!(
                    expect_log.address, log.address,
                    "name: {} tx idx {} log idx {}",
                    name, idx, i
                );
            }
        }
    }
    Ok(())
}

fn get_block_spec(timestamp: u64, block_number: u64) -> SpecId {
    if timestamp >= 1710338135 {
        SpecId::CANCUN
    } else if timestamp >= 1681338455 {
        SpecId::SHANGHAI
    } else if block_number >= 15537394 {
        SpecId::MERGE
    } else if block_number >= 12965000 {
        SpecId::LONDON
    } else if block_number >= 12244000 {
        SpecId::BERLIN
    } else if block_number >= 9069000 {
        SpecId::ISTANBUL
    } else if block_number >= 7280000 {
        SpecId::PETERSBURG
    } else if block_number >= 4370000 {
        SpecId::BYZANTIUM
    } else if block_number >= 2675000 {
        SpecId::SPURIOUS_DRAGON
    } else if block_number >= 2463000 {
        SpecId::TANGERINE
    } else if block_number >= 1150000 {
        SpecId::HOMESTEAD
    } else {
        SpecId::FRONTIER
    }
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run(run_args) => {
            for path in &run_args.path {
                info!("\nRunning tests in {}...", path.display());

                let tests = find_all_json_tests(path);
                let pb = ProgressBar::new(tests.len() as u64);
                pb.set_draw_target(ProgressDrawTarget::stdout());
                let builder = std::thread::Builder::new().stack_size(RUNTIME_STACK_SIZE);
                let handle = builder
                    .spawn(move || {
                        for test_path in tests {
                            match execute_test(&test_path) {
                                Ok(_) => pb.inc(1),
                                Err(e) => error!("Test failed: {:?}", e),
                            }
                        }
                        pb.finish_with_message("All tests completed");
                    })
                    .unwrap();
                handle.join().unwrap();
            }
            Ok(())
        }
    }
}
