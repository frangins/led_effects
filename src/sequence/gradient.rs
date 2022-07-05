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

use super::{Sequence, TwoParameterSequence};

/// A sequence in which the LEDs draw a gradient.
pub struct Gradient<const N: usize> {
    /// The departure color of the gradient.
    start_color: RGB8,
    /// The arrival color of the gradient.
    end_color: RGB8,
    /// The counter.
    counter: usize,
}

impl<const N: usize> Sequence<N> for Gradient<N> {
    fn get_main_color(&self) -> RGB8 {
        RGB8 {
            r: self.start_color.r + (self.end_color.r - self.start_color.r) / 2,
            g: self.start_color.g + (self.end_color.g - self.start_color.g) / 2,
            b: self.start_color.b + (self.end_color.b - self.start_color.b) / 2,
        }
    }
}

impl<Color: Into<RGB8>, const N: usize> TwoParameterSequence<Color, N>
    for Gradient<N>
{
    fn new(start_color: Color, end_color: Color) -> Self {
        Self {
            start_color: start_color.into(),
            end_color: end_color.into(),
            counter: 0,
        }
    }
}

impl<const N: usize> Iterator for Gradient<N> {
    type Item = RGB8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < N {
            let color = RGB8 {
                r: gradient_step::<N>(
                    self.start_color.r,
                    self.end_color.r,
                    self.counter,
                ),
                g: gradient_step::<N>(
                    self.start_color.g,
                    self.end_color.g,
                    self.counter,
                ),
                b: gradient_step::<N>(
                    self.start_color.b,
                    self.end_color.b,
                    self.counter,
                ),
            };
            self.counter += 1;
            Some(color)
        } else {
            None
        }
    }
}

fn gradient_step<const N: usize>(start: u8, end: u8, step: usize) -> u8 {
    let start_i16 = start as i16;
    let end_i16 = end as i16;
    let step_i16 = step as i16;
    let led_number = N as i16;

    (start_i16 + (step_i16 * (end_i16 - start_i16)) / (led_number - 1)) as u8
}
