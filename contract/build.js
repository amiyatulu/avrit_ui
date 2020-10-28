const shell = require('shelljs');

shell.fatal = true; // same as "set -e"

shell.cd('contract_new');
// Note: see flags in ./cargo/config
shell.exec('cargo build --target wasm32-unknown-unknown --release');
shell.cp('./target/wasm32-unknown-unknown/release/status_message.wasm', './res');