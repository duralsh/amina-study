use clap::{Parser, Subcommand};
use ethers::types::{Address, U256};

#[derive(Parser)]
#[clap(name = "AMINA CLI", about = "A CLI tool for interacting with an ERC20 contract", version = "1.0")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Mint tokens to an address
    Mint {
        #[clap(value_parser)]
        to: Address,
        #[clap(value_parser)]
        amount: U256,
    },
    /// Get the balance of an address
    BalanceOf {
        #[clap(value_parser)]
        address: Address,
    },
    /// Get the decimals of the token
    Decimals {
    },
    
    ABI {
       
    }
}

