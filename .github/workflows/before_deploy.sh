set -ex

main() {
    local tag=$(git tag --points-at HEAD)
    local src=$(pwd) \
          stage=

    if [ "$OS_NAME" = "macOS-latest" ]; then
        stage=$(mktemp -d -t tmp)
    else
        stage=$(mktemp -d)
    fi

    (cd capi/bind_gen && cargo run)

    cp -r capi/bindings $stage/
    cp target/$TARGET/release/livesplit_core.dll $stage/livesplit_core.dll 2>/dev/null || :
    cp target/$TARGET/release/livesplit_core.lib $stage/livesplit_core.lib 2>/dev/null || :
    cp target/$TARGET/release/liblivesplit_core.a $stage/liblivesplit_core.a 2>/dev/null || :
    cp target/$TARGET/release/liblivesplit_core.so $stage/liblivesplit_core.so 2>/dev/null || :
    cp target/$TARGET/release/livesplit*.js* $stage/. 2>/dev/null || :
    cp target/$TARGET/release/deps/*.wasm $stage/livesplit.wasm 2>/dev/null || :
    cp target/$TARGET/release/liblivesplit_core.dylib $stage/liblivesplit_core.dylib 2>/dev/null || :

    cd $stage
    if [ "$OS_NAME" = "windows-latest" ]; then
        7z a $src/livesplit-core-$tag-$TARGET.zip *
    else
        tar czf $src/livesplit-core-$tag-$TARGET.tar.gz *
    fi
    cd $src

    rm -rf $stage
}

main
