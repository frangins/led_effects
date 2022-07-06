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

use super::Sequence;

/// A sequences that repeats the same sequence twice in symmetry.
pub struct Symmetry<S: Sequence<N>, const N: usize> {
    /// The underlying sequencee.
    sequence: S,
    /// The symmetry.
    symmetry: [Option<S::Item>; N],
    /// The counter.
    counter: usize,
    /// Whether the output is origin or symmetry.
    is_origin: bool,
}

impl<S: Sequence<N, Item = impl Copy>, const N: usize> Sequence<N>
    for Symmetry<S, N>
{
    type Config = S::Config;

    fn new(config: Self::Config) -> Self {
        Self {
            sequence: S::new(config),
            symmetry: [None; N],
            counter: 0,
            is_origin: true,
        }
    }

    fn config(&self) -> Self::Config {
        self.sequence.config()
    }
}

impl<S: Sequence<N, Item = impl Copy>, const N: usize> Iterator
    for Symmetry<S, N>
{
    type Item = S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_origin {
            if self.counter < N / 2 + N % 2 {
                let next = self.sequence.next();
                self.sequence.next();
                self.symmetry[self.counter] = next;
                self.counter += 1;
                next
            } else {
                self.is_origin = false;
                self.counter -= 1 + N % 2;
                self.symmetry[self.counter]
            }
        } else if self.counter > 0 {
            self.counter -= 1;
            self.symmetry[self.counter]
        } else {
            None
        }
    }
}
