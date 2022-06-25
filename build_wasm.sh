#!/bin/bash
rm -rf ./out/
cargo build --release --no-default-features --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/chimera-rancher.wasm
cp -R ./wasm-page-template/* ./out/
cp -R ./assets/ ./out/
python -m http.server 8080 --directory ./out/