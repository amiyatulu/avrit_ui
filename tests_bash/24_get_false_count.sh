set -e
cd ..
export TOKEN=`cat neardev/dev-account`

# apply_jurors(&mut self, review_id: U128, stake: U128)

near call $TOKEN  get_false_count_js '{"review_id": "1"}' --accountId amiyatulu.testnet 