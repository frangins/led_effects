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

pub use gradient::{Gradient, GradientConfig};
pub use rainbow::{Rainbow, RainbowConfig};
pub use unicolor::{Unicolor, UnicolorConfig};

use smart_leds::RGB8;

/// A LED sequence.
pub trait Sequence<const N: usize>: Iterator {
    type Config: Copy;

    /// Creates a new sequence with the given config.
    fn new(config: Self::Config) -> Self;

    /// Gets the configuration of the sequence.
    fn config(&self) -> Self::Config;
}

/// A sequence configuration with a main color.
pub trait ConfigWithMainColor: Copy {
    /// Gets the main color.
    fn main_color(&self) -> RGB8;

    /// Sets the main color.
    fn set_main_color(&mut self, color: RGB8);
}

/// A sequence configuration with a secondary color.
pub trait ConfigWithSecondaryColor: Copy {
    /// Gets the secondary color.
    fn secondary_color(&self) -> RGB8;

    /// Sets the secondary color.
    fn set_secondary_color(&mut self, color: RGB8);
}
