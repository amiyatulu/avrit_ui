set -e
cd ..
export TOKEN=`cat neardev/dev-account`
# Default 15 days = 1296000
# 3 hours 3*60*60 = 10800
# 15 mins = 900
near call $TOKEN set_reveal_phase_time '{"time_in_secs": 10800}' --accountId amiyatulu.testnet