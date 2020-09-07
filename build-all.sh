#!/bin/bash
set -e

# Change this as necessary to point a different location of llvm toolchain
LLVM_ROOT=/usr/local/Cellar/llvm/10.0.1

echo "!!! This script adds custom build of Rust toolchain to rustup with name 'custom-wasi'. Ctrl-C now if you want to stop execution !!!" >&2
sleep 10

cd $(dirname $0)

echo "### Building wasmtime" >&2
cd wasmtime
cargo build
cd ..

echo "### Building wasmtime-java" >&2
cd wasmtime-java
./gradlew --no-daemon build
cd ..

echo "### Building decaton" >&2
cd decaton
./gradlew --no-daemon wasmton:build
cd ..

echo "### Building wasi crate" >&2
cd wasi
cargo build --manifest-path=crates/witx-bindgen/Cargo.toml
./target/debug/witx-bindgen ../WebAssembly-WASI/phases/snapshot/witx/wasi_snapshot_preview1.witx > src/lib_generated.rs
cargo build
cd ..

echo "### Fetching and building wasi-libc" >&2
if [ ! -d ./wasi-libc ]; then
    git clone https://github.com/WebAssembly/wasi-libc.git
fi
cd wasi-libc
make WASM_CC=$LLVM_ROOT/bin/clang WASM_AR=$LLVM_ROOT/bin/llvm-ar WASM_NM=$LLVM_ROOT/bin/llvm-nm -j8
wasi_libc_sysroot_path="$(pwd)/sysroot"
cd ..

echo "### Building rust toolchain" >&2
cd rust
sed -i '' -e "s,^wasi-root *= *.*\$,wasi-root = \"$wasi_libc_sysroot_path\"," ./config.toml
./x.py build
echo "### Adding custom rust build to rustup toolchain as 'custom-wasi'" >&2
rustup toolchain link custom-wasi $(pwd)/build/*/stage2
cd ..

for dir in wasm-processor-fileio wasm-processor-socket wasm-processor-redis; do
    echo "### Building $dir" >&2
    cd $dir
    rustup run custom-wasi cargo clean
    rustup run custom-wasi cargo wasi build
    cd ..
done

echo "### All build complete" >&2
