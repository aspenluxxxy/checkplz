#!/bin/bash
rm -f checknit* || true
RUST_DIR=`readlink -f "$PWD/../build/sysroot/rust/bin"`
env RUSTC="$RUST_DIR/rustc" "$RUST_DIR/cargo" build --release || exit 1
llvm-strip --strip-all target/x86_64-unknown-linux-musl/release/checknit || exit 1
sstrip -z target/x86_64-unknown-linux-musl/release/checknit || exit 1
cp target/x86_64-unknown-linux-musl/release/checknit . || exit 1
du -sh checknit || exit 1
cp -f checknit "$PWD/../build/initramfs/init" || exit 1
