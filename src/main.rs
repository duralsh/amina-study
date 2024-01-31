use alloy_json_abi::Function;
use alloy_primitives::{Address, U256};
use ethers_core::types::{
    transaction::eip2718::TypedTransaction, Eip1559TransactionRequest, NameOrAddress,
    TransactionRequest,
};
use ethers::providers::{Http, Provider,Middleware};

use eyre::{eyre, Result};
use foundry_common::types::{ToAlloy, ToEthers};

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;

  
    let from = "your-from-address"; 
    let to = Some("your-to-address"); 
    let chain_id = 1; 

    let tx_builder = TxBuilder::new(&provider, from, to, chain_id).await?;



    Ok(())
}



pub struct TxBuilder<'a, M: Middleware> {
    to: Option<Address>,
    chain: u32,
    tx: TypedTransaction,
    func: Option<Function>,
    etherscan_api_key: Option<String>,
    provider: &'a M,
}

impl<'a, M: Middleware> TxBuilder<'a, M> {

    async fn new<F: Into<NameOrAddress>, T: Into<NameOrAddress>>(
        provider: &'a M,
        from: F,
        to: Option<T>,
        chain: u32,
    ) -> Result<TxBuilder<'a, M>> {

        let from_addr = resolve_ens(provider, from).await?;


        let mut tx: TypedTransaction = 
            Eip1559TransactionRequest::new().from(from_addr.to_ethers()).chain_id(chain).into();
            
        let to_addr = if let Some(to) = to {
            let addr = resolve_ens(provider, to).await?;
            tx.set_to(addr.to_ethers());
            Some(addr)
        } else {
            None
        };
        
        Ok(Self { to: to_addr, chain, tx, func: None, etherscan_api_key: None, provider })
    }
}

async fn resolve_ens<M: Middleware, T: Into<NameOrAddress>>(
    provider: &M,
    addr: T,
) -> Result<Address> {
    let from_addr = match addr.into() {
        NameOrAddress::Name(ref ens_name) => provider.resolve_name(ens_name).await,
        NameOrAddress::Address(addr) => Ok(addr),
    }
    .map_err(|x| eyre!("Failed to resolve ENS name: {x}"))?;
    Ok(from_addr.to_alloy())
}