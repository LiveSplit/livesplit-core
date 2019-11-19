set -ex

main() {
    local cargo=cross
    if [ "$SKIP_CROSS" = "skip" ]; then
        cargo=cargo
    fi

    $cargo test -p livesplit-core --all-features --target $TARGET
    $cargo test -p livesplit-core --no-default-features --features std --target $TARGET
}

main
