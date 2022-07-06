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

//! A collection of LED chasers on top of `smart_leds`.

mod rainbow_chaser;
mod unicolor_transition;

#[cfg(feature = "rand")]
mod random_unicolor;

pub use rainbow_chaser::RainbowChaser;
#[cfg(feature = "rand")]
pub use random_unicolor::RandomUnicolor;
pub use unicolor_transition::UnicolorTransition;

use crate::time::TimeConfig;

/// A LED chaser.
pub trait Chaser<const N: usize>: Iterator {
    fn set_time_config(&mut self, time_config: &TimeConfig);
}
