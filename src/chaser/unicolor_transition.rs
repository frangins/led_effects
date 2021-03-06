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

use smart_leds::RGB8;

use super::Chaser;
use crate::{
    sequence::{ConfigWithMainColor, Sequence, Unicolor, UnicolorConfig},
    time::TimeConfig,
};

/// A simple transition between two colors.
pub struct UnicolorTransition<const N: usize> {
    /// The sequence configuration.
    sequence_config: UnicolorConfig<RGB8>,
    /// The start color of the transition.
    start_color: RGB8,
    /// The end color of the transition.
    end_color: RGB8,
    /// The number of steps to perform the transition.
    step_number: u32,
    /// The current step.
    step: u32,
}

impl<const N: usize> UnicolorTransition<N> {
    pub fn end_color(&self) -> RGB8 {
        self.end_color
    }
}

impl<const N: usize> Chaser<N> for UnicolorTransition<N> {
    // IDEA: Factorise with other implementations.
    fn set_time_config(&mut self, time_config: &TimeConfig) {
        let step_number = time_config.transition_steps();
        self.step = self.step * step_number / self.step_number;
        self.step_number = step_number;
    }
}

impl<const N: usize> UnicolorTransition<N> {
    pub fn new(
        sequence_config: UnicolorConfig<RGB8>,
        start_color: impl Into<RGB8>,
        end_color: impl Into<RGB8>,
        time_config: &TimeConfig,
    ) -> Self {
        Self {
            sequence_config,
            start_color: start_color.into(),
            end_color: end_color.into(),
            step_number: time_config.transition_steps(),
            step: 0,
        }
    }
}

impl<const N: usize> Iterator for UnicolorTransition<N> {
    type Item = Unicolor<RGB8, N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step < self.step_number {
            let color = RGB8 {
                r: transition_step(
                    self.start_color.r,
                    self.end_color.r,
                    self.step,
                    self.step_number,
                ),
                g: transition_step(
                    self.start_color.g,
                    self.end_color.g,
                    self.step,
                    self.step_number,
                ),
                b: transition_step(
                    self.start_color.b,
                    self.end_color.b,
                    self.step,
                    self.step_number,
                ),
            };

            self.sequence_config.set_main_color(color);
            self.step += 1;

            Some(Unicolor::new(self.sequence_config))
        } else {
            None
        }
    }
}

fn transition_step(start: u8, end: u8, step: u32, step_number: u32) -> u8 {
    let start_i32 = start as i32;
    let end_i32 = end as i32;
    let step_i32 = step as i32;
    let step_number_i32 = step_number as i32;

    (start_i32 + (step_i32 * (end_i32 - start_i32)) / step_number_i32) as u8
}
