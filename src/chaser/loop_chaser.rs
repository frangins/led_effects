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
pub struct RandomUnicolor<C: Chaser<N>, const N: usize> {
    /// The chaser to loop.
    chaser: C,
    // TODO: Add pause beween loops.
}

impl<C: Chaser<N>, const N: usize> Chaser<N> for RandomUnicolor<C, N> {
    // IDEA: Factorise with other implementations.
    fn set_time_config(&mut self, time_config: &TimeConfig) {
        self.chaser.set_time_config(time_config);
    }
}

impl<C: Chaser<N>, const N: usize> RandomUnicolor<C, N> {
    /// Builds a new random unicolor chaser.
    pub fn new(chaser: C) -> Self {
        Self { chaser }
    }
}

impl<C: Chaser<N>, const N: usize> Iterator for RandomUnicolor<C, N> {
    type Item = C::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chaser.next() {
            Some(sequence) => Some(sequence),
            None => {
                // TODO: New chaser.
                self.chaser.next()
            }
        }
    }
}
