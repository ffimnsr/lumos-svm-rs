# Lumos SVM 🚀

A Solana Test Validator Manager (SVM) that simplifies local development and testing of Solana programs.

## Features

- 🚀 Quick setup of local Solana test validator
- 📦 Automatic account and program state cloning
- 🔍 Token analysis and management
- 🛠 Configurable via TOML
- 🔄 Port management and validator configuration

## Installation

```bash
cargo install lumos-svm
```

## Quick Start

1. Create a configuration file (`lumos.toml`):

```toml
[general]
rpc_endpoint = "https://eclipse.lgns.net/"
ledger_dir = ".lumos-ledger"  # Optional, defaults to .lumos-ledger

[account.usdc]
address = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"

[program.orca_whirlpool]
address = "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc"
```

2. Start the validator:

```bash
lumos-svm start
```

## Advanced Features

### Modify Token Mint Authority

You can modify the mint authority for cloned token accounts:

```toml
[general]
rpc_endpoint = "https://eclipse.lgns.net/"
mint_authority = "<YOUR-PUBLIC-KEY>"

[account.usdc]
address = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
mint = true  # Set this token as mintable
update = true  # Update if previously cloned
```

This configuration:

- Sets your public key as the mint authority
- Makes the USDC token mintable locally
- Updates existing cloned accounts when specified

## Commands

- `lumos-svm run`: Start the test validator
- `lumos-svm analyze <ADDRESS>`: Analyze token or program details
- `lumos-svm clone`: Clone accounts and programs from config

## Development

```bash
# Clone the repository
git clone https://github.com/ffimnsr/lumos-svm-rs
cd lumos-svm-rs

# Build
cargo build

# Run tests
cargo test
```

## Requirements

- Rust 1.70+
- Solana CLI tools with `solana-test-validator`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the Apache and MIT License - see the [COPYING](COPYING) file for details.
