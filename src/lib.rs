// roller - A container-based system for generating die rolls
// Copyright (C) 2016 by John Berry
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![warn(missing_docs)]
//! # roller - A container-like system for generating dice rolls
//! 
//! roller is a library for handling repeatable dice rolls in a conveniently storable way.
//! At the heart of roller is the `Roller`, a simple struct that contains both the syntax for a specific
//! dice roll, and the result of the most recent roll. This allows you to store a particular roll
//! type in a variable, that can then be passed around (within the limits of mutable values), and 
//! repeated via method calls as needed. It is meant to be useful for CRPGs and tabletop game aids.
//! 
//! ## Example usage
//!
//! ```
//! use roller::Roller;
//!
//! let mut stat = Roller::new("3d6");
//!
//! println!("STR: {}", stat.total());
//! println!("DEX: {}", stat.reroll());
//! ``` 

#[macro_use]
extern crate pest;
extern crate rand;

mod parse;
mod random;

use std::fmt;
use std::cmp::Ordering;

use pest::*;
use parse::*;

pub fn roll_dice(r: &str) -> i64 {
    let mut parser = Rdp::new(StringInput::new(r));
    parser.expression();

    parser.compute()
}

#[derive(Debug, Clone, Copy)]
pub struct Roller<'a> {
    roll: &'a str,
    total: i64,
}

impl<'a> Roller<'a> {
     pub fn new(roll: &str) -> Roller {
         Roller{
             roll: roll,
             total: roll_dice(roll)
         }
     }

     pub fn reroll(&mut self) -> i64 {
         self.total = roll_dice(self.roll);

         self.total
     }

     pub fn total(&self) -> i64 {
         self.total
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