# This script takes care of testing your crate

set -ex

# TODO This is the "test phase", tweak it as you see fit
main() {
    cross build -p livesplit-core-capi --target $TARGET
    cross build -p livesplit-core-capi --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test -p livesplit-core-capi --target $TARGET
    cross test -p livesplit-core-capi --target $TARGET --release

    # cross run --target $TARGET
    # cross run --target $TARGET --release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
