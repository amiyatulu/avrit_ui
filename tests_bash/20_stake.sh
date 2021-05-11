set -e
cd ..
export TOKEN=`cat neardev/dev-account`

# apply_jurors(&mut self, review_id: U128, stake: U128)

# near call $TOKEN  apply_jurors '{"review_id": "1", "stake": "10"}' --accountId avrit2.testnet 

near call $TOKEN  apply_jurors '{"review_id": "1", "stake": "10"}' --accountId avrit3.testnet 

near call $TOKEN  apply_jurors '{"review_id": "1", "stake": "10"}' --accountId avrit4.testnet 

near call $TOKEN  apply_jurors '{"review_id": "1", "stake": "10"}' --accountId avrit5.testnet 

near call $TOKEN  apply_jurors '{"review_id": "1", "stake": "20"}' --accountId avrit6.testnet 

near call $TOKEN  apply_jurors '{"review_id": "1", "stake": "20"}' --accountId avrit7.testnet 

near call $TOKEN  apply_jurors '{"review_id": "1", "stake": "20"}' --accountId avrit8.testnet 

near call $TOKEN  apply_jurors '{"review_id": "1", "stake": "30"}' --accountId avrit9.testnet 

near call $TOKEN  apply_jurors '{"review_id": "1", "stake": "25"}' --accountId avrit10.testnet 

near call $TOKEN  apply_jurors '{"review_id": "1", "stake": "25"}' --accountId avrit11.testnet 

near call $TOKEN  apply_jurors '{"review_id": "1", "stake": "23"}' --accountId avrit12.testnet 

