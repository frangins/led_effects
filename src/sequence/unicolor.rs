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

use super::{ConfigWithMainColor, Sequence};

/// A sequence in which all LEDs have the same color.
pub struct Unicolor<Color, const N: usize> {
    /// The configuration.
    config: UnicolorConfig<Color>,
    /// The counter.
    counter: usize,
}

/// The configuration for unicolor sequences.
#[derive(Debug, Clone, Copy)]
pub struct UnicolorConfig<Color> {
    /// The color for all LEDs.
    pub color: Color,
}

impl<Color: Copy + Into<RGB8>, const N: usize> Sequence<N>
    for Unicolor<Color, N>
{
    type Config = UnicolorConfig<Color>;

    fn new(config: Self::Config) -> Self {
        Self { config, counter: 0 }
    }

    fn config(&self) -> Self::Config {
        self.config
    }
}

impl<Color: Copy + Into<RGB8>, const N: usize> Iterator for Unicolor<Color, N> {
    type Item = RGB8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < N {
            self.counter += 1;
            Some(self.config.color.into())
        } else {
            None
        }
    }
}

impl<Color: Copy + Into<RGB8> + From<RGB8>> ConfigWithMainColor
    for UnicolorConfig<Color>
{
    fn main_color(&self) -> RGB8 {
        self.color.into()
    }

    fn set_main_color(&mut self, color: RGB8) {
        self.color = color.into();
    }
}
