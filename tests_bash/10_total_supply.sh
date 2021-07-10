set -e
cd ..
export TOKEN=`cat neardev/dev-account`
near call avrit0.testnet ft_total_supply '' --accountId avrit.testnet