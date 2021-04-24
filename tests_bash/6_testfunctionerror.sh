set -e
cd ..
export TOKEN=`cat neardev/dev-account`
near call $TOKEN change_burn_percentage '{"value":3}' --accountId amiyatulu.testnet