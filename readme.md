# AMINA CLI Documentation

AMINA CLI is a command-line interface tool designed for interacting with Amina Token [ERC20 contract](https://sepolia.etherscan.io/address/0xdcb6669062b47f6cc3db7a70eaa8fca3d1d16dfc). It is built using Rust and leverages several libraries to manage blockchain interactions, parsing, and command-line operations.

Amina Token : https://sepolia.etherscan.io/address/0xdcb6669062b47f6cc3db7a70eaa8fca3d1d16dfc

## Getting Started

To use AMINA CLI, ensure you have Rust installed on your system. Clone the repository and build the project using Cargo, Rust's package manager and build system.

```bash
git clone https://github.com/partychad/amina-study.git
cd amina-study
cargo build --release
```

## Commands

AMINA CLI supports various commands for interacting with the ERC20 contract:

- **Mint**: Mint tokens to a specified address.

  ```bash
  amina-study mint --to <ADDRESS> --amount <AMOUNT>
  ```

- **Transfer**: Transfer tokens to a specified address.

  ```
  amina-study transfer --to <ADDRESS> --amount <AMOUNT>
  ```

- **BalanceOf**: Get the balance of a specified address.

  ```bash
  amina-study balanceof --address <ADDRESS>
  ```

- **Decimals**: Get the decimals of the token.

  ```bash
  amina-study decimals
  ```

- **Owner**: Get the owner of the token.

  ```bash
  amina-study owner
  ```

- **TotalSupply**: Get the total supply of the token.

  ```bash
  amina-study totalsupply
  ```

- **MaxSupply**: Get the max supply of the token.

  ```bash
  amina-study maxsupply
  ```

- **TaxPercent**: Get the transfer tax percent of the token.

  ```bash
  amina-study taxpercent
  ```

- **Abi**: Get the ABI of the token.

  ```bash
  amina-study abi
  ```

## Dependencies

AMINA CLI uses the following Rust crates:

- `tokio` for asynchronous runtime.
- `eyre` for error handling.
- `ethers` for Ethereum blockchain interactions.
- `serde` and `serde_json` for serialization and deserialization.
- `toml` for configuration file parsing.
- `clap` for command-line argument parsing.

## Configuration

The CLI tool requires a `config.toml` file and an ERC20 ABI JSON file (`erc20_abi.json`) to interact with the blockchain. Ensure these files are correctly set up and located as expected by the application.

For more details on the implementation, refer to the source code:

- [main.rs](https://github.com/partychad/amina-study/blob/master/src/main.rs) - Entry point and command handling.
- [args.rs](https://github.com/partychad/amina-study/blob/master/src/args.rs) - Command line arguments definition.
- [Cargo.toml](https://github.com/partychad/amina-study/blob/master/Cargo.toml) - Project dependencies and metadata.