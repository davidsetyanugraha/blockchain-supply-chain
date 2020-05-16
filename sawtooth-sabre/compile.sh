#!/bin/bash 

# Compile Smart Contract
rustup update
rustup default nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
cd example/intkey_add/processor && cargo build --target wasm32-unknown-unknown --release

