// rouler - A container-based system for generating die rolls
// Copyright (C) 2016 by John Berry
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use pest::{
    prec_climber::*,
    iterators::*,
};
use random::*;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use self::Assoc::*;
        use self::Rule::*;

        PrecClimber::new(vec![
            Operator::new(plus, Left) | Operator::new(minus, Left),
            Operator::new(times, Left) | Operator::new(slash, Left),
            Operator::new(roll, Right),
        ])
    };
}

#[derive(Parser)]
#[grammar = "rouler.pest"]
pub struct RollParser;

pub fn compute(expr: Pairs<Rule>) -> i64 {
    PREC_CLIMBER.climb(
        expr,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::number => pair.as_str().parse::<i64>().unwrap(),
            Rule::expr => compute(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: i64, op: Pair<Rule>, rhs: i64| match op.as_rule() {
            Rule::roll => {
                if rhs < 1 {
                    panic!("Sides must be greater than zero")
                } else {
                    match lhs.signum() {
                        0 => panic!("Number of sides must not be zero"),
                        -1 => -roll_dice_raw(lhs.abs(), rhs as u64),
                        1 => roll_dice_raw(lhs.abs(), rhs as u64),
                        _ => unreachable!(),
                    }
                }
            },
            Rule::plus => lhs + rhs,
            Rule::minus => lhs - rhs,
            Rule::times => lhs * rhs,
            Rule::slash => lhs / rhs,
            _ => unreachable!(),
        }
    )
}
