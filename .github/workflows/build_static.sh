set -ex

main() {
    local cargo=cross
    if [ "$SKIP_CROSS" = "skip" ]; then
        cargo=cargo
    fi
    local release_flag=""
    if [ "$IS_DEPLOY" = "true" ]; then
        release_flag="--profile max-opt"
    fi

    if [ "$NO_STD" = "true" ]; then
        cargo build --target $TARGET --no-default-features --features software-rendering $FEATURES
        return
    fi

    case $TARGET in
        wasm32-unknown-unknown)
            $cargo rustc -p livesplit-core-capi --crate-type cdylib --target $TARGET $release_flag $FEATURES
            ;;
        wasm32-wasi)
            $cargo rustc -p livesplit-core-capi --crate-type cdylib --target $TARGET $release_flag $FEATURES
            ;;
        *)
            $cargo rustc -p livesplit-core-capi --crate-type staticlib --target $TARGET $release_flag $FEATURES
            ;;
    esac
}

main
