set -e
cd ..
export TOKEN=`cat neardev/dev-account`
echo Enter the account to register
read register
near call $TOKEN register_account '{"account_id":"'$register'"}' --accountId amiyatulu.testnet