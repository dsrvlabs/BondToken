#!/bin/bash
nearup localnet --binary-path /nearcore/target/release
near create-account yoonsung.node0 --masterAccount node0 --initialBalance 100000 --keyPath ~/.near/localnet/node0/validator_key.json