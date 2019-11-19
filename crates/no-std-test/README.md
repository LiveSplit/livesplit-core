# no_std Testing

In order to test the build for a no_std target, we can't use the same workspace
as the main one as that one includes std-only dev-dependencies that leak their
features into the no_std main dependencies. This is a cargo bug. This crate is
therefore only a temporary workaround.
