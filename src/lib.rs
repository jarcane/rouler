#![allow(dead_code)]

#[macro_use]
extern crate pest;

extern crate rand;

mod parse;

use pest::*;
use parse::*;

pub fn roll_dice(r: &str) -> i32 {
    let mut parser = Rdp::new(StringInput::new(r));
    parser.expression();

    parser.compute()
}

#[derive(Debug)]
pub struct Roller<'a> {
    roll: &'a str,
    total: i32,
}

impl<'a> Roller<'a> {
     fn new(roll: &str) -> Roller {
         Roller{
             roll: roll,
             total: roll_dice(roll)
         }
     }

     fn reroll(&mut self) -> i32 {
         self.total = roll_dice(self.roll);

         self.total
     }

     fn total(&self) -> i32 {
         self.total
     }
}