set -e
cd ..
export TOKEN=`cat neardev/dev-account`
near call $TOKEN change_burn_percentage_w_near '{"value":"10"}' --accountId amiyatulu.testnet