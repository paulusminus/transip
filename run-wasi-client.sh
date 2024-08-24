#!/bin/sh

cargo component build --release --target wasm32-wasip1 -p http-client --example paulmin
wasmtime -S cli=y,http=y target/wasm32-wasip1/release/examples/paulmin.wasm
