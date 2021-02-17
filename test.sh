#!/bin/bash
set -e
./build.sh
cargo +nightly test -- --nocapture
