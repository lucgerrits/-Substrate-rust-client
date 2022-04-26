# Rust Version of substrate client

```bash
#set requirements
rustup update
rustup update nightly
rustup default nightly
rustup target add wasm32-unknown-unknown --toolchain nightly

#build all the scripts
cargo build --release
```

## Set storage 

```bash
#run
./target/release/set_storage

#if need to give node endpoint
./target/release/set_storage -port 9944 -url ws://127.0.0.1
```
## Get storage

```bash
#run
./target/release/get_storage

#if need to give node endpoint
./target/release/get_storage -port 9944 -url ws://127.0.0.1
```

## Debug

To enable debut and view all in/out communications, execute the scripts with `RUST_LOG=debug` or `RUST_LOG=info`.

```bash
#example
RUST_LOG=debug ./target/release/get_storage
#example
RUST_LOG=info ./target/release/get_storage
```

### Cross-compile

Require the right toolchain: 
- ARM [here](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-a/downloads)
- RISC-V [here](https://github.com/riscv-collab/riscv-gnu-toolchain)