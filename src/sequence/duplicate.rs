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

use super::{ConfigWithMainColor, Sequence};

/// A sequence that repeats the same sequence multiple times.
pub struct Duplicate<S: Sequence<M>, const N: usize, const M: usize> {
    /// The underlying sequence.
    sequence: S,
    /// The configuration.
    config: DuplicateConfig<S::Config>,
    /// The counter.
    counter: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct DuplicateConfig<Config> {
    pub config: Config,
    pub duplicates: usize,
}

impl<S: Sequence<M, Item = impl Copy>, const N: usize, const M: usize>
    Sequence<N> for Duplicate<S, N, M>
{
    type Config = DuplicateConfig<S::Config>;

    fn new(config: Self::Config) -> Self {
        assert!(N == config.duplicates * M);

        Self {
            sequence: S::new(config.config),
            config,
            counter: 0,
        }
    }

    fn config(&self) -> Self::Config {
        self.config
    }
}

impl<S: Sequence<M, Item = impl Copy>, const N: usize, const M: usize> Iterator
    for Duplicate<S, N, M>
{
    type Item = S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.sequence.next() {
            Some(color) => Some(color),
            None => {
                if self.counter < self.config.duplicates - 1 {
                    self.counter += 1;
                    self.sequence = S::new(self.config.config);
                    self.sequence.next()
                } else {
                    None
                }
            }
        }
    }
}

impl<Config: ConfigWithMainColor> ConfigWithMainColor
    for DuplicateConfig<Config>
{
    fn main_color(&self) -> smart_leds::RGB8 {
        self.config.main_color()
    }

    fn set_main_color(&mut self, color: smart_leds::RGB8) {
        self.config.set_main_color(color);
    }
}
