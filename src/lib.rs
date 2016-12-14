#[macro_use]
extern crate pest;
extern crate rand;

mod parse;
mod random;

use std::cmp::Ordering;

use pest::*;
use parse::*;

pub fn roll_dice(r: &str) -> i64 {
    let mut parser = Rdp::new(StringInput::new(r));
    parser.expression();

    parser.compute()
}

#[derive(Debug)]
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