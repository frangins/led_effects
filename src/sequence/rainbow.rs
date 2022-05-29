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

use smart_leds::{hsv::Hsv, RGB8};

use super::{OneParameterSequence, Sequence};

/// A sequence in which the LEDs draw a rainbow.
pub struct Rainbow<const N: usize> {
    /// The first color of the rainbow.
    first_color: Hsv,
    /// The counter.
    counter: usize,
}

impl<const N: usize> Sequence<N> for Rainbow<N> {}

impl<Color: Into<Hsv>, const N: usize> OneParameterSequence<Color, N>
    for Rainbow<N>
{
    fn new(first_color: Color) -> Self {
        Self {
            first_color: first_color.into(),
            counter: 0,
        }
    }
}

impl<const N: usize> Iterator for Rainbow<N> {
    type Item = RGB8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < N {
            let color = Hsv {
                hue: self.first_color.hue + (self.counter * (255 / N)) as u8,
                ..self.first_color // sat: self.first_color.sat, val: self.first_color.val
            };
            self.counter += 1;
            Some(color.into())
        } else {
            None
        }
    }
}
