use ethers::{
    types::{Address, U256},
};

use eyre::Result;

use serde_json::Value;



mod config;
mod abi_decoder;
mod app_context;

use app_context::AppContext;
use abi_decoder::read_abi;


#[tokio::main]
async fn main() -> Result<()> {

    let context = AppContext::new("config.toml", "0xdcB6669062b47f6cC3DB7A70EAa8FcA3d1D16DfC").await?;
    read_abi("./erc20_abi.json")?;


    let to_address = "0xF1B792820b52e6503208CAb98ec0B7b89ac64D6A".parse::<Address>()?;


    let whole_amount: u64 = 10;
    let decimals = context.contract.decimals().call().await?;
    let decimal_amount = U256::from(whole_amount) * U256::exp10(decimals as usize);
    println!("Decimal Amount: {}", decimals);

    let balance = context.contract.balance_of(to_address).call().await?;
    println!("Balance: {}", balance);

    let tx = context.contract.mint(to_address, decimal_amount);
    let pending_tx = tx.send().await?;
    let _mined_tx = pending_tx.await?;

    println!("Transaction Receipt: {}", serde_json::to_string(&_mined_tx)?);

    let json_str = serde_json::to_string(&_mined_tx)?;
    let json: Value = serde_json::from_str(&json_str)?;

    if let Some(transaction_hash) = json["transactionHash"].as_str() {
        println!("\n URL: https://sepolia.etherscan.io/tx/{}", transaction_hash);
    } else {
        println!("Transaction Hash not found");
    }
    let balance = context.contract.balance_of(to_address).call().await?;
    println!("Balance: {}", balance);

   Ok(())
}


