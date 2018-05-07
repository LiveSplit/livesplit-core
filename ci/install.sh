set -ex

main() {
    local target=
    if [ $TRAVIS_OS_NAME = linux ]; then
        target=x86_64-unknown-linux-musl
        sort=sort
    else
        target=x86_64-apple-darwin
        sort=gsort  # for `sort --sort-version`, from brew's coreutils.
    fi

    case $TARGET in
        aarch64-apple-ios)
            rustup target install $TARGET
            ;;
        armv7-apple-ios)
            rustup target install $TARGET
            ;;
        armv7s-apple-ios)
            rustup target install $TARGET
            ;;
        i386-apple-ios)
            rustup target install $TARGET
            ;;
        x86_64-apple-ios)
            rustup target install $TARGET
            ;;
        wasm32-unknown-unknown)
            rustup target install $TARGET
            ;;
        x86_64-unknown-linux-gnux32)
            rustup target install $TARGET
            ;;
        i586-unknown-linux-musl)
            rustup target install $TARGET
            ;;
        armv5te-unknown-linux-gnueabi)
            rustup target install $TARGET
            ;;
    esac

    # This fetches latest stable release
    local tag=$(git ls-remote --tags --refs --exit-code https://github.com/japaric/cross \
                       | cut -d/ -f3 \
                       | grep -E '^v[0.1.0-9.]+$' \
                       | $sort --version-sort \
                       | tail -n1)

    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --force \
           --git japaric/cross \
           --tag $tag \
           --target $target
}

main
