#!/bin/sh
cd curl_adapter
marine build --release
cd ../
marine build --release

mkdir -p artifacts
rm artifacts/*
cp curl_adapter/target/wasm32-wasi/release/curl_adapter.wasm artifacts/                                   
cp target/wasm32-wasi/release/bazaar_ex.wasm artifacts/

wget https://github.com/fluencelabs/sqlite/releases/download/v0.14.0_w/sqlite3.wasm
mv sqlite3.wasm artifacts/

mrepl Config.toml