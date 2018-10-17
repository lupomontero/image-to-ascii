#! /usr/bin/env bash


mkdir -p build/release build/debug


if [[ "${1}" != "--release" ]]; then
  # Build CLI
  cargo +nightly build
  # Build WASM target
  cargo +nightly build --target wasm32-unknown-unknown
  # Generate JavaScript bindings
  wasm-bindgen target/wasm32-unknown-unknown/debug/image_to_ascii.wasm \
    --out-dir ./build/debug/
else
  # Build CLI
  cargo +nightly build --release
  # Build WASM target
  cargo +nightly build --target wasm32-unknown-unknown --release
  # Generate JavaScript bindings
  wasm-bindgen target/wasm32-unknown-unknown/release/image_to_ascii.wasm \
    --out-dir ./build/release/
fi
