set -ex

main() {
    local cargo=cross
    if [ "$SKIP_CROSS" = "skip" ]; then
        cargo=cargo
    fi

    if [ "$TARGET" = "wasm32-wasi" ]; then
        curl https://wasmtime.dev/install.sh -sSf | bash
        export PATH="$HOME/.wasmtime/bin:$PATH"
        $cargo test -p livesplit-core --features software-rendering --target $TARGET
        return
    fi

    $cargo test -p livesplit-core --all-features --target $TARGET
    $cargo test -p livesplit-core --no-default-features --features std --target $TARGET
}

main
