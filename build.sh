#! /usr/bin/env bash

mkdir -p build/release build/debug

cargo +nightly build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/image_to_ascii.wasm --out-dir ./build/debug/

cargo +nightly build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/image_to_ascii.wasm --out-dir ./build/release/
