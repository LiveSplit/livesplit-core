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

    $cargo rustc -p livesplit-core-capi --crate-type cdylib --target $TARGET $release_flag $FEATURES
}

main
