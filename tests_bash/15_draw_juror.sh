set -e
cd ..
export TOKEN=`cat neardev/dev-account`
# 3 hours 3*60*60
near call $TOKEN draw_jurors '{"review_id": "1", "length":3}' --accountId amiyatulu.testnet