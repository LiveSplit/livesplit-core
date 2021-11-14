# Changelog

## [0.12.0] - 2021-11-14

- Runs now support custom variables that are key value pairs that either the
  user can specify in the run editor or are provided by a script like an auto
  splitter. [#201](https://github.com/LiveSplit/livesplit-core/pull/201)
- There is now an option in the run editor to generate a comparison based on a
  user specified goal time. This uses the same algorithm as the `Balanced PB`
  comparison but with the time specified instead of the personal best.
  [#209](https://github.com/LiveSplit/livesplit-core/pull/209)
- Images internally are now stored as is without being reencoded as Base64 which
  was done before in order to make it easier for the web LiveSplit One to
  display them. [#227](https://github.com/LiveSplit/livesplit-core/pull/227)
- The Splits.io API is now available under the optional `networking` feature.
  [#236](https://github.com/LiveSplit/livesplit-core/pull/236)
- All key value based components share the same component state type now.
  [#257](https://github.com/LiveSplit/livesplit-core/pull/257)
- The crate now properly supports `wasm-bindgen` and `WASI`.
  [#263](https://github.com/LiveSplit/livesplit-core/pull/263)
- There is now a dedicated component for displaying the comparison's segment
  time. [#264](https://github.com/LiveSplit/livesplit-core/pull/264)
- Compiling the crate without `std` is now supported. Most features are not
  supported at this time though.
  [#270](https://github.com/LiveSplit/livesplit-core/pull/270)
- [`Splitterino`](https://github.com/prefixaut/splitterino) splits can now be
  parsed. [#276](https://github.com/LiveSplit/livesplit-core/pull/276)
- The `Timer` component can now show a segment timer instead.
  [#288](https://github.com/LiveSplit/livesplit-core/pull/288)
- Gamepads are now supported on the web.
  [#310](https://github.com/LiveSplit/livesplit-core/pull/310)
- The underlying "skill curve" that the `Balanced PB` samples is now exposed in
  the API. [#330](https://github.com/LiveSplit/livesplit-core/pull/330)
- The layout states can now be updated, which means almost all of the
  allocations can be reused from the previous frame. This is a lot faster.
  [#334](https://github.com/LiveSplit/livesplit-core/pull/334)
- In order to calculate a layout state, the timer now provides a snapshot
  mechanism that ensures that the layout state gets calculated at a fixed point
  in time. [#339](https://github.com/LiveSplit/livesplit-core/pull/339)
- Text shaping is now done via `rustybuzz` which is a port of `harfbuzz`.
  [#378](https://github.com/LiveSplit/livesplit-core/pull/378)
- Custom fonts are now supported.
  [#385](https://github.com/LiveSplit/livesplit-core/pull/385)
- The renderer is not based on meshes anymore that are suitable for rendering
  with a 3D graphics API. Instead the renderer is now based on paths, which are
  suitable for rendering with a 2D graphics API such as Direct2D, Skia, HTML
  Canvas, and many more. The software renderer is now based on `tiny-skia` which
  is so fast that it actually outperforms any other rendering and is the
  recommended way to render.
  [#408](https://github.com/LiveSplit/livesplit-core/pull/408)
- Remove support for parsing `worstrun` splits. `worstrun` doesn't support
  splits anymore, so `livesplit-core` doesn't need to keep its parsing support.
  [#411](https://github.com/LiveSplit/livesplit-core/pull/411)
- Remove support for parsing `Llanfair 2` splits. `Llanfair 2` was never
  publicly available and is now deleted entirely.
  [#420](https://github.com/LiveSplit/livesplit-core/pull/420)
- Hotkeys are now supported on macOS.
  [#422](https://github.com/LiveSplit/livesplit-core/pull/422)
- The renderer is now based on two layers. A bottom layer that rarely needs to
  be rerendered and the top layer that needs to be rerendered on every frame.
  Additionally the renderer is now a scene manager which manages a scene that an
  actual rendering backend can then render out.
  [#430](https://github.com/LiveSplit/livesplit-core/pull/430)
- The hotkeys are now based on the [UI Events KeyboardEvent code
  Values](https://www.w3.org/TR/uievents-code/) web standard.
  [#440](https://github.com/LiveSplit/livesplit-core/pull/440)
- Timing is now based on `CLOCK_BOOTTIME` on Linux and `CLOCK_MONOTONIC` on
  macOS and iOS. This ensures that all platforms keep tracking time while the
  operating system is in a suspended state.
  [#445](https://github.com/LiveSplit/livesplit-core/pull/445)
- Segment time columns are now formatted as segment times.
  [#448](https://github.com/LiveSplit/livesplit-core/pull/448)
- Hotkeys can now be resolved to the US keyboard layout.
  [#452](https://github.com/LiveSplit/livesplit-core/pull/452)
- They hotkeys are now based on `keydown` instead of `keypress` in the web.
  `keydown` handles all keys whereas `keypress` only handles visual keys and is
  also deprecated. [#455](https://github.com/LiveSplit/livesplit-core/pull/455)
- Hotkeys can now be resolved to the user's keyboard layout on both Windows and
  macOS. [#459](https://github.com/LiveSplit/livesplit-core/pull/459) and
  [#460](https://github.com/LiveSplit/livesplit-core/pull/460)
- The `time` crate is now used instead of `chrono` for keeping track of time.
  [#462](https://github.com/LiveSplit/livesplit-core/pull/462)
- The scene manager now caches a lot more information. This improves the
  performance a lot as it does not need to reshape the text on every frame
  anymore, which is a very expensive operation.
  [#466](https://github.com/LiveSplit/livesplit-core/pull/466) and
  [#467](https://github.com/LiveSplit/livesplit-core/pull/467)
- The hotkeys on Linux are now based on `evdev`, which means Wayland is now
  supported. Additionally the hotkeys are not consuming the key press anymore.
  [#474](https://github.com/LiveSplit/livesplit-core/pull/474)
- When holding down a key, the hotkey doesn't repeat anymore on Linux, macOS and
  WebAssembly. The problem still occurs on Windows at this time.
  [#475](https://github.com/LiveSplit/livesplit-core/pull/475) and
  [#476](https://github.com/LiveSplit/livesplit-core/pull/476)

## [0.11.0] - 2019-05-14

This release focuses a lot on getting rendering working properly outside a web
scenario.

- If the crate is compiled with the `image-shrinking` feature, which is
  activated by default, all images such as game and segment icons that are too
  large are automatically being shrunk to reduce the file size of splits files.
  [#145](https://github.com/LiveSplit/livesplit-core/pull/145)
- [Flitter](https://github.com/alexozer/flitter) splits can now be parsed.
  [#105](https://github.com/LiveSplit/livesplit-core/issues/150)
- The splits component now has support for custom columns.
  [#149](https://github.com/LiveSplit/livesplit-core/pull/149)
- A generic renderer suitable for targeting various graphics frameworks has been
  implemented. It is available via the `rendering` feature.
  [#163](https://github.com/LiveSplit/livesplit-core/pull/163)
- A software renderer using the generic renderer has been implemented as well.
  While certainly slower than the GPU based rendering, it offers portable
  rendering of the layouts without a need for a GPU. It is mostly suitable for
  screenshots. It is available via the `software-rendering` feature.
  [#163](https://github.com/LiveSplit/livesplit-core/pull/163)
- The Layout files of the original LiveSplit can now be imported.
  [#103](https://github.com/LiveSplit/livesplit-core/pull/103)
- Horizontal layouts are now supported in livesplit-core.
  [#180](https://github.com/LiveSplit/livesplit-core/pull/180)
- Hotkeys can now be edited.
  [#152](https://github.com/LiveSplit/livesplit-core/pull/152)

[0.12.0]: https://github.com/linebender/druid/compare/v0.11.0...v0.12.0
[0.11.0]: https://github.com/linebender/druid/compare/v0.10.0...v0.11.0
