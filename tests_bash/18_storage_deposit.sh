set -e
cd ..
export TOKEN=`cat neardev/dev-account`

near call $TOKEN  storage_deposit '{"account_id": "avrit7.testnet"}' --accountId amiyatulu.testnet --amount 0.00235
