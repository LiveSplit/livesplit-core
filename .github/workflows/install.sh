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
    local tag=$(git ls-remote --tags --refs --exit-code https://github.com/rust-embedded/cross \
                       | cut -d/ -f3 \
                       | grep -E '^v[0.1.0-9.]+$' \
                       | $sort --version-sort \
                       | tail -n1)

    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --force \
           --git rust-embedded/cross \
           --tag $tag \
           --target $target
}

main
