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

        // Order of precedence: "+-" is less than "*/" is less than "dD"
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

// Force recompile when parse changes
const _GRAMMAR : &'static str = include_str!("rouler.pest");

pub fn compute(expr: Pairs<Rule>) -> i64 {
    PREC_CLIMBER.climb(
        expr,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::number => pair.as_str().parse::<i64>().unwrap().into(),
            Rule::expr => compute(pair.into_inner()),
            Rule::custom_dice => {
                let mut inner = pair.into_inner();
                // LHS
                let num = inner.next().unwrap();
                let lhs = num.as_str().parse::<i64>().expect("Did not find a number on LHS!");
                // Operator
                let d = inner.next().unwrap().as_str();
                assert!(d == "d" || d == "D");
                // RHS
                let mut sides = vec![];
                while let Some(s) = inner.next() {
                    // Collect numbers
                    if s.as_rule() == Rule::number {
                        sides.push(s.as_str().parse::<u64>().expect("Non-number found on RHS!"));
                    }
                }
                lhs.signum() * roll_custom_dice_raw(lhs.abs(), &sides)
            },
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
