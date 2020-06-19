# Status Message

Records the status messages of the accounts that call this contract.

## Build
To build run:
```bash
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/contract_erc20.wasm  res
```


## Testing
To test run:
```bash
cargo test --package contract_erc20 -- --nocapture
```
