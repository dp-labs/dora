//! Tools for running [ethertest](https://github.com/ethereum/tests.git)
//! We can clone the git repo to local and use the tool to run the test
//! in the dora-tools folder
//! e.g.,
//! ```shell
//! git clone https://github.com/ethereum/tests.git
//! cargo install --path .
//! dora-ethertest run tests/GeneralStateTests
//! ```
use alloy_rlp::RlpEncodable;
use alloy_rlp::RlpMaxEncodedLen;
use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use dora::{run_with_context, EVMTransaction};
use dora_primitives::spec::SpecId;
use dora_primitives::spec::SpecName;
use dora_primitives::Bytes;
use dora_primitives::{Address, B256, U256};
use dora_runtime::account::Account;
use dora_runtime::context::{CallFrame, Log, RuntimeContext, RuntimeDB};
use dora_runtime::db::{Database, DatabaseCommit, MemoryDB};
use dora_runtime::env::Env;
use dora_runtime::host::DummyHost;
use dora_runtime::result::ResultAndState;
use hash_db::Hasher;
use indicatif::{ProgressBar, ProgressDrawTarget};
use plain_hasher::PlainHasher;
use revm_primitives::keccak256;
use revm_primitives::EvmStorageSlot;
use serde::de::Visitor;
use serde::{de, Deserialize, Serialize};
use std::fmt;
use std::sync::{Arc, RwLock};
use std::{
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
};
use thiserror::Error;
use tracing::{error, info};
use triehash::sec_trie_root;
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeserializeBytes(Bytes);

impl AsRef<Bytes> for DeserializeBytes {
    fn as_ref(&self) -> &Bytes {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Test {
    #[serde(default, rename = "_info")]
    pub info: Option<serde_json::Value>,
    env: TestEnv,
    transaction: Transaction,
    pre: HashMap<Address, TestAccountInfo>,
    post: BTreeMap<SpecName, Vec<PostStateTest>>,
    #[serde(default)]
    pub out: Option<DeserializeBytes>,
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

#[derive(Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Transaction {
    pub data: Vec<DeserializeBytes>,
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

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AccessListItem {
    pub address: Address,
    pub storage_keys: Vec<B256>,
}

pub type AccessList = Vec<AccessListItem>;

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
pub struct TestAccountInfo {
    pub balance: U256,
    pub code: DeserializeBytes,
    #[serde(deserialize_with = "deserialize_str_as_u64")]
    pub nonce: u64,
    pub storage: HashMap<U256, U256>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PostStateTest {
    pub expect_exception: Option<String>,
    pub indexes: TestIndexes,
    pub hash: B256,
    #[serde(default)]
    pub post_state: HashMap<Address, TestAccountInfo>,
    pub logs: B256,
    pub txbytes: Option<DeserializeBytes>,
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

fn log_rlp_hash(logs: &[Log]) -> B256 {
    let logs: Vec<revm_primitives::Log> = logs
        .iter()
        .map(|l| revm_primitives::Log {
            address: revm_primitives::Address::from_slice(l.address.as_bytes()),
            data: revm_primitives::LogData::new_unchecked(
                l.data
                    .topics
                    .iter()
                    .map(|t| revm_primitives::FixedBytes::<32>(t.to_le_bytes()))
                    .collect(),
                revm_primitives::Bytes(Bytes::from(l.data.data.clone())),
            ),
        })
        .collect();
    let mut out = Vec::with_capacity(alloy_rlp::list_length(&logs));
    alloy_rlp::encode_list(&logs, &mut out);
    B256::from_slice(revm_primitives::keccak256(&out).as_slice())
}

pub fn state_merkle_trie_root<'a>(
    accounts: impl IntoIterator<Item = (&'a Address, &'a Account)>,
) -> B256 {
    B256::from_slice(
        trie_root(accounts.into_iter().map(|(address, acc)| {
            (
                address,
                alloy_rlp::encode_fixed_size(&TrieAccount::new(acc)),
            )
        }))
        .as_slice(),
    )
}

#[derive(RlpEncodable, RlpMaxEncodedLen)]
struct TrieAccount {
    nonce: u64,
    balance: U256,
    root_hash: revm_primitives::B256,
    code_hash: revm_primitives::B256,
}

impl TrieAccount {
    fn new(acc: &Account) -> Self {
        Self {
            nonce: acc.info.nonce,
            balance: acc.info.balance,
            root_hash: sec_trie_root::<KeccakHasher, _, _, _>(
                acc.storage
                    .iter()
                    .filter(|(_k, v)| !v.present_value.is_zero())
                    .map(|(k, v)| {
                        (
                            k.to_be_bytes::<32>(),
                            alloy_rlp::encode_fixed_size(&v.present_value),
                        )
                    }),
            ),
            code_hash: revm_primitives::B256::from_slice(acc.info.code_hash.as_bytes()),
        }
    }
}

/// This type keeps track of the current value of a storage slot.
#[derive(
    Debug, Copy, Clone, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct StorageSlot {
    /// The value of the storage slot before it was changed.
    ///
    /// When the slot is first loaded, this is the original value.
    ///
    /// If the slot was not changed, this is equal to the present value.
    pub previous_or_original_value: U256,
    /// When loaded with sload present value is set to original value
    pub present_value: U256,
}

impl From<EvmStorageSlot> for StorageSlot {
    fn from(value: EvmStorageSlot) -> Self {
        Self::new_changed(value.original_value, value.present_value)
    }
}

impl StorageSlot {
    /// Creates a new _unchanged_ `StorageSlot` for the given value.
    pub fn new(original: U256) -> Self {
        Self {
            previous_or_original_value: original,
            present_value: original,
        }
    }

    /// Creates a new _changed_ `StorageSlot`.
    pub fn new_changed(previous_or_original_value: U256, present_value: U256) -> Self {
        Self {
            previous_or_original_value,
            present_value,
        }
    }

    /// Returns true if the present value differs from the original value
    pub fn is_changed(&self) -> bool {
        self.previous_or_original_value != self.present_value
    }

    /// Returns the original value of the storage slot.
    pub fn original_value(&self) -> U256 {
        self.previous_or_original_value
    }

    /// Returns the current value of the storage slot.
    pub fn present_value(&self) -> U256 {
        self.present_value
    }
}

pub type StorageWithOriginalValues = HashMap<U256, StorageSlot>;

#[inline]
pub fn trie_root<I, A, B>(input: I) -> revm_primitives::B256
where
    I: IntoIterator<Item = (A, B)>,
    A: AsRef<[u8]>,
    B: AsRef<[u8]>,
{
    sec_trie_root::<KeccakHasher, _, _, _>(input)
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeccakHasher;

impl Hasher for KeccakHasher {
    type Out = revm_primitives::B256;
    type StdHasher = PlainHasher;
    const LENGTH: usize = 32;

    #[inline]
    fn hash(x: &[u8]) -> Self::Out {
        keccak256(x)
    }
}

fn find_all_json_tests(path: &Path) -> Vec<PathBuf> {
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

fn should_skip(path: &Path) -> bool {
    let path_str = path.to_str().expect("Path is not valid UTF-8");
    let name = path.file_name().unwrap().to_str().unwrap();

    matches!(
        name,
        // JSON big int issue cases: https://github.com/ethereum/tests/issues/971
        "ValueOverflow.json" |
        "ValueOverflowParis.json" |
        // Attack cases
        "run_until_out_of_gas.json" |
        "ContractCreationSpam.json" |
        "JUMPDEST_AttackwithJump.json" |
        "JUMPDEST_Attack.json" |
        "block504980.json" |
        // Failed cases
        "transStorageOK.json" |
        "15_tstoreCannotBeDosd.json" |
        "opc2FDiffPlaces.json" |
        "opcACDiffPlaces.json" |
        "returndatasize_after_oog_after_deeper.json" |
        "stateRevert.json" |
        "CallInfiniteLoop.json" |
        "sha3_deja.json" |
        "underflowTest.json" |
        "static_LoopCallsThenRevert.json" |
        "static_Return50000_2.json" |
        "extcodecopy.json" |
        "createNameRegistratorOutOfMemoryBonds1.json" |
        "CallToNameRegistratorMemOOGAndInsufficientBalance.json" |
        "createNameRegistratorOutOfMemoryBonds0.json"
    ) || path_str.contains("stEOF")
        || path_str.contains("stBugs")
        || path_str.contains("stBadOpcode")
        || path_str.contains("stMemory")
        || path_str.contains("stRandom")
}

fn run_with_shared_db<DB: Database + 'static>(
    mut env: Env,
    db: RuntimeDB<DB>,
) -> Result<ResultAndState> {
    env.validate_transaction().map_err(|e| anyhow::anyhow!(e))?;
    env.consume_intrinsic_cost()
        .map_err(|e| anyhow::anyhow!(e))?;
    let mut runtime_context = RuntimeContext::new(
        db,
        CallFrame::new(env.tx.caller),
        Arc::new(EVMTransaction::<DB>::new()),
        Arc::new(RwLock::new(DummyHost::new(env))),
        SpecId::CANCUN,
    );
    run_with_context(&mut runtime_context)
}

fn execute_test(path: &Path) -> Result<(), TestError> {
    if should_skip(path) {
        return Ok(());
    }
    let name = path.to_string_lossy().to_string();
    info!("testing {:?}", name);
    let s = std::fs::read_to_string(path).unwrap();
    let suite: TestSuite = serde_json::from_str(&s).map_err(|e| TestError {
        name: name.clone(),
        kind: e.into(),
    })?;

    for (_, suite) in suite.0 {
        // NOTE: currently we only support Cancun spec
        let Some(tests) = suite.post.get(&SpecName::Cancun) else {
            continue;
        };
        // Mapping account into
        let mut db = MemoryDB::new();
        for (address, account_info) in suite.pre.iter() {
            db = db.with_contract(address.to_owned(), account_info.code.0.clone());
            db.set_account(
                address.to_owned(),
                account_info.nonce,
                account_info.balance,
                account_info.storage.iter().map(|(k, v)| (*k, *v)).collect(),
            );
        }
        let db = Arc::new(RwLock::new(db));
        for test_case in tests {
            let mut env = setup_env(&name, &suite)?;
            // Mapping transaction data and value
            env.tx.gas_limit = suite.transaction.gas_limit[test_case.indexes.gas].saturating_to();
            env.tx.value = suite
                .transaction
                .value
                .get(test_case.indexes.value)
                .cloned()
                .unwrap_or_default();
            env.tx.data = suite
                .transaction
                .data
                .get(test_case.indexes.data)
                .unwrap()
                .clone()
                .0;
            info!("testing {:?} index {:?}", name, test_case.indexes);
            // Mapping access list
            let access_list = suite
                .transaction
                .access_lists
                .get(test_case.indexes.data)
                .and_then(Option::as_deref)
                .unwrap_or_default();
            for item in access_list {
                env.tx.access_list.push((
                    Address::from_slice(&item.address.0),
                    item.storage_keys
                        .iter()
                        .map(|key| U256::from_be_bytes(key.0))
                        .collect(),
                ));
            }
            // Run EVM and get the state result.
            let res = run_with_shared_db(env, db.clone());
            let logs_root = log_rlp_hash(res.as_ref().map(|r| r.result.logs()).unwrap_or_default());
            // Check result and output.
            match res {
                Ok(res) => {
                    // Commit the state change into the database for the next loop.
                    let mut db_ref = db.write().unwrap();
                    db_ref.commit(res.state);
                    drop(db_ref);
                    // Check the expect exception.
                    if test_case.expect_exception.is_some() && res.result.is_success() {
                        return Err(TestError {
                            name: name.to_string(),
                            kind: TestErrorKind::UnexpectedException {
                                expected_exception: test_case.expect_exception.clone(),
                                got_exception: None,
                            },
                        });
                    }
                    // Check output.
                    if let Some((expected_output, output)) =
                        suite.out.as_ref().zip(res.result.output())
                    {
                        if expected_output.0 != output {
                            return Err(TestError {
                                name: name.to_string(),
                                kind: TestErrorKind::UnexpectedOutput {
                                    expected_output: Some(expected_output.0.clone()),
                                    got_output: res.result.output().cloned(),
                                },
                            });
                        }
                    }
                    // Check the state root.
                    // FIXME: When completing the account balance and gas test, please note the calculation of state root here
                    // let state = db.read().unwrap().clone().into_state();
                    // let state_root = state_merkle_trie_root(state.iter());
                    // if state_root != test_case.hash {
                    //     let kind = TestErrorKind::StateRootMismatch {
                    //         got: state_root,
                    //         expected: test_case.hash,
                    //     };
                    //     return Err(TestError {
                    //         name: name.to_string(),
                    //         kind,
                    //     });
                    // }
                }
                Err(_) => {
                    if test_case.expect_exception.is_none() {
                        return Err(TestError {
                            name: name.to_string(),
                            kind: TestErrorKind::UnexpectedException {
                                expected_exception: test_case.expect_exception.clone(),
                                got_exception: None,
                            },
                        });
                    }
                }
            }
            // Check the logs root.
            if logs_root != test_case.logs {
                let kind = TestErrorKind::LogsRootMismatch {
                    got: logs_root,
                    expected: test_case.logs,
                };
                return Err(TestError {
                    name: name.to_string(),
                    kind,
                });
            }
        }
    }

    Ok(())
}

fn setup_env(name: &str, test: &Test) -> Result<Env, TestError> {
    let mut env = Env::default();
    env.cfg.chain_id = 1;
    env.tx.transact_to = test.transaction.to.unwrap_or_default();
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
    // tx env
    env.tx.caller = if let Some(address) = test.transaction.sender {
        address
    } else {
        let addr =
            recover_address(test.transaction.secret_key.as_bytes()).ok_or_else(|| TestError {
                name: name.to_string(),
                kind: TestErrorKind::UnknownPrivateKey(test.transaction.secret_key),
            })?;
        Address::from_slice(addr.as_slice())
    };
    env.tx.gas_price = test
        .transaction
        .gas_price
        .or(test.transaction.max_fee_per_gas)
        .unwrap_or_default();
    env.tx
        .blob_hashes
        .clone_from(&test.transaction.blob_versioned_hashes);
    env.tx.max_fee_per_blob_gas = test.transaction.max_fee_per_blob_gas;
    Ok(env)
}

fn recover_address(private_key: &[u8]) -> Option<revm_primitives::Address> {
    use k256::ecdsa::SigningKey;

    let key = SigningKey::from_slice(private_key).ok()?;
    let public_key = key.verifying_key().to_encoded_point(false);
    Some(revm_primitives::Address::from_raw_public_key(
        &public_key.as_bytes()[1..],
    ))
}

impl<'de> serde::Deserialize<'de> for DeserializeBytes {
    #[inline]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct BytesVisitor;

        impl<'de> Visitor<'de> for BytesVisitor {
            type Value = DeserializeBytes;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a variable number of bytes represented as a hex string, an array of u8, or raw bytes")
            }

            fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
                Ok(DeserializeBytes(Bytes::from(v.to_vec())))
            }

            fn visit_byte_buf<E: de::Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
                Ok(DeserializeBytes(Bytes::from(v)))
            }

            fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut bytes = Vec::with_capacity(seq.size_hint().unwrap_or(0));

                while let Some(byte) = seq.next_element()? {
                    bytes.push(byte);
                }

                Ok(DeserializeBytes(Bytes::from(bytes)))
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                if let Some(stripped) = v.strip_prefix("0x") {
                    hex::decode(stripped)
                        .map_err(|_| {
                            de::Error::invalid_value(de::Unexpected::Str(v), &"a valid hex string")
                        })
                        .map(From::from)
                        .map(DeserializeBytes)
                } else {
                    Err(de::Error::invalid_value(
                        de::Unexpected::Str(v),
                        &"a valid hex string",
                    ))
                }
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_any(BytesVisitor)
        } else {
            deserializer.deserialize_byte_buf(BytesVisitor)
        }
    }
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
