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
        ])
    };
}

#[derive(Parser)]
#[grammar = "rouler.pest"]
pub struct RollParser;

// Force recompile when parse changes
const _GRAMMAR : &str = include_str!("rouler.pest");

pub fn compute(expr: Pairs<Rule>) -> i64 {
    PREC_CLIMBER.climb(
        expr,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::number => pair.as_str().parse::<i64>().unwrap(),
            Rule::expr => compute(pair.into_inner()),
            Rule::dice => {
                let mut inner = pair.into_inner();
                // LHS
                let num = inner.next().unwrap();
                let lhs = num.as_str().parse::<i64>().expect("Did not find a number on LHS!");
                assert!(lhs != 0, "Number of dice must not be zero!");
                // Operator
                let d = inner.next().unwrap().as_str();
                assert!(d == "d" || d == "D");
                // RHS
                let num = inner.next().unwrap();
                let rhs = num.as_str().parse::<i64>().expect("Did not find a number on RHS!");
                assert!(rhs > 0, "Number of sides must be greater than zero!");
                // Postfix
                let op = inner.next();
                match op.map(|r| r.as_rule()) {
                    Some(Rule::best) => {
                        let num = inner.next().unwrap();
                        let take = num.as_str().parse::<i64>().expect("Did not find a number on postfix!");
                        lhs.signum() * roll_and_take_dice_raw(lhs.abs(), rhs as u64, take)
                    }
                    Some(Rule::worst) => {
                        let num = inner.next().unwrap();
                        let take = num.as_str().parse::<i64>().expect("Did not find a number on postfix!");
                        lhs.signum() * roll_and_take_dice_raw(lhs.abs(), rhs as u64, -take)
                    }
                    Some(Rule::adv) => {
                        lhs.signum() * roll_and_take_dice_raw(lhs.abs() * 2, rhs as u64, lhs.abs())
                    }
                    Some(Rule::dis) => {
                        lhs.signum() * roll_and_take_dice_raw(lhs.abs() * 2, rhs as u64, -lhs.abs())
                    }
                    None => lhs.signum() * roll_dice_raw(lhs.abs(), rhs as u64),
                    Some(_) => unreachable!(),
                }
            }
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
                for s in inner {
                    // Collect numbers
                    if s.as_rule() == Rule::number {
                        sides.push(s.as_str().parse::<u64>().expect("Non-number found on RHS!"));
                    }
                };
                lhs.signum() * roll_custom_dice_raw(lhs.abs(), &sides)
            },
            _ => unreachable!(),
        },
        |lhs: i64, op: Pair<Rule>, rhs: i64| match op.as_rule() {
            Rule::plus => lhs + rhs,
            Rule::minus => lhs - rhs,
            Rule::times => lhs * rhs,
            Rule::slash => lhs / rhs,
            _ => unreachable!(),
        }
    )
}
