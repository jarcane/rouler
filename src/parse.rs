// roller - A container-based system for generating die rolls
// Copyright (C) 2016 by John Berry
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use pest::*;
use random::*;

impl_rdp! {
    grammar! {
        expression = _{
            { ["("] ~ expression ~ [")"] | number }
            addition       = { plus  | minus }
            multiplication = { times | slash }
            die_roll       = { roll }
        }
        number = @{ ["-"]? ~ (["0"] | ['1'..'9'] ~ ['0'..'9']*) }
        plus   =  { ["+"] }
        minus  =  { ["-"] }
        times  =  { ["*"] }
        slash  =  { ["/"] }
        roll   =  { ["d"] }

        whitespace = _{ [" "] }
    }

    process! {
        compute(&self) -> i64 {
            (&number: number) => number.parse::<i64>().unwrap(),
            (_: addition, left: compute(), sign, right: compute()) => {
                match sign.rule {
                    Rule::plus  => left + right,
                    Rule::minus => left - right,
                    _ => unreachable!()
                }
            },
            (_: multiplication, left: compute(), sign, right: compute()) => {
                match sign.rule {
                    Rule::times => left * right,
                    Rule::slash => left / right,
                    _ => unreachable!()
                }
            },
            (_: die_roll, left: compute(), sign, right: compute()) => {
                match sign.rule {
                    Rule::roll => {
                        if right < 1 {
                            panic!("Sides must be greater than zero");
                        } else {
                            match left.signum() {
                                0  => panic!("Number of sides must not be zero"),
                                -1 => -roll_dice_raw(left.abs(), right as u64),
                                1  => roll_dice_raw(left, right as u64),
                                _  => unreachable!()
                            }
                        }
                    },
                    _ => unreachable!()
                }
            }
        }
    }
}