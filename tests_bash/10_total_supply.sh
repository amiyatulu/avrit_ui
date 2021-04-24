set -e
cd ..
export TOKEN=`cat neardev/dev-account`
near call $TOKEN ft_total_supply '' --accountId avrit.testnet