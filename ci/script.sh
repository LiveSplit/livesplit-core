# This script takes care of testing your crate

set -ex

main() {
    case $TARGET in
        asmjs-unknown-emscripten)
            cross build -p livesplit --target $TARGET --release
            return
            ;;
        wasm32-unknown-emscripten)
            rm target/wasm32-unknown-emscripten/release/deps/*.wasm 2>/dev/null || :
	        rm target/wasm32-unknown-emscripten/release/deps/*.js 2>/dev/null || :
            cross build -p livesplit --target $TARGET --release
            return
            ;;
        wasm32-unknown-unknown)
            cross build -p cdylib --target $TARGET --release
            return
            ;;
    esac

    cross build -p staticlib --target $TARGET
    # cross build -p staticlib --target $TARGET --release
    if [ -z $NO_DYLIB ]; then
        cross build -p cdylib --target $TARGET
        # cross build -p cdylib --target $TARGET --release
    fi

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test -p livesplit-core --all-features --target $TARGET
    cross test -p livesplit-core --no-default-features --target $TARGET
    # cross test -p livesplit-core --target $TARGET --release

    # cross run --target $TARGET
    # cross run --target $TARGET --release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
