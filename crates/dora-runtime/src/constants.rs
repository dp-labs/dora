/// Max stack size
pub const MAX_STACK_SIZE: usize = 1024;
/// VM call stack limit.
pub const CALL_STACK_LIMIT: usize = 1024;
/// MLIR call entry point name.
pub const ENTRYPOINT: &str = "call";

pub mod env {
    pub const DORA_TRACING: &str = "DORA_TRACING";
    pub const DORA_DISABLE_CONSOLE: &str = "DORA_DISABLE_CONSOLE";
}

pub mod gas_cost {
    pub const KECCAK256_WORD_COST: u64 = 6;
    pub const COPY_WORD_COST: u64 = 3;
    pub const CALLVALUE: u64 = 9000;
    pub const NEWACCOUNT: u64 = 25000;
    pub const BLOCKHASH: i64 = 20;
    pub const CODEDEPOSIT: u64 = 200;
    pub const CREATE: i64 = 32_000;
    pub const SELFDESTRUCT: i64 = 5_000;
    /// EIP-3860 : Limit and meter initcode
    pub const INITCODE_WORD_COST: u64 = 2;
    /// EIP-170: Contract code size limit
    /// By default the limit is `0x6000` (~25kb)
    pub const MAX_CODE_SIZE: usize = 0x6000;
    pub const MAX_INITCODE_SIZE: usize = 2 * MAX_CODE_SIZE;
    /// EIP-1884: Repricing for trie-size-dependent opcodes
    pub const INSTANBUL_SLOAD_GAS: u64 = 800;
    pub const SSTORE_SET: u64 = 20000;
    pub const SSTORE_RESET: u64 = 5000;
    pub const REFUND_SSTORE_CLEARS: i64 = 15000;
    pub const TRANSACTION_ZERO_DATA: u64 = 4;
    pub const ACCESS_LIST_ADDRESS: u64 = 2400;
    pub const ACCESS_LIST_STORAGE_KEY: u64 = 1900;
    pub const COLD_SLOAD_COST: u64 = 2100;
    pub const COLD_ACCOUNT_ACCESS_COST: u64 = 2600;
    pub const WARM_SLOAD_COST: u64 = 100;
    pub const WARM_SSTORE_RESET: u64 = SSTORE_RESET - COLD_SLOAD_COST;
    pub const CALL_STIPEND: u64 = 2300;
    pub const MIN_CALLEE_GAS: u64 = CALL_STIPEND;
}

// Float truncation constants
pub const LEF32_GEQ_I32_MIN: u64 = i32::MIN as u64;
pub const GEF32_LEQ_I32_MAX: u64 = 0x4eff_ffff; // Max f32 bits for i32
pub const LEF64_GEQ_I32_MIN: u64 = i32::MIN as u64;
pub const GEF64_LEQ_I32_MAX: u64 = i32::MAX as u64;
pub const LEF32_GEQ_U32_MIN: u64 = u32::MIN as u64;
pub const GEF32_LEQ_U32_MAX: u64 = 0x4f7f_ffff; // Max f32 bits for u32
pub const LEF64_GEQ_U32_MIN: u64 = u32::MIN as u64;
pub const GEF64_LEQ_U32_MAX: u64 = u32::MAX as u64;
pub const LEF32_GEQ_I64_MIN: u64 = i64::MIN as u64;
pub const GEF32_LEQ_I64_MAX: u64 = 0x5eff_ffff; // Max f32 bits for i64
pub const LEF64_GEQ_I64_MIN: u64 = i64::MIN as u64;
pub const GEF64_LEQ_I64_MAX: u64 = i64::MAX as u64;
pub const LEF32_GEQ_U64_MIN: u64 = u64::MIN;
pub const GEF32_LEQ_U64_MAX: u64 = 0x5f7f_ffff; // Max f32 bits for u64
pub const LEF64_GEQ_U64_MIN: u64 = u64::MIN;
pub const GEF64_LEQ_U64_MAX: u64 = u64::MAX;
