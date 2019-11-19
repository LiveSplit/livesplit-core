set -ex

main() {
    local cargo=cross
    if [ "$SKIP_CROSS" = "skip" ]; then
        cargo=cargo
    fi
    local release_flag=""
    if [ "$IS_DEPLOY" = "true" ]; then
        release_flag="--release"
    fi

    if [ "$NO_STD" = "true" ]; then
        (cd crates/no-std-test && cargo build --target $TARGET $FEATURES)
        return
    fi

    case $TARGET in
        asmjs-unknown-emscripten)
            $cargo build -p livesplit --target $TARGET --release $FEATURES
            ;;
        wasm32-unknown-emscripten)
            rm target/wasm32-unknown-emscripten/release/deps/*.wasm 2>/dev/null || :
            rm target/wasm32-unknown-emscripten/release/deps/*.js 2>/dev/null || :
            $cargo build -p livesplit --target $TARGET --release $FEATURES
            ;;
        wasm32-unknown-unknown)
            $cargo build -p cdylib --target $TARGET --release $FEATURES
            ;;
        *)
            $cargo build -p staticlib --target $TARGET $release_flag $FEATURES
            ;;
    esac
}

main
