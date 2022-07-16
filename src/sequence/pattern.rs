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

use smart_leds::{colors::BLACK, RGB8};

use super::{ConfigWithMainColor, ConfigWithOffset, Sequence};

/// A sequence in which LEDs have a color or not.
pub struct Pattern<Color, const N: usize> {
    /// The configuration.
    config: PatternConfig<Color, N>,
    /// The counter.
    counter: usize,
}

// TODO: Optimise this abomination.
/// The configuration for pattern sequences.
#[derive(Debug, Copy, Clone)]
pub struct PatternConfig<Color, const N: usize> {
    /// The color for on LEDs.
    pub color: Color,
    /// The pattern.
    pub pattern: [bool; N],
    /// The minimum valid offset.
    pub min_offset: i32,
    /// The maximum valid offset.
    pub max_offset: i32,
    /// The offset of the pattern.
    pub offset: i32,
}

impl<Color: Copy + Into<RGB8>, const N: usize> Sequence<N>
    for Pattern<Color, N>
{
    type Config = PatternConfig<Color, N>;

    fn new(config: Self::Config) -> Self {
        Self { config, counter: 0 }
    }

    fn config(&self) -> Self::Config {
        self.config
    }
}

impl<Color: Copy + Into<RGB8>, const N: usize> Iterator for Pattern<Color, N> {
    type Item = RGB8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < N {
            let index = self.counter as i32 - self.config.offset;
            let color = if index >= 0
                && (index as usize) < N
                && self.config.pattern[index as usize]
            {
                self.config.color.into()
            } else {
                BLACK
            };

            self.counter += 1;
            Some(color)
        } else {
            None
        }
    }
}

impl<Color: Copy + Into<RGB8> + From<RGB8>, const N: usize> ConfigWithMainColor
    for PatternConfig<Color, N>
{
    fn main_color(&self) -> RGB8 {
        self.color.into()
    }

    fn set_main_color(&mut self, color: RGB8) {
        self.color = color.into();
    }
}

impl<Color: Copy + Into<RGB8> + From<RGB8>, const N: usize> ConfigWithOffset
    for PatternConfig<Color, N>
{
    fn min_offset(&self) -> i32 {
        self.min_offset
    }

    fn max_offset(&self) -> i32 {
        self.max_offset
    }

    fn offset(&self) -> i32 {
        self.offset
    }

    fn set_offset(&mut self, offset: i32) -> bool {
        if offset >= self.min_offset && offset <= self.max_offset {
            self.offset = offset;
            true
        } else {
            false
        }
    }

    fn increment_offset(&mut self, offset: i32) -> bool {
        if self.offset >= self.min_offset && self.offset <= self.max_offset {
            self.offset += offset;
            true
        } else {
            false
        }
    }
}
