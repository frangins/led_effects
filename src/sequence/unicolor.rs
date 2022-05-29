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

use smart_leds::RGB8;

use super::{OneParameterSequence, Sequence};

/// A sequence in which all LEDs have the same color.
pub struct Unicolor<Color, const N: usize> {
    /// The color for all LEDs.
    color: Color,
    /// The counter.
    counter: usize,
}

impl<Color: Copy + Into<RGB8>, const N: usize> Sequence<N>
    for Unicolor<Color, N>
{
}

impl<Color: Copy + Into<RGB8>, const N: usize> OneParameterSequence<Color, N>
    for Unicolor<Color, N>
{
    fn new(color: Color) -> Self {
        Self { color, counter: 0 }
    }
}

impl<Color: Copy + Into<RGB8>, const N: usize> Iterator for Unicolor<Color, N> {
    type Item = RGB8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < N {
            self.counter += 1;
            Some(self.color.into())
        } else {
            None
        }
    }
}
