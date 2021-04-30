set -e
cd ..
export TOKEN=`cat neardev/dev-account`
near state $TOKEN
