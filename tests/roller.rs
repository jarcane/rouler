// rouler - A container-based system for generating die rolls
// Copyright (C) 2016 by John Berry
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate rouler;

use rouler::*;

macro_rules! assert_range {
    ($begin:expr => $val:expr => $end:expr) => {
        assert!(($begin <= $val) && ($val <= $end));
    };
}

#[test]
fn roll_dice_within_range() {
    for _ in 0..100 {
        assert_range!(4 => roll_dice("4d6") => 24);
    }
}

#[test]
fn roller_object_within_range() {
    let test_roll = Roller::new("2d6 + 4");

    assert_range!(6 => test_roll.total() => 16);
}

#[test]
fn reroll_changes_value() {
    let mut test_roll = Roller::new("100d100");

    assert_ne!(test_roll.total(), test_roll.reroll())
}

#[test]
fn roll_custom_dice_within_range() {
    for _ in 0..100 {
        assert_range!(10 => roll_dice("2d[5, 6, 7]") => 14)
    }
}

#[test]
fn custom_dice_spaces_optional() {
    for _ in 0..100 {
        assert_range!(10 => roll_dice("2d[5,6,7]") => 14)
    }
}

#[test]
fn negative_dice_negates_roll_value() {
    assert_range!(-18 => Roller::new("-3d6").total() => -3);
}

#[test]
#[should_panic(expected = "not be zero")]
fn num_of_dice_nonzero() {
    assert!(Roller::new("0d6").total() == 0);
}

#[test]
#[should_panic(expected = "greater than zero")]
fn non_zero_sides_disallowed() {
    assert!(Roller::new("3d-6").total() < 0);
}

#[test]
fn d_op_is_case_insensitive() {
    assert_range!(1 => Roller::new("1D6").total() => 6);
    assert_range!(1 => Roller::new("1d6").total() => 6);
}

#[test]
#[ignore]
#[should_panic(expected = "no pattern matched")]
fn spaces_not_allowed_in_die_codes() {
    assert_range!(1 => Roller::new("1 d 6").total() => 6)
}

#[test]
fn rollers_are_iterators() {
    let mut d20 = Roller::new("1d20");
    let mut roll5 = d20.iter().take(5).collect::<Vec<i64>>();

    assert_eq!(5, roll5.len());
    assert_eq!(d20.total(), roll5.pop().unwrap());
}
