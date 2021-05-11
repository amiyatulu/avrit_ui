set -e
cd ..
export TOKEN=`cat neardev/dev-account`

# apply_jurors(&mut self, review_id: U128, stake: U128)

near call $TOKEN  unstaking_non_selected_juror '{"review_id": "1", "user_id": "2"}' --accountId amiyatulu.testnet 