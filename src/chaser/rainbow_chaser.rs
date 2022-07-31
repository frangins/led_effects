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

use smart_leds::hsv::Hsv;

use super::Chaser;
use crate::{
    sequence::{ConfigWithMainColor, Sequence},
    time::TimeConfig,
};

/// A chaser that loops on the wheel of hues.
pub struct RainbowChaser<S: Sequence<N>, const N: usize> {
    /// The sequence configuration.
    sequence_config: S::Config,
    /// The start color.
    start_color: Hsv,
    /// The number of steps in a loop.
    step_number: u32,
    /// The current step.
    step: u32,
}

impl<S: Sequence<N, Config = impl ConfigWithMainColor>, const N: usize>
    Chaser<N> for RainbowChaser<S, N>
{
    fn set_time_config(&mut self, time_config: &TimeConfig) {
        let step_number = time_config.transition_steps();
        self.step = self.step * step_number / self.step_number;
        self.step_number = step_number;
    }
}

impl<S: Sequence<N, Config = impl ConfigWithMainColor>, const N: usize>
    RainbowChaser<S, N>
{
    pub fn new(sequence_config: S::Config, time_config: &TimeConfig) -> Self {
        Self {
            sequence_config,
            start_color: sequence_config.main_color().into(),
            step_number: time_config.transition_steps(),
            step: 0,
        }
    }
}

impl<S: Sequence<N, Config = impl ConfigWithMainColor>, const N: usize> Iterator
    for RainbowChaser<S, N>
{
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step == self.step_number {
            self.step = 0;
        }

        let color = Hsv {
            hue: self.start_color.hue
                + ((self.step * 255) / self.step_number) as u8,
            ..self.start_color
        };
        self.sequence_config.set_main_color(color.into());
        self.step += 1;

        Some(S::new(self.sequence_config))
    }
}
