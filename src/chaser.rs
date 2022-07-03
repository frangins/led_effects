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
pub use unicolor_transition::UnicolorTransition;

#[cfg(feature = "rand")]
pub use random_unicolor::RandomUnicolor;

use crate::{
    sequence::{OneParameterSequenceEnum, Rainbow, Unicolor},
    time::TimeConfig,
};

#[cfg(feature = "rand")]
use rand::distributions::Uniform;

use smart_leds::hsv::Hsv;

/// A LED chaser.
pub trait Chaser<const N: usize>: Iterator {
    fn set_time_config(&mut self, time_config: &TimeConfig);
}

/// A LED chaser with one parameter.
pub trait OneParameterChaser<Color, const N: usize>: Chaser<N> {
    fn new(start_color: Color, time_config: &TimeConfig) -> Self;
}

/// A LED chaser with two parameters.
pub trait TwoParameterChaser<Color, const N: usize>: Chaser<N> {
    fn new(color1: Color, color2: Color, time_config: &TimeConfig) -> Self;
}

/// Container enum for one-parameter chasers.
pub enum ChaserEnum<const N: usize> {
    RainbowUnicolor(RainbowChaser<Unicolor<Hsv, N>, N>),
    #[cfg(feature = "rand")]
    RandomUnicolor(RandomUnicolor<Uniform<u32>, N>),
    DoubleRainbow(RainbowChaser<Rainbow<N>, N>),
}

impl<const N: usize> From<RainbowChaser<Unicolor<Hsv, N>, N>>
    for ChaserEnum<N>
{
    fn from(chaser: RainbowChaser<Unicolor<Hsv, N>, N>) -> Self {
        ChaserEnum::RainbowUnicolor(chaser)
    }
}

impl<const N: usize> From<RainbowChaser<Rainbow<N>, N>> for ChaserEnum<N> {
    fn from(chaser: RainbowChaser<Rainbow<N>, N>) -> Self {
        ChaserEnum::DoubleRainbow(chaser)
    }
}

impl<const N: usize> Chaser<N> for ChaserEnum<N> {
    fn set_time_config(&mut self, time_config: &TimeConfig) {
        match self {
            ChaserEnum::RainbowUnicolor(chaser) => {
                chaser.set_time_config(time_config)
            }
            #[cfg(feature = "rand")]
            ChaserEnum::RandomUnicolor(chaser) => {
                chaser.set_time_config(time_config)
            }
            ChaserEnum::DoubleRainbow(chaser) => {
                chaser.set_time_config(time_config)
            }
        }
    }
}

impl<const N: usize> Iterator for ChaserEnum<N> {
    type Item = OneParameterSequenceEnum<N>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ChaserEnum::RainbowUnicolor(chaser) => {
                chaser.next().map(Into::into)
            }
            #[cfg(feature = "rand")]
            ChaserEnum::RandomUnicolor(chaser) => chaser.next().map(Into::into),
            ChaserEnum::DoubleRainbow(chaser) => chaser.next().map(Into::into),
        }
    }
}
