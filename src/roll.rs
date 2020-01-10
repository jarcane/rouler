// rouler - A container-based system for generating die rolls
// Copyright (C) 2016 by John Berry
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::cmp::Ordering;
use std::fmt;

use parse::*;
use pest::*;

/// A simple function for throwaway die rolls that do not need to be saved as a
/// `Roller`. Provided for convenience.
///
/// Takes an input of a `&str` containing syntax for a die roll, returns total.
///
/// # Panics
///
/// As `roll_dice` parses its argument, it will thus panic if the given syntax is incorrect.
///
/// # Examples
/// ```
/// use rouler::roll_dice;
///
/// println!("Wizard HP at lvl 9: {}", roll_dice("6d6+6"));
/// ```
///
pub fn roll_dice(r: &str) -> i64 {
    let parser = RollParser::parse(Rule::calc, r);
    compute(parser.expect("Failed to parse roll!"))
}

/// A function for throwaway die rolls that do not need to be saved as a
/// `Roller`. Provided for convenience.
///
/// Takes an input of a `&str` containing syntax for a die roll, returns Ok(total)
/// if the input parses successfully, otherwise a `ParsingError`.
///
/// # Examples
/// ```
/// use rouler::roll_dice_or_fail;
///
/// assert!(roll_dice_or_fail("6d6").is_ok());
/// assert!(roll_dice_or_fail("food4").is_err())
/// ```
pub fn roll_dice_or_fail(r: &str) -> Result<i64, Error<impl RuleType>> {
    let parser = RollParser::parse(Rule::calc, r);
    parser.map(|p| compute(p))
}

/// A function for safely creating a new `Roller` without panicking.
///
/// Takes a `&str` input and if the syntax parses, returns a Roller wrapped by a Result.
/// Otherwise returns a `ParsingError`.
///
/// # Examples
/// ```
/// use rouler::roller_or_fail;
///
///
/// ```
pub fn roller_or_fail<'a>(r: &'a str) -> Result<Roller<'a>, Error<impl RuleType>> {
    let parser = RollParser::parse(Rule::calc, r);
    parser.map(|p| {
        Roller {
            roll: r,
            total: compute(p),
        }
    })
}

/// The `Roller` is the core struct of the library. The basic principle is to provide a reusable
/// container that provides a specific kind of die roll, so that it can be quickly and easily repeated
/// whenever called for. Each container contains the syntax of the roll type it represents, and the
/// value of the last roll it made. Containers are thus self-mutating, but self-contained.
///
/// The main benefit is thus in code organization, as one can store different types of rolls for
/// easy use later.
#[derive(Debug, Clone, Copy)]
pub struct Roller<'a> {
    roll: &'a str,
    total: i64,
}

impl<'a> Roller<'a> {
    /// Creates a new `Roller` with the given die roll syntax, and populates the stored total with a first
    /// roll of the indicated dice. Because subsequent rerolls mutate the object in order to store the most
    /// recent roll, it should be declared with `let mut` if you intend to reroll it.
    ///
    /// # Panics
    ///
    /// As a `Roller` rolls itself on creation, it thus triggers the parser, and incorrect syntax will cause
    /// a panic.
    ///
    /// # Examples
    ///
    /// ```
    /// use rouler::Roller;
    ///
    /// let mut laser_damage = Roller::new("1d4*10");
    ///
    /// println!("Damage, rnd 1: {}", laser_damage.total());
    /// println!("Damage, rnd 2: {}", laser_damage.reroll());
    /// ```
    ///
    /// Rollers implement `Eq` and `Ord`, based on their current totals, so die results can be compared
    /// directly without having to first call the `total` method:
    ///
    /// ```
    /// # use rouler::Roller;
    /// let att = Roller::new("1d20 + 5");
    /// let def = Roller::new("1d20 + 2");
    ///
    /// if att > def {
    ///     println!("You struck the monster!");
    /// } else {
    ///     println!("You missed!");
    /// }
    /// ```
    ///
    /// For convenience's sake, Rollers also implement `Display`, so they are printable:
    ///
    /// ```
    /// # use rouler::Roller;
    /// println!("{}", Roller::new("4d8 + 5"));
    /// // => [4d8 + 5: 24]
    /// ```
    pub fn new(roll: &'a str) -> Self {
        Roller {
            roll: roll,
            total: roll_dice(roll),
        }
    }

    /// Rolls the `Roller`'s die roll, stores the value in total, and then returns it. In this way, you can use
    /// `Roller::reroll()` in place as a value, rather than needing to call the method seperately.
    ///
    /// # Examples
    /// ```
    /// use rouler::Roller;
    ///
    /// let mut stat = Roller::new("3d6");
    ///
    /// println!("STR (3d6): {}", stat.total());        // => STR (3d6): 14
    /// println!("DEX (3d6+1): {}", stat.reroll() + 1); // => DEX (3d6+1): 13
    ///
    /// println!("{}", stat);                           // prints [3d6: 12]
    /// ```
    pub fn reroll(&mut self) -> i64 {
        self.total = roll_dice(self.roll);

        self.total
    }

    /// Returns the result of the last roll made by the `Roller`.
    pub fn total(&self) -> i64 {
        self.total
    }

    /// Returns a reference to self for use as an `Iterator`. This allows for iterating infinitely and lazily over
    /// successive rolls of the dice. By borrowing as mutable, the state of the internal total is preserved,
    /// so that calls to `total()` will remain consistent.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rouler::Roller;
    /// // Collect multiple results to a vector:
    /// let stats = Roller::new("3d6").iter().take(6).collect::<Vec<i64>>();
    /// ```
    ///
    /// *Remember!* Rollers are infinite iterators: *always* use `take()` to avoid infinite loops!
    /// This is safe:
    ///
    /// ```
    /// # use rouler::Roller;
    /// // Keep rolling until a result greater than a threshold:
    /// assert!(Roller::new("4d6").iter().skip_while(|&x| x < 13).take(1).last().unwrap() >= 13);
    /// ```
    ///
    /// But this will not terminate:
    ///
    /// ```rust,ignore
    /// assert!(Roller::new("4d6").iter().skip_while(|&x| x < 13).last().unwrap() >= 13);
    /// ```
    pub fn iter(&mut self) -> &mut Self {
        self.by_ref()
    }
}

impl<'a> Iterator for Roller<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        Some(self.reroll())
    }
}

impl<'a> PartialEq for Roller<'a> {
    fn eq(&self, other: &Roller) -> bool {
        self.total == other.total
    }
}

impl<'a> Eq for Roller<'a> {}

impl<'a> Ord for Roller<'a> {
    fn cmp(&self, other: &Roller) -> Ordering {
        self.total.cmp(&other.total)
    }
}

impl<'a> PartialOrd for Roller<'a> {
    fn partial_cmp(&self, other: &Roller) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> fmt::Display for Roller<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}: {}]", self.roll, self.total)
    }
}
