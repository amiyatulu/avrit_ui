set -e
cd ..
export TOKEN=`cat neardev/dev-account`
near call $TOKEN new '{"owner_id":"amiyatulu.testnet", "total_supply":"500000000000000000000000000"}' --accountId amiyatulu.testnet