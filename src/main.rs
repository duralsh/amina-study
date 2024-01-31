use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, U256},
    contract::abigen
};
use eyre::Result;
use std::convert::TryFrom;
use std::sync::Arc;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<()> {

    let provider = Provider::<Http>::try_from("")?;

    let chain_id = provider.get_chainid().await?;

    let contract_address = "0x779877A7B0D9E8603169DdbD7836e478b4624789".parse::<Address>()?;

    abigen!(ERC20Contract, "./erc20_abi.json",);

    let to_address = "0xF1B792820b52e6503208CAb98ec0B7b89ac64D6A".parse::<Address>()?;

   
    let wallet: LocalWallet = ""
        .parse::<LocalWallet>()?
        .with_chain_id(chain_id.as_u64());

    let signer = Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(chain_id.as_u64())));
    let contract = ERC20Contract::new(contract_address, signer);

    let whole_amount: u64 = 1;
    let decimals = contract.decimals().call().await?;
    let decimal_amount = U256::from(whole_amount) * U256::exp10(decimals as usize);
    print!("Decimal Amount: {}", decimals);
    let tx = contract.transfer(to_address, decimal_amount);
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

   Ok(())
}