# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [Unreleased]

### Added
- add `all` root module.
- add features: `nightly_docs`, `safest`, `unsafe`, `unsafest`, `alloc`.
- add safeguarding for conflicting features.
- add `check.rs`.

### Changed
- update MSRV to `1.71.1`.
- update crate categories.
- update `devela` to `0.7.1`.
- remove `safe` from default features.
- make `Color` trait be `Copy + Debug + PartialEq`.
- update aliases, docs.

### Fixed
- misc. manifest fixes.

## [0.0.7] - 2023-04-08

### Changed
- update depedencies, aliases.

## [0.0.6] - 2023-03-15

### Fixed
- fix docs.rs build, no-std.
- update CI.

## [0.0.5] - 2023-03-13

### Changed
- improve std & no-std feature compilation.
- update dependencies, readme.

### Fixed
- fix tests.

## [0.0.4] - 2023-02-09

### Changed
- update dependencies, readme.

## [0.0.3] - 2023-02-09

### Added
- add optional `rgb` dependency and conversions.

### Fixed
- fix docs.rs build.

## [0.0.2] - 2023-02-08

### Added
- support `tiny-skia` pre-multiplied colors.
- add `safe`, `nighly` features.
- add `devela` dependency.
- add categories and keywords.

### Changed
- rename feature `all` to `complete`.
- make several functions const.
- update clamping functions.
- update dependencies.

## [0.0.1] - 2022-10-06

### Added
- add `Color` trait.
- add conversions from/to arrays and tuples.
- add functions: `linearize32`, `nonlinearize32`.
- add `min`, `max` & `clamp` fns based on PartialOrd.
- add types: `OkLab32`, `OkLch32`, `Srgb8`, `Srgb32`, `LinearSrgb32`, `Srgba8`, `Srgba32`, `LinearSrgba32`.
- add optional dependencies: `libm`, `macroquad`, `notcurses`, `sdl2`, `tiny-skia`.
- add `iunorm` dependency.
- add `no-std` feature.
- add tests.

[unreleased]: https://github.com/andamira/devela/compare/v0.0.7...HEAD
[0.0.7]: https://github.com/andamira/devela/releases/tag/v0.0.7
[0.0.6]: https://github.com/andamira/devela/releases/tag/v0.0.6
[0.0.5]: https://github.com/andamira/devela/releases/tag/v0.0.5
[0.0.4]: https://github.com/andamira/devela/releases/tag/v0.0.4
[0.0.3]: https://github.com/andamira/devela/releases/tag/v0.0.3
[0.0.2]: https://github.com/andamira/devela/releases/tag/v0.0.2
[0.0.1]: https://github.com/andamira/devela/releases/tag/v0.0.1

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html