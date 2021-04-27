#!/bin/sh
# This script builds all sub-projects and puts our Wasm module(s) in a high-level dir

fce build --release                                            # 1

mkdir -p artifacts                                             # 2
rm artifacts/*                                                                                          
cp target/wasm32-wasi/release/xpact.wasm artifacts/            # 3