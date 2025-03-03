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
use indicatif::{ProgressBar, ProgressDrawTarget};
use revm::primitives::TxKind;
use serde::Deserialize;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use thiserror::Error;
use tracing::{error, info};
use walkdir::{DirEntry, WalkDir};

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

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Suite {
    env: TestEnv,
    transactions: HashMap<String, Transaction>,
    pre: HashMap<Address, Account>,
    logs: HashMap<String, HashMap<String, ExpectLog>>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct TestEnv {
    pub block_number: U256,
    pub current_coinbase: Address,
    pub current_difficulty: U256,
    pub current_gas_limit: U256,
    pub current_timestamp: U256,
    pub previous_hash: B256,
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

fn find_all_json_tests(path: &Path) -> Vec<PathBuf> {
    let mut paths = if path.is_file() {
        vec![path.to_path_buf()]
    } else {
        WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().extension() == Some("json".as_ref()))
            .map(DirEntry::into_path)
            .collect()
    };
    paths.sort();
    paths
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
    let mut cache_state = revm::CacheState::new(false);
    for (name, suite) in suites {
        for (address, info) in &suite.pre {
            let code_hash = keccak256(info.code.clone());
            if let Some(cc_code_hash) = info.code_hash {
                assert_eq!(cc_code_hash, code_hash);
            }
            let bytecode = revm::primitives::Bytecode::new_raw(info.code.clone());
            let acc_info = revm::primitives::AccountInfo {
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
        let mut state = revm::db::State::builder()
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
            let spec_id = get_block_spec(
                as_u64_saturated!(suite.env.current_timestamp),
                as_u64_saturated!(suite.env.block_number),
            );
            let mut env = Env::default();
            env.cfg.chain_id = 1;
            env.block.number = suite.env.block_number;
            env.block.coinbase = suite.env.current_coinbase;
            env.block.gas_limit = suite.env.current_gas_limit;
            env.block.timestamp = suite.env.current_timestamp;
            env.block.difficulty = suite.env.current_difficulty;
            env.tx.data = tx.data.clone();
            env.tx.data = tx.data.clone();
            env.tx.gas_limit = as_u64_saturated!(tx.gas_limit);
            env.tx.gas_price = tx.gas_price.unwrap_or_default();
            env.tx.nonce = Some(as_u64_saturated!(tx.nonce));
            env.tx.caller = tx.sender.unwrap_or_default();
            env.tx.value = tx.value;
            env.tx.transact_to = match tx.to {
                Some(to) => TxKind::Call(to),
                None => TxKind::Create,
            };
            vm.env = Box::new(env);
            vm.set_spec_id(spec_id);
            info!("testing name: {} tx idx {}", name, idx);
            let mut evm = revm::Evm::builder()
                .with_db(&mut state)
                .with_spec_id(spec_id)
                .modify_cfg_env(|cfg| {
                    cfg.chain_id = 1;
                })
                .modify_block_env(|block| {
                    block.number = suite.env.block_number;
                    block.coinbase = suite.env.current_coinbase;
                    block.gas_limit = suite.env.current_gas_limit;
                    block.timestamp = suite.env.current_timestamp;
                    block.difficulty = suite.env.current_difficulty;
                })
                .modify_tx_env(|etx| {
                    etx.data = tx.data.clone();
                    etx.gas_limit = as_u64_saturated!(tx.gas_limit);
                    etx.gas_price = tx.gas_price.unwrap_or_default();
                    etx.nonce = Some(as_u64_saturated!(tx.nonce));
                    etx.caller = tx.sender.unwrap_or_default();
                    etx.value = tx.value;
                    etx.transact_to = match tx.to {
                        Some(to) => TxKind::Call(to),
                        None => TxKind::Create,
                    };
                })
                .build();
            // Run revm
            let revm_res = evm.transact_commit().unwrap();
            // Run the dora VM and get the state result.
            let dora_res = vm.transact_commit().unwrap();
            info!(
                "testing name: {} tx idx {} gas used {}",
                name,
                idx,
                dora_res.gas_used()
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
    }
    // Checking for total difficulty is more precise but many RPC providers stopped returning it...
    else if block_number >= 15537394 {
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
