set -e
cd ..
export TOKEN=`cat neardev/dev-account`
echo Enter the account to check balance
read account
near call $TOKEN ft_balance_of '{"account_id":"'$account'"}' --accountId avrit.testnet