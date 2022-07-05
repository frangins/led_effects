// led_effects - A collection of LED effects on top of smart-leds.
// Copyright (C) 2021 Guillaume Cugnet <guillaume@cugnet.eu>
// Copyright (C) 2021 Jean-Philippe Cugnet <jean-philippe@cugnet.eu>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! A collection of LED sequences on top of `smart_leds`.

mod gradient;
mod rainbow;
mod unicolor;

pub use gradient::Gradient;
pub use rainbow::Rainbow;
pub use unicolor::Unicolor;

use smart_leds::{hsv::Hsv, RGB8};

/// A LED sequence.
pub trait Sequence<const N: usize>: Iterator {
    fn get_main_color(&self) -> RGB8;
}

/// A LED sequence with one parameter.
pub trait OneParameterSequence<Color, const N: usize>: Sequence<N> {
    fn new(color: Color) -> Self;
}

/// A LED sequence with two parameters.
pub trait TwoParameterSequence<Color, const N: usize>: Sequence<N> {
    fn new(color1: Color, color2: Color) -> Self;
}

/// Container enum for one-parameter sequences.
pub enum OneParameterSequenceEnum<const N: usize> {
    UnicolorRgb8(Unicolor<RGB8, N>),
    UnicolorHsv(Unicolor<Hsv, N>),
    Rainbow(Rainbow<N>),
}

impl<const N: usize> From<Unicolor<RGB8, N>> for OneParameterSequenceEnum<N> {
    fn from(sequence: Unicolor<RGB8, N>) -> Self {
        OneParameterSequenceEnum::UnicolorRgb8(sequence)
    }
}

impl<const N: usize> From<Unicolor<Hsv, N>> for OneParameterSequenceEnum<N> {
    fn from(sequence: Unicolor<Hsv, N>) -> Self {
        OneParameterSequenceEnum::UnicolorHsv(sequence)
    }
}

impl<const N: usize> From<Rainbow<N>> for OneParameterSequenceEnum<N> {
    fn from(sequence: Rainbow<N>) -> Self {
        OneParameterSequenceEnum::Rainbow(sequence)
    }
}

impl<const N: usize> Sequence<N> for OneParameterSequenceEnum<N> {
    fn get_main_color(&self) -> RGB8 {
        match self {
            OneParameterSequenceEnum::UnicolorRgb8(sequence) => {
                sequence.get_main_color()
            }
            OneParameterSequenceEnum::UnicolorHsv(sequence) => {
                sequence.get_main_color()
            }
            OneParameterSequenceEnum::Rainbow(sequence) => {
                sequence.get_main_color()
            }
        }
    }
}

impl<const N: usize> Iterator for OneParameterSequenceEnum<N> {
    type Item = RGB8;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            OneParameterSequenceEnum::UnicolorRgb8(sequence) => sequence.next(),
            OneParameterSequenceEnum::UnicolorHsv(sequence) => sequence.next(),
            OneParameterSequenceEnum::Rainbow(sequence) => sequence.next(),
        }
    }
}
