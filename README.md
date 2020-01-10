# rouler 
[![Crates.io](https://img.shields.io/crates/v/rouler.svg)](https://crates.io/crates/rouler) [![docs.rs](https://docs.rs/rouler/badge.svg)](https://docs.rs/rouler/) [![Build Status](https://travis-ci.org/jarcane/rouler.svg?branch=master)](https://travis-ci.org/jarcane/rouler)

A container-like system for generating dice rolls

## Usage

rouler is a Rust library for generating die rolls from convenient little state containers. A die roll can be created and stored and called repeatedly to generate new values.

This allows easy use and re-use of specific die rolls, and even comparison. 

```rust
extern crate rouler;

use rouler::Roller;

let mut stat = Roller::new("3d6");

println!("STR: {}", stat.total());
println!("DEX: {}", stat.reroll());

println!("Last stat roll: {}", stat);

let att = Roller::new("1d20 + 5");
let def = Roller::new("1d20 + 2");

if att > def {
    println!("You struck the monster!");        
} else {
    println!("You missed!");
}
```

## Current Wishlist

* Arbitrary die sequences, for custom dice: `Roller::new("4d[1, 3, 5, 7]")`
* (Maybe) FromStr implementation for Rollers: `"3d20 * 2".from_str().unwrap()"`

## License

This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at http://mozilla.org/MPL/2.0/.
