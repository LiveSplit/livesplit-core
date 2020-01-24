# <img src="https://raw.githubusercontent.com/LiveSplit/LiveSplit/master/LiveSplit/Resources/Icon.png" alt="LiveSplit" height="42" width="45" align="top"/> livesplit-core

[![Build Status](https://github.com/LiveSplit/livesplit-core/workflows/Rust/badge.svg)](https://github.com/LiveSplit/livesplit-core/actions)
[![crates.io](https://img.shields.io/crates/v/livesplit-core.svg)](https://crates.io/crates/livesplit-core)
[![npm](https://img.shields.io/npm/v/livesplit-core.svg)](https://www.npmjs.com/package/livesplit-core)
[![docs.rs](https://docs.rs/livesplit-core/badge.svg)](https://docs.rs/livesplit-core/)
[![dependency status](https://deps.rs/repo/github/LiveSplit/livesplit-core/status.svg)](https://deps.rs/repo/github/LiveSplit/livesplit-core)

livesplit-core is a library that provides a lot of functionality for creating a
speedrun timer. It can be used directly from Rust. Additional bindings are
available for the following programming languages:

- C
- C++
- C#
- Java with Java Native Access or Java Native Interface
- Kotlin with Java Native Interface
- Swift
- Ruby
- Python
- JavaScript + TypeScript for Node.js and WebAssembly

The documentation is available here:

- [Rust Documentation](https://docs.rs/livesplit-core/)
- [TypeScript Documentation](https://livesplit.org/livesplit-core-docs/)

## Build Instructions

You can install Rust with the installer available on [rustup.rs](https://rustup.rs/).
Clone the repository and build the library with the following command:

```bash
# Dynamic Library
cargo build --release -p cdylib
# Static Library
cargo build --release -p staticlib
```

The library will then be available as a shared and static library in the `target` folder.

If you want to build the bindings for the library too, you need to go into the
`capi/bind_gen` folder and run the following command:

```bash
cargo run
```

The bindings will then be available in `capi/bindings`.

## Download

Builds for a lot of common platforms are available in the [Releases](https://github.com/LiveSplit/livesplit-core/releases).

## License

Licensed under either of
  * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
    http://www.apache.org/licenses/LICENSE-2.0)
  * MIT license ([LICENSE-MIT](LICENSE-MIT) or
    http://opensource.org/licenses/MIT) at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
