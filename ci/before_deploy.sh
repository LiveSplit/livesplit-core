# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    case $TARGET in
        asmjs-unknown-emscripten)
            cross build -p livesplit --target $TARGET --release
            ;;
        wasm32-unknown-emscripten)
            rm target/$TARGET/release/deps/*.wasm 2>/dev/null || :
	        rm target/$TARGET/release/deps/*.js 2>/dev/null || :
            cross build -p livesplit --target $TARGET --release
            ;;
        wasm32-unknown-unknown)
            cross build -p cdylib --target $TARGET --release
            ;;
        *)
            cross rustc -p staticlib --target $TARGET --release
            if [ -z $NO_DYLIB ]; then
                cross rustc -p cdylib --target $TARGET --release
            fi
            ;;
    esac

    (cd capi/bind_gen && cargo run)

    cp -r capi/bindings $stage/
    case $TRAVIS_OS_NAME in
        linux)
            cp target/$TARGET/release/liblivesplit_core.so $stage/liblivesplit_core.so 2>/dev/null || :
            cp target/$TARGET/release/livesplit*.js* $stage/. 2>/dev/null || :
            cp target/$TARGET/release/deps/*.wasm $stage/livesplit.wasm 2>/dev/null || :
            ;;
        osx)
            cp target/$TARGET/release/liblivesplit_core.dylib $stage/liblivesplit_core.dylib 2>/dev/null || :
            ;;
    esac
    cp target/$TARGET/release/liblivesplit_core.a $stage/liblivesplit_core.a 2>/dev/null || :

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
