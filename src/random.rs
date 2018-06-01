// rouler - A container-based system for generating die rolls
// Copyright (C) 2016 by John Berry
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use rand::{thread_rng, Rng};

pub fn roll_dice_raw(num: i64, sides: u64) -> i64 {
    let mut rng = thread_rng();

    (0..num.abs()).map(|_| rng.gen_range(1, sides as i64 + 1)).fold(0, |acc, x| acc + x)
}

pub fn roll_custom_dice_raw(num: i64, sides: &[u64]) -> i64 {
    let mut rng = thread_rng();

    (0..num.abs()).map(|_| rng.choose(sides).unwrap()).fold(0, |acc, x| acc + *x as i64)
}
