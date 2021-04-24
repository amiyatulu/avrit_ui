set -e
sh ./1_compile.sh
cd ..
near dev-deploy contract/res/avrit.wasm
