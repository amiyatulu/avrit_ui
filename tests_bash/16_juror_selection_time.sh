set -e
cd ..
export TOKEN=`cat neardev/dev-account`
near call $TOKEN get_juror_selection_time_js '{"review_id": "1"}' --accountId amiyatulu.testnet