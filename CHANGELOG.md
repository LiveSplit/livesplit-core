# Changelog

## [0.13.0] - 2022-12-29

- The `livesplit-hotkey` crate is now documented. (@CryZe)
  [#479](https://github.com/LiveSplit/livesplit-core/pull/479)
- Not every key press emits a scan code on Windows. For those the virtual key
  code is now translated to a scan code. (@CryZe)
  [#480](https://github.com/LiveSplit/livesplit-core/pull/480)
- Time parsing is now a lot more robust, handles more edge cases, and is also a
  lot more accurate. (@CryZe)
  [#483](https://github.com/LiveSplit/livesplit-core/pull/483) and
  [#578](https://github.com/LiveSplit/livesplit-core/pull/578)
- When parsing a GDI based font name, platforms other than Windows now don't
  attempt to parse "normal" as part of the font name anymore as it is too
  ambigious. It could either refer to a font weight or stretch. (@kadiwa4)
  [#487](https://github.com/LiveSplit/livesplit-core/pull/487)
- The text engine can now be customized. You can either provide your own text
  engine or use the one provided by `livesplit-core`. The one provided is now
  behind the `path-based-text-engine` and converts all glyphs to paths that can
  easily be drawn. (@CryZe)
  [#495](https://github.com/LiveSplit/livesplit-core/pull/495)
- The path based text engine now caches the width of digits for tabular numbers,
  as well as the ellipsis glyph and its width, so that they can be layed out
  faster. (@kadiwa4)
  [#490](https://github.com/LiveSplit/livesplit-core/pull/490) and
  [#499](https://github.com/LiveSplit/livesplit-core/pull/499)
- On Windows GDI is now used to resolve GDI based font names. (@CryZe)
  [#500](https://github.com/LiveSplit/livesplit-core/pull/500)
- (Total) Possible Time Save now properly indicates that it's updating
  frequently. This results in faster rendering times. (@kadiwa4)
  [#501](https://github.com/LiveSplit/livesplit-core/pull/501)
- Initial support for auto splitting has landed in `livesplit-core`. Auto
  splitters are provided as WebAssembly modules. Support can be activated via
  the `auto-splitting` feature. (@P1n3appl3)
  [#477](https://github.com/LiveSplit/livesplit-core/pull/477)
- Auto splitting is also supported via the C API when activating its
  `auto-splitting` feature. (@DarkRTA)
  [#503](https://github.com/LiveSplit/livesplit-core/pull/503)
- A watchdog for the Auto Splitting Runtime was added which unloads scripts that
  aren't responsive. (@CryZe)
  [#528](https://github.com/LiveSplit/livesplit-core/pull/528)
- Splits and layouts can now be parsed and saved on `no_std` platforms. (@CryZe)
  [#532](https://github.com/LiveSplit/livesplit-core/pull/532)
- The splits component column labels can now be queried via the C API.
  (@MichaelJBerk) [#526](https://github.com/LiveSplit/livesplit-core/pull/526)
- The Software Renderer is now supported on `no_std` platforms. (@CryZe)
  [#536](https://github.com/LiveSplit/livesplit-core/pull/536)
- The parsers are now faster because they don't allocate as much memory anymore.
  (@CryZe) [#546](https://github.com/LiveSplit/livesplit-core/pull/546)
- The auto splitters have unstable support the `WebAssembly System Interface`
  via the `unstable-auto-splitting` feature. (@CryZe)
  [#547](https://github.com/LiveSplit/livesplit-core/pull/547)
- The Timer component can now use the color of the delta for its background.
  (@Hurricane996) [#539](https://github.com/LiveSplit/livesplit-core/pull/539)
- The splits component now takes the font into account when calculating the
  width of the columns. (@Hurricane996)
  [#550](https://github.com/LiveSplit/livesplit-core/pull/550)
- The `Resource Allocator` now decodes the images, allowing the underlying
  renderer to do the encoding by itself. (@CryZe)
  [#562](https://github.com/LiveSplit/livesplit-core/pull/562)
- Cargo's `--crate-type` parameter is now used to build the C API. (@CryZe)
  [#565](https://github.com/LiveSplit/livesplit-core/pull/565)
- The columns of the splits component can now show the custom variables.
  (@CryZe) [#566](https://github.com/LiveSplit/livesplit-core/pull/566)
- On the web, the `keydown` event may not always pass a `KeyboardEvent` despite
  the specification saying that this should be the case. This is now properly
  handled. (@CryZe) [#567](https://github.com/LiveSplit/livesplit-core/pull/567)
- An integer overflow in the `FuzzyList` used for searching game and category
  names has been fixed. (@CryZe)
  [#569](https://github.com/LiveSplit/livesplit-core/pull/569)
- The way the background is handled in the Detailed Timer component has been
  fixed. (@CryZe) [#572](https://github.com/LiveSplit/livesplit-core/pull/572)
- The times are now formatted as strings without going through floating point
  numbers which increases both the correctness and the performance. (@CryZe)
  [#576](https://github.com/LiveSplit/livesplit-core/pull/576)
- Instead of using `core::fmt` formatting machinery to format the times as
  strings, we now use a custom implementation that's much faster. (@CryZe)
  [#577](https://github.com/LiveSplit/livesplit-core/pull/577) and
  [#580](https://github.com/LiveSplit/livesplit-core/pull/580)
- Holding down a hotkey on Windows now doesn't cause it to be triggered over and
  over again. Other platforms already behaved this way. (@CryZe)
  [#584](https://github.com/LiveSplit/livesplit-core/pull/584)
- The `base64` crate is now replaced with `base64-simd` which uses SIMD to speed
  up the decoding of the images. (@CryZe)
  [#585](https://github.com/LiveSplit/livesplit-core/pull/585)
- Splits from `SpeedRunIGT`, which is a Minecraft speedruning mod, can now be
  parsed. (@CryZe) [#591](https://github.com/LiveSplit/livesplit-core/pull/591)
- It turns out using `evdev` for the hotkeys on Linux requires the user to be in
  the `input` group, which is not always the case. Therefore we now fall back to
  `X11` if `evdev` is not usable. (@CryZe)
  [#592](https://github.com/LiveSplit/livesplit-core/pull/592)
- When an auto splitter wants to attach to a Process by name, the start time and
  process id are now used to prioritize duplicate processes. (@Eein)
  [#589](https://github.com/LiveSplit/livesplit-core/pull/589)
- It is now possible to resolve the key codes to the particular name of the key
  based on the current keyboard layout on Linux and the web. This was already
  the case on Windows and macOS. (@CryZe)
  [#594](https://github.com/LiveSplit/livesplit-core/pull/594) and
  [#595](https://github.com/LiveSplit/livesplit-core/pull/595)
- It is now possible to trust the user of the C API to always pass valid UTF-8
  strings to the C API via the optional `assume-str-parameters-are-utf8`
  feature. This is also always the case when using WebAssembly on the web. This
  improves the performance because no validation of the strings is necessary.
  (@CryZe) [#597](https://github.com/LiveSplit/livesplit-core/pull/597)
- There is now a new `max-opt` cargo profile that can be used to maximally
  optimize the resulting executable. The release profile is now using its
  default configuration again. (@CryZe)
  [#598](https://github.com/LiveSplit/livesplit-core/pull/598)
- When encountering images `livesplit-core` checks their dimensions to
  potentially automatically shrink them if they are larger than necessary. It
  turns out that checking the dimensions of PNG images was a lot less efficient
  than it could have been. This even improves parsing speed of entire splits
  files by up to 30%. (@CryZe)
  [#600](https://github.com/LiveSplit/livesplit-core/pull/600)
- The documentation now uses links to types mentioned. (@Eein)
  [#596](https://github.com/LiveSplit/livesplit-core/pull/596)
- Auto splitters can now query size of the modules of a process. (@CryZe)
  [#602](https://github.com/LiveSplit/livesplit-core/pull/602)
- The log messages emitted by auto splitters can now be consumed directly
  instead of always being emitted via the `log` crate. (@CryZe)
  [#603](https://github.com/LiveSplit/livesplit-core/pull/603)
- The auto splitters can provide settings that can be configured. For now the
  auto splitters need to be reloaded when the settings change. (@CryZe)
  [#606](https://github.com/LiveSplit/livesplit-core/pull/606)
- The file path used to be tracked in the `Run`, but no frontend even used this.
  So it has been removed. (@CryZe)
  [#616](https://github.com/LiveSplit/livesplit-core/pull/616)
- The documentation states that the title component's lines store the
  unabbreviated line as their last element. This was not actually the case and
  has been fixed. (@DarkRTA)
  [#615](https://github.com/LiveSplit/livesplit-core/pull/615)

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

[0.13.0]: https://github.com/linebender/druid/compare/v0.12.0...v0.13.0
[0.12.0]: https://github.com/linebender/druid/compare/v0.11.0...v0.12.0
[0.11.0]: https://github.com/linebender/druid/compare/v0.10.0...v0.11.0
