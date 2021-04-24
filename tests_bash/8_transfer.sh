set -e
cd ..
export TOKEN=`cat neardev/dev-account`
near call $TOKEN ft_transfer '{"receiver_id":"amiyatulu.testnet", "amount":"1000"}' --accountId avrit.testnet --amount 0.000000000000000000000001