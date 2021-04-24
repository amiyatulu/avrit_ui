set -e
sh ./1_compile.sh
cd ..
near dev-deploy -f contract/res/avrit.wasm
