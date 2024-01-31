use ethers::types::{Address, U256};
use eyre::Result;
use clap::{App, Arg, SubCommand};
use serde_json::Value;

mod abi_decoder;
mod app_context;
mod config;
mod args;

use abi_decoder::read_abi;
use app_context::AppContext;
use clap::{Parser};
use crate::args::Commands;
use crate::args::Cli;
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let context = AppContext::new("config.toml", "0xdcB6669062b47f6cC3DB7A70EAa8FcA3d1D16DfC").await?;

    match cli.command {
        Commands::Mint { to, amount } => {
            // Call your mint function here
            println!("Minting {} tokens to {}", amount, to);
        },
        Commands::BalanceOf { address } => {
            let balance = context.contract.balance_of(address).call().await?;
            println!("Balance of {address} is {balance}");
        },
        Commands::Decimals {  } => {
            let decimals = context.contract.decimals().call().await?;
            println!("Decimals: {}", decimals);
        },
        Commands::ABI {  } => {
            read_abi("./erc20_abi.json")?;
        }
    }
    Ok(())
}

// read_abi("./erc20_abi.json")?;

//     let context = AppContext::new("config.toml", "0xdcB6669062b47f6cC3DB7A70EAa8FcA3d1D16DfC").await?;

//     let to_address = "0xF1B792820b52e6503208CAb98ec0B7b89ac64D6A".parse::<Address>()?;
//     let whole_amount: u64 = 10;
//     let decimals = context.contract.decimals().call().await?;
//     let decimal_amount = U256::from(whole_amount) * U256::exp10(decimals as usize);
//     println!("Decimal Amount: {}", decimals);

//     let balance = context.contract.balance_of(to_address).call().await?;
//     println!("Balance: {}", balance);

//     let tx = context.contract.mint(to_address, decimal_amount);
//     let pending_tx = tx.send().await?;
//     let _mined_tx = pending_tx.await?;

//     println!("Transaction Receipt: {}", serde_json::to_string(&_mined_tx)?);

//     let json_str = serde_json::to_string(&_mined_tx)?;
//     let json: Value = serde_json::from_str(&json_str)?;

//     if let Some(transaction_hash) = json["transactionHash"].as_str() {
//         println!("\n URL: https://sepolia.etherscan.io/tx/{}", transaction_hash);
//     } else {
//         println!("Transaction Hash not found");
//     }
//     let balance = context.contract.balance_of(to_address).call().await?;
//     println!("Balance: {}", balance);

//    Ok(())