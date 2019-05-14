# 0.11.0 (2019-05-14)

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
