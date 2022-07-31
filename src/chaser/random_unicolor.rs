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

use embedded_time::{duration::Extensions, rate::Hertz};
use rand::{
    distributions::{Distribution, Uniform},
    rngs::SmallRng,
    Rng, SeedableRng,
};
use smart_leds::{hsv::Hsv, RGB8};

use super::{Chaser, UnicolorTransition};
use crate::{
    sequence::{Unicolor, UnicolorConfig},
    time::TimeConfig,
};

/// A LED chaser that performs random transitions.
pub struct RandomUnicolor<
    HD: Distribution<i16>,
    TD: Distribution<u32>,
    const N: usize,
> {
    /// The random number generator for color and transition speed selection.
    rng: SmallRng,
    /// The refresh rate.
    refresh_rate: Hertz,
    /// The hue distribution.
    hue_distr: HD,
    /// The transition speed distribution.
    transition_time_distr: TD,
    /// The ongoing transition.
    transition: UnicolorTransition<N>,
}

impl<const N: usize> Chaser<N>
    for RandomUnicolor<Uniform<i16>, Uniform<u32>, N>
{
    fn set_time_config(&mut self, time_config: &TimeConfig) {
        self.refresh_rate = time_config.refresh_rate;

        let median_time_ms = time_config.transition_time.integer()
            * 1000
            * time_config.transition_time.scaling_factor().numerator()
            / time_config.transition_time.scaling_factor().denominator();

        // IDEA: Make this parametrisable.
        let v = median_time_ms * 2 / 3;
        self.transition_time_distr =
            Uniform::new(median_time_ms - v, median_time_ms + v);

        // Update the ongoing transition.
        self.transition.set_time_config(time_config);
    }
}

impl<const N: usize> RandomUnicolor<Uniform<i16>, Uniform<u32>, N> {
    /// Builds a new random unicolor chaser.
    pub fn new(
        refresh_rate: Hertz,
        hue_distr: Uniform<i16>,
        transition_time_distr: Uniform<u32>,
    ) -> Self {
        let mut rng = SmallRng::seed_from_u64(0);

        let start_color = Hsv {
            hue: rng.gen(),
            sat: 255,
            val: 255,
        };

        let transition = generate_transition(
            &mut rng,
            refresh_rate,
            &hue_distr,
            &transition_time_distr,
            start_color,
        );

        Self {
            rng,
            refresh_rate,
            hue_distr,
            transition_time_distr,
            transition,
        }
    }

    /// Sets the color temperature.
    ///
    /// A negative values gives warmer hues, a positive one colder hues.
    pub fn set_temperature(&mut self, temperature: i8) {
        self.hue_distr = if temperature.is_negative() {
            Uniform::new(
                -128 + temperature.abs() as i16,
                127 - temperature.abs() as i16,
            )
        } else {
            Uniform::new(
                temperature.abs() as i16,
                255 - temperature.abs() as i16,
            )
        };
    }
}

impl<HD: Distribution<i16>, TD: Distribution<u32>, const N: usize> Iterator
    for RandomUnicolor<HD, TD, N>
{
    type Item = Unicolor<RGB8, N>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.transition.next() {
            Some(sequence) => Some(sequence),
            None => {
                self.transition = generate_transition(
                    &mut self.rng,
                    self.refresh_rate,
                    &self.hue_distr,
                    &self.transition_time_distr,
                    self.transition.end_color().into(),
                );

                self.transition.next()
            }
        }
    }
}

fn generate_transition<const N: usize>(
    rng: &mut impl Rng,
    refresh_rate: Hertz,
    hue_distr: &impl Distribution<i16>,
    transition_time_distr: &impl Distribution<u32>,
    start_color: Hsv,
) -> UnicolorTransition<N> {
    let end_color = Hsv {
        hue: rng.sample(hue_distr) as u8,
        sat: 255,
        val: 255,
    };

    let transition_time = rng.sample(transition_time_distr).milliseconds();
    let time_config = TimeConfig::new(refresh_rate, transition_time);

    UnicolorTransition::new(
        UnicolorConfig {
            color: start_color.into(),
        },
        start_color,
        end_color,
        &time_config,
    )
}
