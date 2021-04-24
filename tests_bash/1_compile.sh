set -e
cd ..
cd contract
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/avrit.wasm ./res/