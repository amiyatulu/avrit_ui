set -e
cd ..
export TOKEN=`cat neardev/dev-account`
# 3 hours 3*60*60
near call $TOKEN set_jury_count '{"jury_count": 4}' --accountId amiyatulu.testnet