# Meep NFT

## Installation and Deploy

Make sure you have installed [Solana Tool Suite](https://docs.solana.com/cli/install-solana-cli-tools) and [the Rust Toolchain](https://www.rust-lang.org/tools/install).

```shell
git clone <repository-URL> meep
cd meep

cargo build-bpf
solana program deploy --program-id ./target/deploy/meep-keypair.json \
    ./target/deploy/meep.so
```

## Test on local Validator

Clone Metaplex to a directory next to meep directory:

```shell
cd <meep's parent directory>
git clone https://github.com/metaplex-foundation/metaplex
cd metaplex/rust/token-metadata/program
cargo build-bpf
```

Run from one terminal emulator:
```shell
cd meep
./scripts/init_validator_test.sh
```

Run from another terminal emulator:
```shell
cd meep
cargo test
```

## Instructions

Initialize Meep(0):

0. `[signer, writable]` Authority (Primary creator, Payer)
1. `[signer]` Secondary creator
2. `[writable]` Settings account, PDA("settings\_meep", authority, program\_id)
3. `[]` System program
4. `[]` Rent program

Update Settings(1):

0. `[signer]` Authority (Primary creator, Payer)
1. `[writable]` Settings account, PDA("settings\_meep", authority, program\_id)


Mint NFT(2):

0. `[signer, writable]` Authority (Primary creator, Payer)
1. `[signer]` Secondary creator
2. `[]` Settings account, PDA("settings\_meep", authority, program\_id)
3. `[signer, writable]` Mint account  (Uninitialized)
4. `[signer, writable]` Token account (Uninitialized)
5. `[writable]` TokenMetadata account (Uninitialized)
6. `[writable]` MasterEdition account (Uninitialized)
7. `[]` System program
8. `[]` Token program
9. `[]` Rent program
10. `[]` Metaplex program

## Program ID

Default program ID: `5Hu2bnTxd1mPXNHqMzFfB5SUFEvYW7GG3nPSQ1VWvTK`. It can be changed during deploy:

```shell
solana program deploy --program-id <path-to-cool-looking-program-id> \
    ./target/deploy/meep.so
```
