// Debug functions
pub const NOP: &str = "dora_fn_nop";
pub const TRACING: &str = "dora_fn_tracing";
// Global variables
pub const CTX_IS_STATIC: &str = "dora_fn_is_static";
// System functions
pub const WRITE_RESULT: &str = "dora_fn_write_result";
pub const EXTEND_MEMORY: &str = "dora_fn_extend_memory";
pub const MEMORY_PTR: &str = "dora_fn_memory_ptr";
pub const MEMORY_SIZE: &str = "dora_fn_memory_size";
pub const CALLDATA: &str = "dora_fn_calldata";
pub const CALLDATA_SIZE: &str = "dora_fn_calldata_size";
pub const CALLDATA_COPY: &str = "dora_fn_calldata_copy";
pub const DATA_SECTION: &str = "dora_fn_data_section";
pub const DATA_SECTION_SIZE: &str = "dora_fn_data_section_size";
pub const DATA_SECTION_COPY: &str = "dora_fn_data_section_copy";
pub const EXP: &str = "dora_fn_exp";
pub const KECCAK256_HASHER: &str = "dora_fn_keccak256_hasher";
pub const SSTORE: &str = "dora_fn_sstore";
pub const SLOAD: &str = "dora_fn_sload";
pub const APPEND_LOG: &str = "dora_fn_append_log";
pub const APPEND_LOG_ONE_TOPIC: &str = "dora_fn_append_log_with_one_topic";
pub const APPEND_LOG_TWO_TOPICS: &str = "dora_fn_append_log_with_two_topics";
pub const APPEND_LOG_THREE_TOPICS: &str = "dora_fn_append_log_with_three_topics";
pub const APPEND_LOG_FOUR_TOPICS: &str = "dora_fn_append_log_with_four_topics";
pub const EXT_CODE_SIZE: &str = "dora_fn_extcodesize";
pub const CODE_COPY: &str = "dora_fn_code_copy";
pub const ADDRESS: &str = "dora_fn_address";
pub const STORE_IN_GASLIMIT_PTR: &str = "dora_fn_store_in_gaslimit_ptr";
pub const CALLVALUE: &str = "dora_fn_callvalue";
pub const STORE_IN_BLOBBASEFEE_PTR: &str = "dora_fn_store_in_blobbasefee_ptr";
pub const BLOB_HASH: &str = "dora_fn_blob_hash";
pub const STORE_IN_BALANCE: &str = "dora_fn_store_in_balance";
pub const COINBASE: &str = "dora_fn_coinbase";
pub const STORE_IN_TIMESTAMP_PTR: &str = "dora_fn_store_in_timestamp_ptr";
pub const STORE_IN_BASEFEE_PTR: &str = "dora_fn_store_in_basefee_ptr";
pub const CALLER: &str = "dora_fn_caller";
pub const ORIGIN: &str = "dora_fn_origin";
pub const CHAINID: &str = "dora_fn_chainid";
pub const STORE_IN_GASPRICE_PTR: &str = "dora_fn_store_in_gasprice_ptr";
pub const BLOCK_NUMBER: &str = "dora_fn_block_number";
pub const STORE_IN_SELFBALANCE_PTR: &str = "dora_fn_store_in_selfbalance_ptr";
pub const EXT_CODE_COPY: &str = "dora_fn_ext_code_copy";
pub const PREVRANDAO: &str = "dora_fn_prevrando";
pub const BLOCK_HASH: &str = "dora_fn_block_hash";
pub const EXT_CODE_HASH: &str = "dora_fn_ext_code_hash";
pub const EOFCREATE: &str = "dora_fn_eofcreate";
pub const RETURNCONTRACT: &str = "dora_fn_returncontract";
pub const CALL: &str = "dora_fn_call";
pub const CREATE: &str = "dora_fn_create";
pub const CREATE2: &str = "dora_fn_create2";
pub const EXTCALL: &str = "dora_fn_extcall";
pub const RETURNDATA: &str = "dora_fn_returndata";
pub const RETURNDATA_SIZE: &str = "dora_fn_returndata_size";
pub const RETURNDATA_COPY: &str = "dora_fn_returndata_copy";
pub const TLOAD: &str = "dora_fn_tload";
pub const TSTORE: &str = "dora_fn_tstore";
pub const SELFDESTRUCT: &str = "dora_fn_selfdestruct";

// WASM Related libcall functions
pub mod wasm {
    pub const TABLE_INIT: &str = "dora_fn_wasm_table_init";
    pub const TABLE_FILL: &str = "dora_fn_wasm_table_fill";
    pub const TABLE_SIZE: &str = "dora_fn_wasm_table_size";
    pub const TABLE_GET: &str = "dora_fn_wasm_table_get";
    pub const TABLE_SET: &str = "dora_fn_wasm_table_set";
    pub const TABLE_GROW: &str = "dora_fn_wasm_table_grow";
    pub const IMPROTED_TABLE_SIZE: &str = "dora_fn_wasm_imported_table_size";
    pub const IMPROTED_TABLE_GET: &str = "dora_fn_wasm_imported_table_get";
    pub const IMPROTED_TABLE_SET: &str = "dora_fn_wasm_imported_table_set";
    pub const IMPROTED_TABLE_GROW: &str = "dora_fn_wasm_imported_table_grow";
    pub const MEMORY_INIT: &str = "dora_fn_wasm_memory_init";
    pub const MEMORY_COPY: &str = "dora_fn_wasm_memory_copy";
    pub const MEMORY_FILL: &str = "dora_fn_wasm_memory_fill";
    pub const MEMORY_NOTIFY: &str = "dora_fn_wasm_memory_notify";
    pub const MEMORY_WAIT32: &str = "dora_fn_wasm_memory_wait32";
    pub const MEMORY_WAIT64: &str = "dora_fn_wasm_memory_wait64";
    pub const IMPORTED_MEMORY_COPY: &str = "dora_fn_wasm_imported_memory_copy";
    pub const IMPORTED_MEMORY_FILL: &str = "dora_fn_wasm_imported_memory_fill";
    pub const IMPORTED_MEMORY_NOTIFY: &str = "dora_fn_wasm_imported_memory_notify";
    pub const IMPORTED_MEMORY_WAIT32: &str = "dora_fn_wasm_imported_memory_wait32";
    pub const IMPORTED_MEMORY_WAIT64: &str = "dora_fn_wasm_imported_memory_wait64";
    pub const FUNC_REF: &str = "dora_fn_wasm_func_ref";
    pub const DATA_DROP: &str = "dora_fn_wasm_data_drop";
    pub const ELEM_DROP: &str = "dora_fn_wasm_elem_drop";
    pub const RAISE_TRAP: &str = "dora_fn_wasm_raise_trap";
}
