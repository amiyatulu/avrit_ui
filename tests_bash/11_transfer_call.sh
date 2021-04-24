set -e
cd ..
export TOKEN=`cat neardev/dev-account`
near call $TOKEN ft_transfer_call '{"receiver_id":"amiyatulu.testnet", "amount":"1000", "memo":"new year", "msg":"transfer 1000"}' --accountId avrit.testnet --amount 0.000000000000000000000001