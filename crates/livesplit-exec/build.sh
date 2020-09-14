#!/bin/sh
cargo build --release
cargo build --release --target i686-unknown-linux-gnu
mkdir -p out/bin out/lib
cp ../../target/release/livesplit-exec out/bin
cp ../../target/release/liblivesplit_exec.so out/lib/liblivesplit-exec.so
cp ../../target/i686-unknown-linux-gnu/release/liblivesplit_exec.so out/lib/liblivesplit-exec32.so
