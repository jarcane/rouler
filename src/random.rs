// rouler - A container-based system for generating die rolls
// Copyright (C) 2016 by John Berry
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use rand::{thread_rng, Rng};

pub trait Die : Iterator<Item=i64> {}
impl<T: Iterator<Item=i64>> Die for T {}

pub struct StdDie {
    sides: u64
}

impl Iterator for StdDie {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        Some(thread_rng().gen_range(1, self.sides as i64 + 1))
    }
}

impl From<u64> for StdDie {
    fn from(n: u64) -> Self {
        StdDie{sides:n}
    }
}

pub fn roll_dice_gen<T: Die>(num: i64, die: T) -> i64 {
    die.take(num as usize).sum()
}