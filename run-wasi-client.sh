#!/bin/sh

cargo component build --release --target wasm32-wasip1 -p http-client --example 01
wasmtime -S cli=y,http=y target/wasm32-wasip1/release/examples/01.wasm
