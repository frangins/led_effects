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

use super::Chaser;
use crate::{
    sequence::{ConfigWithOffset, Sequence},
    time::TimeConfig,
};

/// A simple transition between two colors.
pub struct OffsetProgression<S: Sequence<N>, const N: usize> {
    /// The sequence configuration.
    sequence_config: S::Config,
    /// Whether to increment (or dercrement).
    increment: bool,
    /// The number of steps to perform the transition.
    step_number: u32,
    /// The current step.
    step: u32,
}

impl<S: Sequence<N, Config = impl ConfigWithOffset>, const N: usize> Chaser<N>
    for OffsetProgression<S, N>
{
    // IDEA: Factorise with other implementations.
    fn set_time_config(&mut self, time_config: &TimeConfig) {
        let step_number = time_config.transition_steps();
        self.step = self.step * step_number / self.step_number;
        self.step_number = step_number;
    }
}

impl<S: Sequence<N, Config = impl ConfigWithOffset>, const N: usize>
    OffsetProgression<S, N>
{
    pub fn new(
        sequence_config: S::Config,
        increment: bool,
        time_config: &TimeConfig,
    ) -> Self {
        Self {
            sequence_config,
            increment,
            step_number: time_config.transition_steps(),
            step: 0,
        }
    }
}

impl<S: Sequence<N, Config = impl ConfigWithOffset>, const N: usize> Iterator
    for OffsetProgression<S, N>
{
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step < self.step_number {
            // TODO: offset by step or steps by offset depending on the ratio.

            if self.step_number <= self.sequence_config.offset_range() {
                let offset_by_step = (self.sequence_config.offset_range()
                    / self.step_number)
                    as i32;
                defmt::debug!("offset by step: {}", offset_by_step);

                // TODO: Avoid duplication with the else clause.
                if self.increment {
                    self.sequence_config.increment_offset(offset_by_step);
                } else {
                    self.sequence_config.increment_offset(-offset_by_step);
                }
            } else {
                let steps_by_offset =
                    self.step_number / self.sequence_config.offset_range();
                // - self.step_number % self.sequence_config.offset_range();
                defmt::debug!("steps by offset: {}", steps_by_offset);

                if self.step % steps_by_offset == 0 {
                    if self.increment {
                        self.sequence_config.increment_offset(1);
                    } else {
                        self.sequence_config.increment_offset(-1);
                    }
                }
            }

            self.step += 1;

            Some(S::new(self.sequence_config))
        } else {
            None
        }
    }
}
