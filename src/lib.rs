// rouler - A container-based system for generating die rolls
// Copyright (C) 2016 by John Berry
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![warn(missing_docs)]
//! # rouler - A container-like system for generating dice rolls
//!
//! [![Crates.io](https://img.shields.io/crates/v/rouler.svg)](https://crates.io/crates/rouler)
//! 
//! rouler is a library for handling repeatable dice rolls in a conveniently storable way.
//! At the heart of rouler is the `Roller`, a simple struct that contains both the syntax for a specific
//! dice roll, and the result of the most recent roll. This allows you to store a particular roll
//! type in a variable, that can then be passed around (within the limits of mutable values), and 
//! repeated via method calls as needed. It is meant to be useful for CRPGs and tabletop game aids.
//! 
//! ## Example usage
//!
//! ```
//! use rouler::Roller;
//!
//! let mut stat = Roller::new("3d6");
//!
//! println!("STR: {}", stat.total());
//! println!("DEX: {}", stat.reroll());
//! ``` 
//! 
//! ## The Die Roll Syntax
//! 
//! rouler uses parsed strings to define die rolls, according to the following [pest](https://github.com/dragostis/pest/) 
//! grammar found in `parse.rs`, with some additional rules checking: 
//! 
//! ```rust,ignore
//! expression = _{
//!     { ["("] ~ expression ~ [")"] | number }
//!     addition       = { plus  | minus }
//!     multiplication = { times | slash }
//!     die_roll       = { roll }
//! }
//! number = @{ ["-"]? ~ (["0"] | ['1'..'9'] ~ ['0'..'9']*) }
//! plus   =  { ["+"] }
//! minus  =  { ["-"] }
//! times  =  { ["*"] }
//! slash  =  { ["/"] }
//! roll   =  { ["d"] | ["D"] }
//! 
//! whitespace = _{ [" "] }
//! ```
//! 
//! Largely this should all be familiar basic mathematical notation, the key addition being the `d` operator,
//! which operates according to the standard notation familiar from tabletop RPGs, ie.:
//! 
//! `n[d|D]s`, where `n` = the number of dice to roll, and `s` = the number of sides on each die.
//! 
//! There are additional constraints checked for in this operator alone as well: neither `n` or `s` can be zero,
//! and `s` cannot be a negative number. `n` is allowed to be negative, but rather than rolling "negative dice",
//! this merely negates the value of the entire roll, such that `-3d6` would generate a value between -3 and -18.

#[macro_use]
extern crate pest;
extern crate rand;

mod parse;
mod random;
mod roll;

pub use roll::{Roller, roll_dice};