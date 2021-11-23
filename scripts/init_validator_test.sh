#!/usr/bin/env bash

cargo build-bpf

BASEDIR=$(dirname "$0")

solana-test-validator --reset \
    --bpf-program 5Hu2bnTxd1mPXNHqMzFfB5SUFEvYW7GG3nPSQ1VWvTK "$BASEDIR/../target/deploy/meep.so" \
    --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s "$BASEDIR/../../metaplex/rust/target/deploy/metaplex_token_metadata.so" \
    --ledger /tmp/test-ledger
