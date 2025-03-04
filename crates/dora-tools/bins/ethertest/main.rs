//! Tools for running [ethertest](https://github.com/ethereum/tests.git)
//! We can clone the git repo to local and use the tool to run the test
//! in the dora-tools folder
//! e.g.,
//! ```shell
//! git clone https://github.com/ethereum/tests.git
//! cargo install --path .
//! dora-ethertest run tests/GeneralStateTests
//! ```
use alloy_rlp::{Decodable, Error as RlpError, Header, RlpEncodable, RlpMaxEncodedLen};
use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use dora::compile_handler;
use dora_primitives::Bytecode;
use dora_primitives::Bytes;
use dora_primitives::EvmStorageSlot;
use dora_primitives::SignedAuthorization;
use dora_primitives::as_u64_saturated;
use dora_primitives::calc_excess_blob_gas;
use dora_primitives::keccak256;
use dora_primitives::{
    AccessList, Address, AuthorizationList, B256, Env, Log, PrimitiveSignature, SpecId, SpecName,
    TxKind, U256,
};
use dora_runtime::account::Account;
use dora_runtime::context::VMContext;
use dora_runtime::db::{Database, MemoryDB};
use dora_runtime::executor::RUNTIME_STACK_SIZE;
use dora_runtime::vm::VM;
use dora_tools::find_all_json_tests;
use hash_db::Hasher;
use indicatif::{ProgressBar, ProgressDrawTarget};
use plain_hasher::PlainHasher;
use serde::{Deserialize, de};
use std::{
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
};
use thiserror::Error;
use tracing::{error, info};
use triehash::sec_trie_root;

/// Gas consumption of a single data blob (== blob byte size)
pub const GAS_PER_BLOB: u64 = 1 << 17;
pub const TARGET_BLOB_NUMBER_PER_BLOCK_CANCUN: u64 = 3;

/// Target consumable blob gas for data blobs per block (for 1559-like pricing)
pub const TARGET_BLOB_GAS_PER_BLOCK_CANCUN: u64 =
    TARGET_BLOB_NUMBER_PER_BLOCK_CANCUN * GAS_PER_BLOB;

#[derive(Parser)]
#[command(author, version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run Dora ether state tests with given parameters
    Run(RunArgs),
}

#[derive(Args)]
struct RunArgs {
    path: Vec<PathBuf>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct TestSuite(pub BTreeMap<String, Test>);

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Test {
    #[serde(default, rename = "_info")]
    pub info: Option<serde_json::Value>,
    env: TestEnv,
    transaction: TestTransaction,
    pre: HashMap<Address, TestAccountInfo>,
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
    pub parent_target_blobs_per_block: Option<U256>,
    pub current_excess_blob_gas: Option<U256>,
}

#[derive(Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TestTransaction {
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
    pub authorization_list: Option<Vec<TestAuthorization>>,
    #[serde(default)]
    pub blob_versioned_hashes: Vec<B256>,
    pub max_fee_per_blob_gas: Option<U256>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestAuthorization {
    #[serde(flatten)]
    inner: SignedAuthorization,
}

impl From<TestAuthorization> for SignedAuthorization {
    fn from(auth: TestAuthorization) -> Self {
        auth.inner
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TestAccountInfo {
    pub balance: U256,
    pub code: Bytes,
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
    pub txbytes: Option<Bytes>,
    /// Output state.
    #[serde(default)]
    state: HashMap<Address, TestAccountInfo>,
}

impl PostStateTest {
    pub fn eip7702_authorization_list(
        &self,
    ) -> Result<Option<AuthorizationList>, alloy_rlp::Error> {
        let Some(txbytes) = self.txbytes.as_ref() else {
            return Ok(None);
        };

        if txbytes.first() == Some(&0x04) {
            let mut txbytes = &txbytes[1..];
            let tx = TxEip7702::decode(&mut txbytes)?;
            return Ok(Some(
                AuthorizationList::Signed(tx.authorization_list).into_recovered(),
            ));
        }

        Ok(None)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxEip7702 {
    /// Added as EIP-155: Simple replay attack protection
    pub chain_id: u64,
    /// A scalar value equal to the number of transactions sent by the sender; formally Tn.
    pub nonce: u64,
    /// A scalar value equal to the number of
    /// Wei to be paid per unit of gas for all computation
    /// costs incurred as a result of the execution of this transaction; formally Tp.
    ///
    /// As ethereum circulation is around 120mil eth as of 2022 that is around
    /// 120000000000000000000000000 wei we are safe to use u128 as its max number is:
    /// 340282366920938463463374607431768211455
    pub gas_limit: u64,
    /// A scalar value equal to the maximum
    /// amount of gas that should be used in executing
    /// this transaction. This is paid up-front, before any
    /// computation is done and may not be increased
    /// later; formally Tg.
    ///
    /// As ethereum circulation is around 120mil eth as of 2022 that is around
    /// 120000000000000000000000000 wei we are safe to use u128 as its max number is:
    /// 340282366920938463463374607431768211455
    ///
    /// This is also known as `GasFeeCap`
    pub max_fee_per_gas: u128,
    /// Max Priority fee that transaction is paying
    ///
    /// As ethereum circulation is around 120mil eth as of 2022 that is around
    /// 120000000000000000000000000 wei we are safe to use u128 as its max number is:
    /// 340282366920938463463374607431768211455
    ///
    /// This is also known as `GasTipCap`
    pub max_priority_fee_per_gas: u128,
    /// The 160-bit address of the message call’s recipient or, for a contract creation
    /// transaction, ∅, used here to denote the only member of B0 ; formally Tt.
    pub to: TxKind,
    /// A scalar value equal to the number of Wei to
    /// be transferred to the message call’s recipient or,
    /// in the case of contract creation, as an endowment
    /// to the newly created account; formally Tv.
    pub value: U256,
    /// The accessList specifies a list of addresses and storage keys;
    /// these addresses and storage keys are added into the `accessed_addresses`
    /// and `accessed_storage_keys` global sets (introduced in EIP-2929).
    /// A gas cost is charged, though at a discount relative to the cost of
    /// accessing outside the list.
    pub access_list: AccessList,
    /// Authorizations are used to temporarily set the code of its signer to
    /// the code referenced by `address`. These also include a `chain_id` (which
    /// can be set to zero and not evaluated) as well as an optional `nonce`.
    pub authorization_list: Vec<SignedAuthorization>,
    /// Input has two uses depending if the transaction `to` field is [`TxKind::Create`] or
    /// [`TxKind::Call`].
    ///
    /// Input as init code, or if `to` is [`TxKind::Create`]: An unlimited size byte array
    /// specifying the EVM-code for the account initialisation procedure `CREATE`
    ///
    /// Input as data, or if `to` is [`TxKind::Call`]: An unlimited size byte array specifying the
    /// input data of the message call, formally Td.
    pub input: Bytes,
    pub signature: PrimitiveSignature,
}

impl TxEip7702 {
    /// Decodes the inner [`TxEip7702`] fields from RLP bytes.
    ///
    /// NOTE: This assumes a RLP header has already been decoded, and _just_ decodes the following
    /// RLP fields in the following order:
    ///
    /// - `chain_id`
    /// - `nonce`
    /// - `gas_price`
    /// - `gas_limit`
    /// - `to`
    /// - `value`
    /// - `data` (`input`)
    /// - `access_list`
    /// - `authorization_list`
    fn decode_inner(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        Ok(Self {
            chain_id: Decodable::decode(buf)?,
            nonce: Decodable::decode(buf)?,
            max_priority_fee_per_gas: Decodable::decode(buf)?,
            max_fee_per_gas: Decodable::decode(buf)?,
            gas_limit: Decodable::decode(buf)?,
            to: Decodable::decode(buf)?,
            value: Decodable::decode(buf)?,
            input: Decodable::decode(buf)?,
            access_list: Decodable::decode(buf)?,
            authorization_list: Decodable::decode(buf)?,
            signature: PrimitiveSignature::decode_rlp_vrs(buf, bool::decode)?,
        })
    }

    pub fn decode(data: &mut &[u8]) -> alloy_rlp::Result<Self> {
        // decode the list header for the rest of the transaction
        let header = Header::decode(data)?;
        if !header.list {
            return Err(RlpError::Custom(
                "typed tx fields must be encoded as a list",
            ));
        }
        let tx = TxEip7702::decode_inner(data)?;
        Ok(tx)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TestIndexes {
    pub data: usize,
    pub gas: usize,
    pub value: usize,
}

#[derive(Debug, Error)]
#[error("Test {name} suite {suite_name:?} index {indexs:?} failed: {kind}")]
pub struct TestError {
    pub name: String,
    pub suite_name: Option<String>,
    pub indexs: Option<TestIndexes>,
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
    let mut out = Vec::with_capacity(alloy_rlp::list_length(logs));
    alloy_rlp::encode_list(logs, &mut out);
    B256::from_slice(dora_primitives::keccak256(&out).as_slice())
}

pub fn state_merkle_trie_root<'a>(
    accounts: impl IntoIterator<Item = (&'a Address, &'a Account)>,
) -> B256 {
    B256::from_slice(
        trie_root(accounts.into_iter().map(|(address, acc)| {
            (
                address,
                alloy_rlp::encode_fixed_size(&TrieAccount::new(address, acc)),
            )
        }))
        .as_slice(),
    )
}

#[derive(RlpEncodable, RlpMaxEncodedLen)]
struct TrieAccount {
    nonce: u64,
    balance: U256,
    root_hash: B256,
    code_hash: B256,
}

impl TrieAccount {
    fn new(_address: &Address, acc: &Account) -> Self {
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
            code_hash: acc.info.code_hash,
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
pub fn trie_root<I, A, B>(input: I) -> dora_primitives::B256
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
    type Out = dora_primitives::B256;
    type StdHasher = PlainHasher;
    const LENGTH: usize = 32;

    #[inline]
    fn hash(x: &[u8]) -> Self::Out {
        keccak256(x)
    }
}

fn should_skip(path: &Path) -> bool {
    let path_str = path.to_str().expect("Path is not valid UTF-8");
    let name = path.file_name().unwrap().to_str().unwrap();

    matches!(
        name,
        // JSON big int issue cases: https://github.com/ethereum/tests/issues/971
        "ValueOverflow.json"
        | "ValueOverflowParis.json"

        // Precompiles having storage is not possible
        | "RevertPrecompiledTouch_storage.json"
        | "RevertPrecompiledTouch.json"

        // Txbyte is of type 02 and we don't parse tx bytes for this test to fail.
        | "typeTwoBerlin.json"

        // Skip test where basefee/accesslist/difficulty is present but it shouldn't be supported in
        // London/Berlin/TheMerge. https://github.com/ethereum/tests/blob/5b7e1ab3ffaf026d99d20b17bb30f533a2c80c8b/GeneralStateTests/stExample/eip1559.json#L130
        // It is expected to not execute these tests.
        | "basefeeExample.json"
        | "eip1559.json"
        | "mergeTest.json"

        // Test with some storage check.
        | "RevertInCreateInInit_Paris.json"
        | "RevertInCreateInInit.json"
        | "dynamicAccountOverwriteEmpty.json"
        | "dynamicAccountOverwriteEmpty_Paris.json"
        | "RevertInCreateInInitCreate2Paris.json"
        | "create2collisionStorage.json"
        | "RevertInCreateInInitCreate2.json"
        | "create2collisionStorageParis.json"
        | "InitCollision.json"
        | "InitCollisionParis.json"

        // Skip evmone statetest
        | "initcode_transaction_before_prague.json"
        | "invalid_tx_non_existing_sender.json"
        | "tx_non_existing_sender.json"
        | "block_apply_withdrawal.json"
        | "block_apply_ommers_reward.json"
        | "known_block_hash.json"
        | "eip7516_blob_base_fee.json"
        | "create_tx_collision_storage.json"
        | "create_collision_storage.json"
    ) ||// Temporarily skip EOF test suites: https://github.com/dp-labs/dora/issues/5
        path_str.contains("stEOF")
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
        suite_name: None,
        indexs: None,
        kind: e.into(),
    })?;

    for (suite_name, suite) in suite.0 {
        // Mapping account into
        let mut db = MemoryDB::new();
        // Revm comparative test
        let mut cache_state = revm::CacheState::new(false);
        for (address, info) in suite.pre.iter() {
            let code_hash = keccak256(info.code.clone());
            db = db.with_contract(address.to_owned(), Bytecode::new(info.code.clone()));
            db.set_account(
                address.to_owned(),
                info.nonce,
                info.balance,
                info.storage.iter().map(|(k, v)| (*k, *v)).collect(),
            );

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
        }
        let mut cache = cache_state.clone();
        cache.set_state_clear_flag(true);
        let mut state = revm::db::State::builder()
            .with_cached_prestate(cache)
            .with_bundle_update()
            .build();

        // post and execution
        for (spec_name, tests) in &suite.post {
            // Constantinople was immediately extended by Petersburg.
            // There isn't any production Constantinople transaction
            // so we don't support it and skip right to Petersburg.
            if spec_name == &SpecName::Constantinople {
                continue;
            }
            let spec_id = spec_name.to_spec_id();
            for test_case in tests {
                let mut env = setup_env(&name, &suite, spec_id)?;
                if spec_id.is_enabled_in(SpecId::MERGE) && env.block.prevrandao.is_none() {
                    // if spec is merge and prevrandao is not set, set it to default
                    env.block.prevrandao = Some(B256::default());
                }
                // Mapping transaction data and value
                env.tx.gas_limit =
                    suite.transaction.gas_limit[test_case.indexes.gas].saturating_to();
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
                    .clone();
                env.tx.nonce = Some(as_u64_saturated!(suite.transaction.nonce));
                info!(
                    "testing {:?} suite {:?} index {:?}",
                    name, suite_name, test_case.indexes
                );

                env.tx.access_list = suite
                    .transaction
                    .access_lists
                    .get(test_case.indexes.data)
                    .and_then(Option::as_deref)
                    .cloned()
                    .unwrap_or_default();

                let Ok(auth_list) = test_case.eip7702_authorization_list() else {
                    continue;
                };
                env.tx.authorization_list = auth_list;

                // Revm comparative test
                let mut evm = revm::Evm::builder()
                    .with_db(&mut state)
                    .with_spec_id(spec_id)
                    .modify_cfg_env(|cfg| {
                        cfg.chain_id = 1;
                    })
                    .modify_block_env(|block| {
                        block.number = suite.env.current_number;
                        block.coinbase = suite.env.current_coinbase;
                        block.gas_limit = suite.env.current_gas_limit;
                        block.timestamp = suite.env.current_timestamp;
                        block.difficulty = suite.env.current_difficulty;
                    })
                    .modify_tx_env(|etx| {
                        etx.data = suite
                            .transaction
                            .data
                            .get(test_case.indexes.data)
                            .unwrap()
                            .clone();
                        etx.gas_limit =
                            suite.transaction.gas_limit[test_case.indexes.gas].saturating_to();
                        etx.gas_price = suite.transaction.gas_price.unwrap_or_default();
                        etx.nonce = Some(as_u64_saturated!(suite.transaction.nonce));
                        etx.caller = suite.transaction.sender.unwrap_or_default();
                        etx.value = suite
                            .transaction
                            .value
                            .get(test_case.indexes.value)
                            .cloned()
                            .unwrap_or_default();
                        etx.transact_to = match suite.transaction.to {
                            Some(to) => TxKind::Call(to),
                            None => TxKind::Create,
                        };
                    })
                    // .with_external_context(
                    //     revm::inspectors::TracerEip3155::new(Box::new(std::io::stdout()))
                    //         .without_summary(),
                    // )
                    // .append_handler_register(revm::inspector_handle_register)
                    .build();
                let _ = evm.transact_commit();
                // Run the VM and get the state result.
                let mut vm = VM::new(VMContext::new(db.clone(), env, spec_id, compile_handler()));
                let res = vm.transact_commit();

                let logs_root = log_rlp_hash(res.as_ref().map(|r| r.logs()).unwrap_or_default());
                // Check result and output.
                match res {
                    Ok(res) => {
                        // Check the expect exception.
                        if test_case.expect_exception.is_some() && res.is_success() {
                            return Err(TestError {
                                name: name.to_string(),
                                suite_name: Some(suite_name.to_string()),
                                indexs: Some(test_case.indexes.clone()),
                                kind: TestErrorKind::UnexpectedException {
                                    expected_exception: test_case.expect_exception.clone(),
                                    got_exception: None,
                                },
                            });
                        }
                        // Check output.
                        if let Some((expected_output, output)) =
                            suite.out.as_ref().zip(res.output())
                        {
                            if expected_output != output {
                                return Err(TestError {
                                    name: name.to_string(),
                                    suite_name: Some(suite_name.to_string()),
                                    indexs: Some(test_case.indexes.clone()),
                                    kind: TestErrorKind::UnexpectedOutput {
                                        expected_output: Some(expected_output.clone()),
                                        got_output: res.output().cloned(),
                                    },
                                });
                            }
                        }
                        // Check the state root.
                        let state = vm.db.clone().into_state();
                        let state = state.iter().filter(|(_, acc)| {
                            !acc.is_loaded_as_not_existing() || acc.is_touched() && !acc.is_empty()
                        });
                        let state_root = state_merkle_trie_root(state);
                        if state_root != test_case.hash {
                            let kind = TestErrorKind::StateRootMismatch {
                                got: state_root,
                                expected: test_case.hash,
                            };
                            return Err(TestError {
                                name: name.to_string(),
                                suite_name: Some(suite_name.to_string()),
                                indexs: Some(test_case.indexes.clone()),
                                kind,
                            });
                        }
                    }
                    Err(error) => {
                        if test_case.expect_exception.is_none() {
                            return Err(TestError {
                                name: name.to_string(),
                                suite_name: Some(suite_name.to_string()),
                                indexs: Some(test_case.indexes.clone()),
                                kind: TestErrorKind::UnexpectedException {
                                    expected_exception: test_case.expect_exception.clone(),
                                    got_exception: Some(error.to_string()),
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
                        suite_name: Some(suite_name.to_string()),
                        indexs: Some(test_case.indexes.clone()),
                        kind,
                    });
                }
            }
        }
    }

    Ok(())
}

fn setup_env(name: &str, test: &Test, spec_id: SpecId) -> Result<Env, TestError> {
    let mut env = Env::default();
    env.cfg.chain_id = 1;
    env.block.number = test.env.current_number;
    env.block.coinbase = test.env.current_coinbase;
    env.block.gas_limit = test.env.current_gas_limit;
    env.block.timestamp = test.env.current_timestamp;
    env.block.basefee = test.env.current_base_fee.unwrap_or_default();
    env.block.difficulty = test.env.current_difficulty;
    env.block.prevrandao = test.env.current_random;
    // EIP-4844
    if let Some(current_excess_blob_gas) = test.env.current_excess_blob_gas {
        env.block.set_blob_excess_gas_and_price(
            current_excess_blob_gas.to(),
            spec_id.is_enabled_in(SpecId::PRAGUE),
        );
    } else if let (Some(parent_blob_gas_used), Some(parent_excess_blob_gas)) = (
        test.env.parent_blob_gas_used,
        test.env.parent_excess_blob_gas,
    ) {
        env.block.set_blob_excess_gas_and_price(
            calc_excess_blob_gas(
                parent_blob_gas_used.to(),
                parent_excess_blob_gas.to(),
                test.env
                    .parent_target_blobs_per_block
                    .map(|i| i.to())
                    .unwrap_or(TARGET_BLOB_GAS_PER_BLOCK_CANCUN),
            ),
            spec_id.is_enabled_in(SpecId::PRAGUE),
        );
    }
    // tx env
    let to = match test.transaction.to {
        Some(addr) => TxKind::Call(addr),
        None => TxKind::Create,
    };
    env.tx.transact_to = to;
    env.tx.caller = if let Some(address) = test.transaction.sender {
        address
    } else {
        let addr = recover_address(&test.transaction.secret_key.0).ok_or_else(|| TestError {
            name: name.to_string(),
            suite_name: None,
            indexs: None,
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

fn recover_address(private_key: &[u8]) -> Option<Address> {
    use k256::ecdsa::SigningKey;

    let key = SigningKey::from_slice(private_key).ok()?;
    let public_key = key.verifying_key().to_encoded_point(false);
    Some(Address::from_raw_public_key(&public_key.as_bytes()[1..]))
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
