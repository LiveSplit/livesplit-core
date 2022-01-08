set -ex

main() {
    local cargo=cross

    # all features except those that don't easily work with cross such as
    # font-loading or auto-splitting.
    local all_features="--features std,more-image-formats,image-shrinking,rendering,software-rendering,wasm-web,networking"

    if [ "$SKIP_CROSS" = "skip" ]; then
        cargo=cargo
        if [ "$SKIP_AUTO_SPLITTING" -ne "skip" ]; then
            all_features="--all-features"
        fi
    fi

    if [ "$TARGET" = "wasm32-wasi" ]; then
        curl https://wasmtime.dev/install.sh -sSf | bash
        export PATH="$HOME/.wasmtime/bin:$PATH"
        $cargo test -p livesplit-core --features software-rendering --target $TARGET
        return
    fi

    $cargo test -p livesplit-core $all_features --target $TARGET
    $cargo test -p livesplit-core --no-default-features --features std --target $TARGET
}

main
