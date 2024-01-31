
use crate::config::Config; 
use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address},
    contract::abigen
};

use eyre::Result;
use std::convert::TryFrom;
use std::sync::Arc;

abigen!(ERC20Contract, "./erc20_abi.json");

pub struct AppContext {
    pub contract: ERC20Contract<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

impl AppContext {
    pub async fn new(config_path: &str, contract_address: &str) -> Result<Self> {
        let config = Config::new(config_path)?;
        let provider = Provider::<Http>::try_from(config.provider_url())?;
        let chain_id = provider.get_chainid().await?;
        let wallet: LocalWallet = config.private_key().parse::<LocalWallet>()?.with_chain_id(chain_id.as_u64());
        let signer = Arc::new(SignerMiddleware::new(provider, wallet));

        let contract_address = contract_address.parse::<Address>()?;
        let contract = ERC20Contract::new(contract_address, signer);

        Ok(Self {
            contract,
        })
    }
}
