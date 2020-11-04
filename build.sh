#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo +nightly build --lib --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm res/
