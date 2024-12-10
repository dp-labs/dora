use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use dora_primitives::spec::SpecId;
use dora_primitives::{Address, Bytecode, Bytes, Bytes32, U256};
use dora_runtime::db::MemoryDB;
use dora_runtime::env::Env;
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
    /// Path to the contract file,
    /// or hex text of the contract (Classified by 0x prefix)
    contract: String,

    /// Whether the contract file is hex text format (Default is false)
    #[clap(long)]
    hex_file: bool,

    /// Skip validating EOF code (Default is true)
    #[clap(long, default_value = "true")]
    no_validate: bool,

    /// Call data in hex format (0x prefixed)
    #[arg(long)]
    calldata: Option<String>,

    /// Gas limit for execution
    #[arg(long, default_value = "2000000")]
    gas_limit: u64,

    /// Value to send with call in wei
    #[arg(long, default_value = "0")]
    value: String,

    /// Address of the sender
    #[arg(long, default_value = "0x00000000000000000000000000000000000000AA")]
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

    /// VM Spec id
    #[arg(long, default_value = "CANCUN")]
    spec_id: SpecId,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    match &cli.command {
        Commands::Run(run_args) => {
            let bytecode: Vec<u8>;
            // Check hex flag
            if !run_args.hex_file {
                // Check contract string
                match run_args.contract.strip_prefix("0x") {
                    None => {
                        // Parse contract bytecode
                        bytecode = std::fs::read(&run_args.contract).with_context(|| {
                            format!(
                                "Failed to read contract bytecode file: {}",
                                run_args.contract
                            )
                        })?;
                    }
                    Some(contract_hex) => {
                        // Convert contract hex text into bytecode
                        bytecode = hex::decode(contract_hex).with_context(|| {
                            format!("Invalid contract hex text: {contract_hex}")
                        })?;
                    }
                }
            } else {
                // Parse contract hex file
                let contract_hex_text =
                    std::fs::read_to_string(&run_args.contract).with_context(|| {
                        format!("Failed to read contract hex file: {}", run_args.contract)
                    })?;
                bytecode = hex::decode(
                    contract_hex_text
                        .strip_prefix("0x")
                        .unwrap_or(&contract_hex_text),
                )
                .with_context(|| format!("Invalid contract hex file: {}", run_args.contract))?;
            }

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

            let address = Bytes32::from(40_u8).to_address();

            // Set Env
            let mut env = Env::default();
            env.tx.gas_limit = run_args.gas_limit;
            env.tx.value = U256::from_str(&run_args.value).context("Failed to parse value")?;
            env.tx.caller = sender;
            env.tx.transact_to = address;
            env.tx.data = Bytes::from(calldata);
            env.block.number = U256::from(run_args.block_number);
            env.block.timestamp = U256::from(run_args.timestamp);
            // Set DB
            let db = MemoryDB::new().with_contract(address, Bytecode::from(bytecode));
            // Run the contract
            match dora::run_evm(env, db, run_args.spec_id) {
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
