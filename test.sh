#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo +nightly test
cp target/wasm32-unknown-unknown/release/*.wasm res/
