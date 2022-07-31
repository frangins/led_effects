// led_effects - A collection of LED effects on top of smart-leds.
// Copyright (C) 2022 Jean-Philippe Cugnet <jean-philippe@cugnet.eu>
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

/// A sequence that repeats the same sequence twice in symmetry.
pub struct Symmetry<S: Sequence<M>, const N: usize, const M: usize> {
    /// The underlying sequence.
    sequence: S,
    /// The symmetry.
    symmetry: [Option<S::Item>; M],
    /// The counter.
    counter: usize,
    /// Whether the output is origin or symmetry.
    is_origin: bool,
}

impl<S: Sequence<M, Item = impl Copy>, const N: usize, const M: usize>
    Sequence<N> for Symmetry<S, N, M>
{
    type Config = S::Config;

    fn new(config: Self::Config) -> Self {
        // NOTE: As there is currently no way to perform such assert at compile
        // time, let’s do it in the constructor at run time.
        assert!(N == 2 * M || N == 2 * M - 1);

        Self {
            sequence: S::new(config),
            symmetry: [None; M],
            counter: 0,
            is_origin: true,
        }
    }

    fn config(&self) -> Self::Config {
        self.sequence.config()
    }
}

impl<S: Sequence<M, Item = impl Copy>, const N: usize, const M: usize> Iterator
    for Symmetry<S, N, M>
{
    type Item = S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_origin {
            if self.counter < N / 2 + N % 2 {
                let next = self.sequence.next();
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
