// Re-exports
pub use alloy_evm::{Database, Evm, EvmEnv, EvmFactory};
use dora::{
    Env, VM, VMContext, compile_handler,
    primitives::{
        Address, BlockEnv, Bytes, CfgEnv, HaltReason, Journal, ResultAndState, SpecId, TxEnv,
        TxKind, U256,
    },
    runtime::constants::env::DORA_TRACING,
};
use revm::{Context, Inspector, context_interface::result::EVMError, inspector::NoOpInspector};

/// The Ethereum EVM context type.
pub type EthEvmContext<DB> = Context<BlockEnv, TxEnv, CfgEnv, DB>;

/// EVM/WASM instance containing internal the VM context and the compiler handler.
pub struct DoraVM<DB: Database> {
    vm: VM<DB>,
}

impl<DB> Evm for DoraVM<DB>
where
    DB: Database,
{
    type DB = DB;
    type Tx = TxEnv;
    type Error = EVMError<DB::Error>;
    type HaltReason = HaltReason;
    type Spec = SpecId;

    fn block(&self) -> &BlockEnv {
        &self.vm.context.env.block
    }

    fn transact_raw(&mut self, tx: Self::Tx) -> Result<ResultAndState, Self::Error> {
        self.vm.env.tx = tx;
        self.vm.transact().map_err(dora_vm_error_to_evm_error::<DB>)
    }

    fn transact_system_call(
        &mut self,
        caller: Address,
        contract: Address,
        data: Bytes,
    ) -> Result<ResultAndState, Self::Error> {
        let tx = TxEnv {
            caller,
            kind: TxKind::Call(contract),
            // Explicitly set nonce to 0 so revm does not do any nonce checks
            nonce: 0,
            gas_limit: 30_000_000,
            value: U256::ZERO,
            data,
            // Setting the gas price to zero enforces that no value is transferred as part of the
            // call, and that the call will not count against the block's gas limit
            gas_price: 0,
            // The chain ID check is not relevant here and is disabled if set to None
            chain_id: None,
            // Setting the gas priority fee to None ensures the effective gas price is derived from
            // the `gas_price` field, which we need to be zero
            gas_priority_fee: None,
            access_list: Default::default(),
            // blob fields can be None for this tx
            blob_hashes: Vec::new(),
            max_fee_per_blob_gas: 0,
            tx_type: 0,
            authorization_list: Default::default(),
        };

        let mut gas_limit = tx.gas_limit;
        let mut basefee = 0;
        let mut disable_nonce_check = true;

        // ensure the block gas limit is >= the tx
        core::mem::swap(&mut self.vm.env.block.gas_limit, &mut gas_limit);
        // disable the base fee check for this call by setting the base fee to zero
        core::mem::swap(&mut self.vm.env.block.basefee, &mut basefee);
        // disable the nonce check
        core::mem::swap(
            &mut self.vm.env.cfg.disable_nonce_check,
            &mut disable_nonce_check,
        );

        let mut res = self.transact(tx);

        // swap back to the previous gas limit
        core::mem::swap(&mut self.vm.env.block.gas_limit, &mut gas_limit);
        // swap back to the previous base fee
        core::mem::swap(&mut self.vm.env.block.basefee, &mut basefee);
        // swap back to the previous nonce check flag
        core::mem::swap(
            &mut self.vm.env.cfg.disable_nonce_check,
            &mut disable_nonce_check,
        );

        // NOTE: We assume that only the contract storage is modified. Revm currently marks the
        // caller and block beneficiary accounts as "touched" when we do the above transact calls,
        // and includes them in the result.
        //
        // We're doing this state cleanup to make sure that changeset only includes the changed
        // contract storage.
        if let Ok(res) = &mut res {
            res.state.retain(|addr, _| *addr == contract);
        }

        res
    }

    fn db_mut(&mut self) -> &mut Self::DB {
        &mut self.vm.journal.database
    }

    fn finish(self) -> (Self::DB, EvmEnv<Self::Spec>) {
        let VMContext {
            env: Env { block, cfg, .. },
            journal: Journal { database, .. },
            ..
        } = self.vm.context;
        (
            database,
            EvmEnv {
                block_env: block,
                cfg_env: cfg,
            },
        )
    }

    fn set_inspector_enabled(&mut self, enabled: bool) {
        if enabled {
            unsafe { std::env::set_var(DORA_TRACING, "1") };
        } else {
            unsafe { std::env::remove_var(DORA_TRACING) };
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DoraVMFactory;

impl EvmFactory for DoraVMFactory {
    type Evm<DB: Database, I: Inspector<EthEvmContext<DB>>> = DoraVM<DB>;
    type Context<DB: Database> = Context<BlockEnv, TxEnv, CfgEnv, DB>;
    type Tx = TxEnv;
    type Error<DBError: core::error::Error + Send + Sync + 'static> = EVMError<DBError>;
    type HaltReason = HaltReason;
    type Spec = SpecId;

    fn create_evm<DB: Database>(&self, db: DB, input: EvmEnv) -> Self::Evm<DB, NoOpInspector> {
        DoraVM {
            vm: VM::new(VMContext::new(
                db,
                Env {
                    cfg: input.cfg_env,
                    block: input.block_env,
                    ..Default::default()
                },
                compile_handler(),
            )),
        }
    }

    fn create_evm_with_inspector<DB: Database, I: Inspector<Self::Context<DB>>>(
        &self,
        db: DB,
        input: EvmEnv,
        _inspector: I,
    ) -> Self::Evm<DB, I> {
        unsafe { std::env::set_var(DORA_TRACING, "1") };
        self.create_evm(db, input)
    }
}

#[inline]
pub fn dora_vm_error_to_evm_error<DB: Database>(err: dora::VMError) -> EVMError<DB::Error> {
    match err {
        dora::VMError::Transaction(invalid_transaction) => {
            EVMError::Transaction(invalid_transaction)
        }
        dora::VMError::Header(invalid_header) => EVMError::Header(invalid_header),
        dora::VMError::Database(database_error) => EVMError::Custom(database_error.to_string()),
        dora::VMError::Compile(err)
        | dora::VMError::Precompile(err)
        | dora::VMError::Handler(err) => EVMError::Custom(err),
    }
}
