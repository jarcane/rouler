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
mod roll;

pub use roll::{Roller, roll_dice};