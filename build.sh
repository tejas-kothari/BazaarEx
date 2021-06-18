#!/bin/sh

fce build --release

mkdir -p artifacts
rm artifacts/*                                                                                          
cp target/wasm32-wasi/release/bazaar_ex.wasm artifacts/

wget https://github.com/fluencelabs/sqlite/releases/download/v0.10.0_w/sqlite3.wasm
mv sqlite3.wasm artifacts/

fce-repl Config.toml