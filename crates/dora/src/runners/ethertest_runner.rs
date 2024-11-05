use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use dora_primitives::db::MemoryDb;
use dora_primitives::{Address, Bytecode, B256, U256};
use dora_runtime::env::{Env, TransactTo};
use indicatif::{ProgressBar, ProgressDrawTarget};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::{
    collections::HashMap,
    io::{stderr, stdout},
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, Arc, Mutex},
    time::{Duration, Instant},
};
use thiserror::Error;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run Dora with given parameters
    Run(RunArgs),
}

#[derive(Args)]
struct RunArgs {
    path: Vec<PathBuf>,
}

struct Test {
    env: TestEnv,
    transaction: Transaction,
    pre: HashMap<Address, AccountInfo>,
    post: BTreeMap<SpecName, Vec<PostStateTest>>,
    indexes: TestIndexes,
    expect_exception: Option<String>,
    logs: B256,
    hash: B256,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct TestEnv {
    pub current_coinbase: Address,
    #[serde(default)]
    pub current_difficulty: U256,
    pub current_gas_limit: U256,
    pub current_number: U256,
    pub current_timestamp: U256,
    pub current_base_fee: Option<U256>,
    pub previous_hash: Option<B256>,

    pub current_random: Option<B256>,
    pub current_beacon_root: Option<B256>,
    pub current_withdrawals_root: Option<B256>,

    pub parent_blob_gas_used: Option<U256>,
    pub parent_excess_blob_gas: Option<U256>,
    pub current_excess_blob_gas: Option<U256>,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Transaction {
    pub data: Vec<u8>,
    pub gas_limit: Vec<U256>,
    pub gas_price: Option<U256>,
    pub nonce: U256,
    pub secret_key: B256,
    /// if sender is not present we need to derive it from secret key.
    #[serde(default)]
    pub sender: Option<Address>,
    #[serde(default, deserialize_with = "deserialize_maybe_empty")]
    pub to: Option<Address>,
    pub value: Vec<U256>,
    pub max_fee_per_gas: Option<U256>,
    pub max_priority_fee_per_gas: Option<U256>,
    #[serde(default)]
    pub access_lists: Vec<Option<AccessList>>,
    #[serde(default)]
    pub authorization_list: Vec<Authorization>,
    #[serde(default)]
    pub blob_versioned_hashes: Vec<B256>,
    pub max_fee_per_blob_gas: Option<U256>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Authorization {
    chain_id: U256,
    address: Address,
    nonce: U256,
    v: U256,
    r: U256,
    s: U256,
    signer: Option<Address>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AccountInfo {
    pub balance: U256,
    pub code: Vec<u8>,
    #[serde(deserialize_with = "deserialize_str_as_u64")]
    pub nonce: u64,
    pub storage: HashMap<U256, U256>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Hash)]
pub enum SpecName {
    Frontier,
    FrontierToHomesteadAt5,
    Homestead,
    HomesteadToDaoAt5,
    HomesteadToEIP150At5,
    EIP150,
    EIP158, // EIP-161: State trie clearing
    EIP158ToByzantiumAt5,
    Byzantium,
    ByzantiumToConstantinopleAt5, // SKIPPED
    ByzantiumToConstantinopleFixAt5,
    Constantinople, // SKIPPED
    ConstantinopleFix,
    Istanbul,
    Berlin,
    BerlinToLondonAt5,
    London,
    Paris,
    Merge,
    Shanghai,
    Cancun,
    Prague,
    PragueEOF,
    Osaka, // SKIPPED
    #[serde(other)]
    Unknown,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PostStateTest {
    pub expect_exception: Option<String>,
    /// Indexes
    pub indexes: TestIndexes,
    /// Post state hash
    pub hash: B256,
    /// Post state
    #[serde(default)]
    pub post_state: HashMap<Address, AccountInfo>,
    /// Logs root
    pub logs: B256,
    /// Tx bytes
    pub txbytes: Option<Bytes>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct TestIndexes {
    pub data: usize,
    pub gas: usize,
    pub value: usize,
}

#[derive(Debug, Error)]
#[error("Test {name} failed: {kind}")]
pub struct TestError {
    pub name: String,
    pub kind: TestErrorKind,
}

#[derive(Debug, Error)]
pub enum TestErrorKind {
    #[error("logs root mismatch: got {got}, expected {expected}")]
    LogsRootMismatch { got: String, expected: String },
    #[error("state root mismatch: got {got}, expected {expected}")]
    StateRootMismatch { got: String, expected: String },
    #[error(transparent)]
    SerdeDeserialize(#[from] serde_json::Error),
    #[error("unexpected execution error")]
    ExecutionError,
}

pub fn find_all_json_tests(path: &Path) -> Vec<PathBuf> {
    if path.is_file() {
        vec![path.to_path_buf()]
    } else {
        WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().extension() == Some("json".as_ref()))
            .map(DirEntry::into_path)
            .collect()
    }
}

pub fn execute_test(path: &Path) -> Result<(), TestError> {
    let s = std::fs::read_to_string(path)?;
    let suite= serde_json::from_str(&s).map_err(|e| TestError {
        name: path.to_string_lossy().to_string(),
        kind: e.into(),
    })?;

    for (name, test_case) in suite.0 {
        // Setup EVM and environment
        let mut env = setup_env(&test_case);

        // Execute the test case
        // TODO: EXECUTE TEST WITH POST
    }

    Ok(())
}

fn setup_env(test: &Test) -> Env {
    // Create and configure an `Env` object from the test case data
    let mut env = Env::default();
    // for mainnet
    env.cfg.chain_id = 1;
    // env.cfg.spec_id is set down the road

    // block env
    env.block.number = test.env.current_number;
    env.block.coinbase = test.env.current_coinbase;
    env.block.timestamp = test.env.current_timestamp;
    env.block.gas_limit = test.env.current_gas_limit;
    env.block.basefee = test.env.current_base_fee.unwrap_or_default();
    env.block.difficulty = test.env.current_difficulty;
    // after the Merge prevrandao replaces mix_hash field in block and replaced difficulty opcode in EVM.
    env.block.prevrandao = test.env.current_random;
    // EIP-4844
    if let Some(current_excess_blob_gas) = test.env.current_excess_blob_gas {
        env.block
            .set_blob_excess_gas_and_price(current_excess_blob_gas.to());
    } else if let (Some(parent_blob_gas_used), Some(parent_excess_blob_gas)) = (
        test.env.parent_blob_gas_used,
        test.env.parent_excess_blob_gas,
    ) {
        env.block
            .set_blob_excess_gas_and_price(calc_excess_blob_gas(
                parent_blob_gas_used.to(),
                parent_excess_blob_gas.to(),
            ));
    }

    // tx env
    env.tx.caller = if let Some(address) = test.transaction.sender {
        address
    } else {
        recover_address(unit.transaction.secret_key.as_slice()).ok_or_else(|| TestError {
            name: name.clone(),
            kind: TestErrorKind::UnknownPrivateKey(test.transaction.secret_key),
        })?
    };
    env.tx.gas_price = unit
        .transaction
        .gas_price
        .or(test.transaction.max_fee_per_gas)
        .unwrap_or_default();
    env.tx.gas_priority_fee = test.transaction.max_priority_fee_per_gas;
    // EIP-4844
    env.tx.blob_hashes = test.transaction.blob_versioned_hashes;
    env.tx.max_fee_per_blob_gas = test.transaction.max_fee_per_blob_gas;

    env
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Run(run_args) => {
            Ok(for path in &run_args.path {
                println!("\nRunning tests in {}...", path.display());

                let tests = find_all_json_tests(path);
                let pb = ProgressBar::new(tests.len() as u64);
                pb.set_draw_target(ProgressDrawTarget::stdout());

                for test_path in tests {
                    match execute_test(&test_path) {
                        Ok(_) => pb.inc(1),
                        Err(e) => eprintln!("Test failed: {:?}", e),
                    }
                }

                pb.finish_with_message("All tests completed");
            })
        }
    }
}
