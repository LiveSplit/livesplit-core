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

    # This fetches latest stable release
    local tag=$(git ls-remote --tags --refs --exit-code https://github.com/japaric/cross \
                       | cut -d/ -f3 \
                       | grep -E '^v[0.1.0-9.]+$' \
                       | $sort --version-sort \
                       | tail -n1)

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
        i586-unknown-linux-musl)
            rustup target install $TARGET
            ;;
        arm-unknown-linux-gnueabihf)
            rustup target install $TARGET
            ;;
        arm-unknown-linux-musleabihf)
            rustup target install $TARGET
            ;;
        armv5te-unknown-linux-musleabi)
            rustup target install $TARGET
            ;;
        mipsel-unknown-linux-musl)
            rustup target install $TARGET
            ;;
        mipsisa32r6-unknown-linux-gnu)
            rustup target install $TARGET
            ;;
        mipsisa32r6el-unknown-linux-gnu)
            rustup target install $TARGET
            ;;
        mipsisa64r6-unknown-linux-gnuabi64)
            rustup target install $TARGET
            ;;
        mipsisa64r6el-unknown-linux-gnuabi64)
            rustup target install $TARGET
            ;;
        # FIXME: We are partially staying on 0.1.14 until cross works for these targets again.
        # https://github.com/LiveSplit/livesplit-core/issues/237
        x86_64-unknown-linux-gnux32)
            rustup target install $TARGET
            tag=v0.1.14
            ;;
        armv5te-unknown-linux-gnueabi)
            rustup target install $TARGET
            tag=v0.1.14
            ;;
        powerpc64le-unknown-linux-gnu)
            tag=v0.1.14
            ;;
        i686-unknown-freebsd)
            tag=v0.1.14
            ;;
        x86_64-unknown-freebsd)
            tag=v0.1.14
            ;;
    esac

    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --force \
           --git japaric/cross \
           --tag $tag \
           --target $target
}

main
