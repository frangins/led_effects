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

use super::{ConfigWithMainColor, ConfigWithSecondaryColor, Sequence};

/// A sequence in which the LEDs draw a gradient.
pub struct Gradient<const N: usize> {
    /// The configuration.
    config: GradientConfig,
    /// The counter.
    counter: usize,
}

/// The configuration for gradient sequences.
#[derive(Debug, Clone, Copy)]
pub struct GradientConfig {
    /// The departure color of the gradient.
    pub start_color: RGB8,
    /// The arrival color of the gradient.
    pub end_color: RGB8,
}

impl<const N: usize> Sequence<N> for Gradient<N> {
    type Config = GradientConfig;

    fn new(config: Self::Config) -> Self {
        Self { config, counter: 0 }
    }

    fn config(&self) -> Self::Config {
        self.config
    }
}

impl<const N: usize> Iterator for Gradient<N> {
    type Item = RGB8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < N {
            let color = RGB8 {
                r: gradient_step::<N>(
                    self.config.start_color.r,
                    self.config.end_color.r,
                    self.counter,
                ),
                g: gradient_step::<N>(
                    self.config.start_color.g,
                    self.config.end_color.g,
                    self.counter,
                ),
                b: gradient_step::<N>(
                    self.config.start_color.b,
                    self.config.end_color.b,
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

impl ConfigWithMainColor for GradientConfig {
    fn main_color(&self) -> RGB8 {
        self.start_color
    }

    fn set_main_color(&mut self, color: RGB8) {
        self.start_color = color;
    }
}

impl ConfigWithSecondaryColor for GradientConfig {
    fn secondary_color(&self) -> RGB8 {
        self.end_color
    }

    fn set_secondary_color(&mut self, color: RGB8) {
        self.end_color = color;
    }
}

fn gradient_step<const N: usize>(start: u8, end: u8, step: usize) -> u8 {
    let start_i16 = start as i16;
    let end_i16 = end as i16;
    let step_i16 = step as i16;
    let led_number = N as i16;

    (start_i16 + (step_i16 * (end_i16 - start_i16)) / (led_number - 1)) as u8
}
