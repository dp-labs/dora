use anyhow::{Context, Result};
use bytes::Bytes;
use clap::{Args, Parser, Subcommand};
use dora_primitives::db::MemoryDb;
use dora_primitives::{Address, Bytecode, U256};
use dora_runtime::env::{Env, TransactTo};
use std::str::FromStr;
use tracing::{error, info};

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
    /// Path to the contract file
    contract: String,

    /// Call data in hex format (0x prefixed)
    #[arg(long)]
    calldata: Option<String>,

    /// Gas limit for execution
    #[arg(long, default_value = "1000000")]
    gas_limit: u64,

    /// Value to send with call in wei
    #[arg(long, default_value = "0")]
    value: String,

    /// Address of the sender
    #[arg(long, default_value = "0x0000000000000000000000000000000000000000")]
    sender: String,

    /// Chain ID
    #[arg(long, default_value = "1")]
    chain_id: u64,

    /// Block number
    #[arg(long, default_value = "1")]
    block_number: u64,

    /// Block timestamp
    #[arg(long, default_value = "1")]
    timestamp: u64,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    match &cli.command {
        Commands::Run(run_args) => {
            // Parse contract bytecode
            let bytecode = std::fs::read(&run_args.contract)
                .with_context(|| format!("Failed to read contract file: {}", run_args.contract))?;

            // Parse calldata
            let calldata = run_args
                .calldata
                .as_ref()
                .map(|data| {
                    hex::decode(data.strip_prefix("0x").unwrap_or(data))
                        .expect("Invalid calldata hex")
                })
                .unwrap_or_default();

            // Parse sender
            let sender = Address::from_str(run_args.sender.as_str())
                .with_context(|| format!("Invalid sender address: {}", run_args.sender))?;

            let address = Address::from_low_u64_be(40);

            // Set Env
            let mut env = Env::default();
            env.tx.gas_limit = run_args.gas_limit;
            env.tx.value = U256::from_dec_str(&run_args.value).context("Failed to parse value")?;
            env.tx.caller = sender;
            env.tx.transact_to = TransactTo::Call(address);
            env.tx.data = Bytes::from(calldata);
            env.block.number = U256::from(run_args.block_number);
            env.block.timestamp = U256::from(run_args.timestamp);
            // Set DB
            let mut db = MemoryDb::new().with_contract(address, Bytecode::from(bytecode));
            // Run the contract
            match dora::run_evm(env, &mut db) {
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
    Ok(())
}
