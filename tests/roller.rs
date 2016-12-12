extern crate roller;

use roller::*;

macro_rules! assert_range {
    ( $begin:expr => $val:expr => $end:expr ) => (assert!(($begin <= $val) && ($val <= $end));)
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
    let mut test_roll = Roller::new("12d8");
    
    assert_ne!(test_roll.total(), test_roll.reroll())
}

#[test]
#[should_panic(expected = "no pattern matched")]
fn negative_dice_not_allowed() {
    assert_range!(-3 => Roller::new("-3d6").total() => -18);
}

#[test]
#[ignore]
#[should_panic(expected = "no pattern matched")]
fn spaces_not_allowed_in_die_codes() {
    assert_range!(1 => Roller::new("1 d 6").total() => 6)
}