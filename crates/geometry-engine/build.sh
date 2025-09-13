#!/usr/bin/env bash
set -e

echo "Building WASM module..."
wasm-pack build --target web --out-dir ../../packages/wasm

echo "WASM module built successfully!"