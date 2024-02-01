mod abi_decoder;
mod app_context;
mod args;
mod config;
mod tx_recorder;

use eyre::Result;
use crate::args::Cli;
use crate::args::Commands;
use app_context::AppContext;
use clap::Parser;
use config::Config;
use tx_recorder::TxRecorder;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let context =
        AppContext::new("config.toml", "0xdcB6669062b47f6cC3DB7A70EAa8FcA3d1D16DfC").await?;
    let recorder = TxRecorder::new("txs.txt");
    match cli.command {
        Commands::Mint { to, amount } => {
            println!("Minting {} tokens to {}", amount, to);
            let tx = context.contract.mint(to, amount.into());
            let pending_tx = tx.send().await?;
            let _mined_tx = pending_tx.await?;
            recorder.append_to_file(&serde_json::to_string(&_mined_tx)?)?;
            println!(
                "Transaction Receipt: {}",
                serde_json::to_string(&_mined_tx)?
            );
        }
        Commands::Transfer { to, amount } => {
            let config = Config::new("config.toml")?;

            println!(
                "Transferring {} tokens to {} from {} ",
                amount,
                to,
                config.public_key()
            );
            let tx = context.contract.transfer(to, amount.into());
            let pending_tx = tx.send().await?;
            let _mined_tx = pending_tx.await?;
            recorder.append_to_file(&serde_json::to_string(&_mined_tx)?)?;

            println!(
                "Transaction Receipt: {}",
                serde_json::to_string(&_mined_tx)?
            );
        }
        Commands::BalanceOf { address } => {
            let balance = context.contract.balance_of(address).call().await?;
            println!("Balance of {address} is {balance}");
        }
        Commands::Decimals {} => {
            let decimals = context.contract.decimals().call().await?;
            println!("Decimals: {}", decimals);
        }
        Commands::Abi {} => {
            abi_decoder::read_abi("./erc20_abi.json")?;
        }
        Commands::TaxPercent {} => {
            let tax = context.contract.tax_percentage().call().await?;
            println!("Tax percentage is {tax}");
        }
        Commands::TotalSupply {} => {
            let total_supply = context.contract.total_supply().call().await?;
            println!("Total supply is {total_supply}");
        }
        Commands::MaxSupply {} => {
            let max_supply = context.contract.max_supply().call().await?;
            println!("Max supply is {max_supply}");
        }
        Commands::Owner {} => {
            let owner = context.contract.owner().call().await?;
            println!("Owner is {owner}");
        }
    }
    Ok(())
}
