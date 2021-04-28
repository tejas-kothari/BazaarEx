#!/bin/sh

fce build --release

mkdir -p artifacts
rm artifacts/*                                                                                          
cp target/wasm32-wasi/release/xpact.wasm artifacts/