wc -c app_web_build/app_bg.wasm
../binaryen/bin/wasm-opt -O4 -o app_web_build/app_bg.wasm app_web_build/app_bg.wasm
wc -c app_web_build/app_bg.wasm
#twiggy top -n 20 mise_en_place_app_web_build/mise_en_place_app_bg.wasm
