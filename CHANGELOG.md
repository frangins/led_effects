# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic
Versioning](https://semver.org/spec/v2.0.0.html).

# [Unreleased]

### Changed

* **BREAKING**: [chaser::RandomUnicolor] Do not implement the
  `SimpleRandomChaser` trait, because this is a constraining and not really
  useful abstraction.
* [chaser::RandomUnicolor] Update the ongoing transition time config in
  `set_time_config`.

### Fixed

* [chaser::UnicolorTransition] Fix an issue where they could be a division by
  zero when the transition time is very low.

### Removed

* [chaser] Remove the `SimpleRandomChaser` trait.

## [0.1.0] - 2022-05-29

### Changed

* Extract to its own repo.
* Update to Rust 2021.
* Forbid unsafe code.

## [superframe-1.0.0] - 2021-07-18

### Added

* Initial version, part of [superframe](https://github.com/legrec14/superframe),
  featuring:
    * an abstraction over *sequences* of LEDs, representing the static state of
      a set of LEDs,
    * an abstraction over *chasers*, representing an iterable succession of LED
      sequences,
    * unicolor, gradient and rainbow sequences,
    * unicolor transition, randow unicolor, and cyclic rainbow chasers.

[Unreleased]: https://github.com/frangins/led_effects/compare/main...develop
[0.1.0]: https://github.com/frangins/led_effects/compare/superframe-1.0.0...v0.1.0
[superframe-1.0.0]: https://github.com/frangins/led_effects/releases/tag/superframe-1.0.0
