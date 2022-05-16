#!/bin/sh

IDENTITY=$1
ADMIN=$2
TEST_MODE=true

echo Building
cargo build --target wasm32-unknown-unknown --release --package transaction_notifier_impl

echo Optimising wasm
wasm-opt target/wasm32-unknown-unknown/release/transaction_notifier_impl.wasm --strip-debug -Oz -o target/wasm32-unknown-unknown/release/transaction_notifier_impl-opt.wasm

dfx --identity $IDENTITY canister create transaction_notifier
dfx --identity $IDENTITY canister install transaction_notifier --argument '(record { admins = vec { principal "'${ADMIN}'" }; wasm_version = record { major = 0 : nat32; minor = 0 : nat32; patch = 0 : nat32 }; test_mode = true })'

echo $(dfx canister id transaction_notifier)