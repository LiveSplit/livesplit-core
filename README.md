<h1> <img src="https://raw.githubusercontent.com/LiveSplit/LiveSplit/master/LiveSplit/Resources/Icon.png" alt="LiveSplit" height="42" width="45" align="top"/> livesplit-core</h1>

[![Build Status](https://travis-ci.org/LiveSplit/livesplit-core.svg?branch=master)](https://travis-ci.org/LiveSplit/livesplit-core)
[![Build status](https://ci.appveyor.com/api/projects/status/bvv4un099w94kari/branch/master?svg=true)](https://ci.appveyor.com/project/CryZe/livesplit-core/branch/master)
[![crates.io](https://img.shields.io/crates/v/livesplit-core.svg)](https://crates.io/crates/livesplit-core)
[![npm](https://img.shields.io/npm/v/livesplit-core.svg)](https://www.npmjs.com/package/livesplit-core)
[![docs.rs](https://docs.rs/livesplit-core/badge.svg)](https://docs.rs/livesplit-core/)

livesplit-core is a library that provides a lot of functionality for creating a speedrun timer.
It can be used directly from Rust.
Additional Bindings are available for the following programming languages:

 - C
 - C++
 - C#
 - Java with Java Native Access or Java Native Interface
 - Kotlin with Java Native Interface
 - Swift
 - Ruby
 - Python
 - JavaScript + TypeScript for Node.js, asm.js and WebAssembly

The Documentation for the Library is available here: [API Documentation](https://docs.rs/livesplit-core/)

## Build Instructions

You can install Rust with the Installer available on [rustup.rs](https://rustup.rs/).
Clone the repository and build the library with the following command:

```
cargo build --release -p livesplit-core-capi
```

The library will then be available as a shared and static library in the `target` folder.

If you want to build the Bindings for the library too, you need to go into the `capi/bind_gen` folder and run the following command:

```
cargo run
```

The bindings will then be available in `capi/bindings`.

## Download

Builds for a lot of common platforms are available over here: [Releases](https://github.com/LiveSplit/livesplit-core/releases)
