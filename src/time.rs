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

//! Utilities to deal with time in chasers.

use embedded_time::{duration::Generic, rate::Hertz};

/// Timing configuration.
#[derive(Debug)]
pub struct TimeConfig {
    pub refresh_rate: Hertz,
    pub transition_time: Generic<u32>,
}

impl TimeConfig {
    /// Builds a new timing configuration.
    pub fn new(
        refresh_rate: Hertz,
        transition_time: impl Into<Generic<u32>>,
    ) -> Self {
        Self {
            refresh_rate,
            transition_time: transition_time.into(),
        }
    }

    /// Returns the number of steps for a transition.
    pub fn transition_steps(&self) -> u32 {
        self.refresh_rate.0
            * self.transition_time.integer()
            * self.transition_time.scaling_factor().numerator()
            / self.transition_time.scaling_factor().denominator()
    }
}
