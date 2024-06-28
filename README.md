# riscv-evm

A Revm compatible RISC-V EVM implementation.

## Code Structure

The whole project can be divided into 4 parts:

### Riscv vm

The code of this part can be run on native target, like linux and macOS.

* `eth-riscv-interperter`: load ELF code to riscv interperter and execute.
* `eth-riscv-syscalls`: Add EVM opcode riscv syscall mapping.

### EVM Rust sdk

The code of this part can only be run on riscv target: `riscv64imac-unknown-none-elf`

* `eth-riscv-runtime`: EVM runtime, EVM opcode mapping, EVM types like mapping, memory allocation, etc.
* `contract-derive`: contract proc macro for easier contract development.

### Contract examples

The code of this part can only be run on riscv target: `riscv64imac-unknown-none-elf`

* `asm-runtime-elf`: Assembly Riscv ELF sample code.
* `c-runtime-elf`: C Riscv ELF sample code.
* `erc20`: Rust erc20 contract sample code, will be used by r55 e2e test

### e2e test project

The code of this part can be run on native target, like linux and macOS.

* `r55`: runs the whole testing workflow, testing the erc20 sample code.

## Prerequisites


1. install rust riscv toolchain: `rustup target add riscv64imac-unknown-none-elf --toolchain nightly`
2. install gnu riscv toolchains:


Ubuntu Linux

```sh
sudo apt install -y gcc-riscv64-unknown-elf
```

Arch Linux

```sh
yay -S riscv64-unknown-elf-gcc
```

## Build && Testing steps

build asm and c samples

```sh
cd examples/asm-runtime-elf/
make
cd examples/c-runtime-elf/
make
```

test riscv interpreter

```sh
cd eth-riscv-interpreter
cargo build 
cargo test
```

test erc20 contract

```sh
cd examples/erc20
cargo +nightly-2024-02-01 build -r --lib -Z build-std=core,alloc --target riscv64imac-unknown-none-elf --bin runtime
cargo +nightly-2024-02-01 build -r --lib -Z build-std=core,alloc --target riscv64imac-unknown-none-elf --bin deploy
cd ../.. # return to project root directory
cargo build --release
./target/release/r55
```
