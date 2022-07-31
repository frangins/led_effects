// led_effects - A collection of LED effects on top of smart-leds.
// Copyright (C) 2021 Guillaume Cugnet <guillaume@cugnet.eu>
// Copyright (C) 2021-2022 Jean-Philippe Cugnet <jean-philippe@cugnet.eu>
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

use super::{ConfigWithMainColor, Sequence};

/// A sequence in which the LEDs draw a rainbow.
pub struct Rainbow<const N: usize> {
    /// The configuration.
    config: RainbowConfig,
    /// The counter.
    counter: usize,
}

/// The configuration for rainbow sequences.
#[derive(Clone, Copy)]
pub struct RainbowConfig {
    /// The first color of the rainbow.
    pub first_color: Hsv,
    /// The color range.
    pub range: u8,
}

impl<const N: usize> Sequence<N> for Rainbow<N> {
    type Config = RainbowConfig;

    fn new(config: Self::Config) -> Self {
        Self { config, counter: 0 }
    }

    fn config(&self) -> Self::Config {
        self.config
    }
}

impl<const N: usize> Iterator for Rainbow<N> {
    type Item = RGB8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < N {
            let color = Hsv {
                hue: self.config.first_color.hue
                    + (self.counter * (self.config.range as usize / N)) as u8,
                ..self.config.first_color
            };
            self.counter += 1;
            Some(color.into())
        } else {
            None
        }
    }
}

impl ConfigWithMainColor for RainbowConfig {
    fn main_color(&self) -> RGB8 {
        self.first_color.into()
    }

    fn set_main_color(&mut self, color: RGB8) {
        self.first_color = color.into();
    }
}
