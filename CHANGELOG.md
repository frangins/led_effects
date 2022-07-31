# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic
Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2022-07-31

### Added

* [sequence] Add a `ConfigWithMainColor` and `ConfigWithSecondaryColor` traits,
  in order to set configuration capabilities to sequences. This replaces the old
  `OneParameterSequence` and `TwoParameterSequence` traits.
* [sequence] Add `GradientConfig`, `RainbowConfig` and `UnicolorConfig` to work
  with the new `Sequence` trait.
* [sequence] Add a `Symmetry` sequence that transforms a sequence to a symmetry.
* [sequence] Add a `Duplicate` sequence that repeats a sequence multiple times.
* [sequence::Rainbow] Add the ability to control the color range of the rainbow.
* [chaser::RandomUnicolor] Add a `set_temperature` method to set the color
  temperature.

### Changed

* **BREAKING**: [sequence::Sequence] Add a `Config` associated type.
* **BREAKING**: [sequence::Sequence] Add a `new` associated function to create a
  sequence from a given `Self::Config`.
* **BREAKING**: [sequence::Sequence] Add a `config` method to get the
  configuration of the sequence.
* **BREAKING**: [sequence::Gradient] Implement `Sequence::new` instead of
  `TwoParameterSequence::new`
* **BREAKING**: [sequence::Rainbow] Implement `Sequence::new` instead of
  `OneParameterSequence::new`
* **BREAKING**: [sequence::Unicolor] Implement `Sequence::new` instead of
  `OneParameterSequence::new`
* **BREAKING**: [chaser::RandomUnicolor] Do not implement the
  `SimpleRandomChaser` trait, because this is a constraining and not really
  useful abstraction.
* **BREAKING**: [chaser::RandomUnicolor] Require a hue distribution in the
  constructor.
* [chaser::RandomUnicolor] Update the ongoing transition time config in
  `set_time_config`.

### Fixed

* [chaser::UnicolorTransition] Fix an issue where they could be a division by
  zero when the transition time is very low.

### Removed

* [sequence] Remove the `OneParameterSequence` trait.
* [sequence] Remove the `TwoParameterSequence` trait.
* [sequence] Remove the `OneParameterSequenceEnum`. If you need such
  abstraction, please implement it in your application code, with only chosen
  sequence types.
* [chaser] Remove the `ChaserEnum`.
* [chaser] Remove the `OneParameterChaser` trait.
* [chaser] Remove the `TwoParameterChaser` trait.
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

[0.2.0]: https://github.com/frangins/led_effects/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/frangins/led_effects/compare/superframe-1.0.0...v0.1.0
[superframe-1.0.0]: https://github.com/frangins/led_effects/releases/tag/superframe-1.0.0
