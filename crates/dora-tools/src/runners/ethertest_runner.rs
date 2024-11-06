use alloy_eips::eip2930::AccessList;
use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use dora::run_evm;
use dora_primitives::db::MemoryDb;
use dora_primitives::Bytes;
use dora_primitives::{Address, Bytecode, B256, U256};
use dora_runtime::env::Env;
use indicatif::{ProgressBar, ProgressDrawTarget};
use serde::{de, Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
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
    /// Run Dora with given parameters
    Run(RunArgs),
}

#[derive(Args)]
struct RunArgs {
    path: Vec<PathBuf>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct TestSuite(pub BTreeMap<String, Test>);

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Test {
    #[serde(default, rename = "_info")]
    pub info: Option<serde_json::Value>,
    env: TestEnv,
    transaction: Transaction,
    pre: HashMap<Address, AccountInfo>,
    post: BTreeMap<SpecName, Vec<PostStateTest>>,
    #[serde(default)]
    pub out: Option<Bytes>,
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
    pub data: Vec<Bytes>,
    pub gas_limit: Vec<U256>,
    pub gas_price: Option<U256>,
    pub nonce: U256,
    pub secret_key: B256,
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
    pub code: Bytes,
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
    pub indexes: TestIndexes,
    pub hash: B256,
    #[serde(default)]
    pub post_state: HashMap<Address, AccountInfo>,
    pub logs: B256,
    pub txbytes: Option<Bytes>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TestIndexes {
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
    let s = std::fs::read_to_string(path).unwrap();
    let suite: TestSuite = serde_json::from_str(&s).map_err(|e| TestError {
        name: path.to_string_lossy().to_string(),
        kind: e.into(),
    })?;

    for (_, test) in suite.0 {
        let mut env = setup_env(&test);

        for (address, info) in test.pre {
            let mut db = MemoryDb::new().with_contract(address, Bytecode::from(info.code));

            // post and execution
            for (spec_name, tests) in &test.post {
                if *spec_name == SpecName::Constantinople || *spec_name == SpecName::Osaka {
                    continue;
                }

                for (_index, testcase) in tests.iter().enumerate() {
                    env.tx.gas_limit =
                        test.transaction.gas_limit[testcase.indexes.gas].saturating_to();
                    env.tx.data = test
                        .transaction
                        .data
                        .get(testcase.indexes.data)
                        .unwrap()
                        .clone();
                    // Mapping access list
                    let access_list = test
                        .transaction
                        .access_lists
                        .get(testcase.indexes.data)
                        .and_then(Option::as_deref)
                        .cloned()
                        .unwrap_or_default();
                    for item in access_list {
                        env.tx.access_list.push((
                            Address::from_slice(&item.address.0 .0),
                            item.storage_keys
                                .iter()
                                .map(|key| U256::from_be_bytes(key.0))
                                .collect(),
                        ));
                    }

                    // TODO: env.tx.authorization_list

                    // Transaction to or zero address to create.
                    env.tx.transact_to = test.transaction.to.unwrap_or_default();

                    match run_evm(env.clone(), &mut db) {
                        Ok(result) => {
                            info!("Execution result: {:#?}", result);
                        }
                        Err(e) => {
                            error!("Execution failed: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn setup_env(test: &Test) -> Env {
    let mut env = Env::default();
    env.cfg.chain_id = 1;
    env.block.number = test.env.current_number;
    env.block.coinbase = test.env.current_coinbase;
    env.block.timestamp = test.env.current_timestamp;
    env.block.basefee = test.env.current_base_fee.unwrap_or_default();
    env.block.prevrandao = test.env.current_random;
    if let Some(current_excess_blob_gas) = test.env.current_excess_blob_gas {
        env.block.excess_blob_gas = Some(current_excess_blob_gas.to());
    } else if let (Some(_), Some(parent_excess_blob_gas)) = (
        test.env.parent_blob_gas_used,
        test.env.parent_excess_blob_gas,
    ) {
        env.block.excess_blob_gas = Some(parent_excess_blob_gas.to());
    }
    env.tx.caller = test
        .transaction
        .sender
        .unwrap_or_else(|| panic!("Test error: Transaction sender is None"));
    env.tx.gas_price = test
        .transaction
        .gas_price
        .or(test.transaction.max_fee_per_gas)
        .unwrap_or_default();
    env.tx.blob_hashes.clone_from(&test.transaction.blob_versioned_hashes);
    env.tx.max_fee_per_blob_gas = test.transaction.max_fee_per_blob_gas;
    env
}

pub fn deserialize_str_as_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: de::Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;

    if let Some(stripped) = string.strip_prefix("0x") {
        u64::from_str_radix(stripped, 16)
    } else {
        string.parse()
    }
    .map_err(serde::de::Error::custom)
}

pub fn deserialize_maybe_empty<'de, D>(deserializer: D) -> Result<Option<Address>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;
    if string.is_empty() {
        Ok(None)
    } else {
        string.parse().map_err(de::Error::custom).map(Some)
    }
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run(run_args) => {
            for path in &run_args.path {
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
            }
            Ok(())
        }
    }
}
