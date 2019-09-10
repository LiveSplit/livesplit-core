set -ex

main() {
    local target=
    if [ "$OS_NAME" = "ubuntu-latest" ]; then
        target=x86_64-unknown-linux-musl
        sort=sort
    else
        target=x86_64-apple-darwin
        sort=gsort
    fi

    # This fetches latest stable release
    # local tag=$(git ls-remote --tags --refs --exit-code https://github.com/japaric/cross \
    #                    | cut -d/ -f3 \
    #                    | grep -E '^v[0.1.0-9.]+$' \
    #                    | $sort --version-sort \
    #                    | tail -n1)

    # FIXME: We use a custom pre-release version that works with GitHub Actions.
    # Also don't forget to also change back from CryZe/cross down there.
    # https://github.com/LiveSplit/livesplit-core/issues/237
    local tag=v0.1.16-pre

    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --force \
           --git CryZe/cross \
           --tag $tag \
           --target $target
}

main
