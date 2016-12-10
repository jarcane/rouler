#![allow(dead_code)]

#[macro_use]
extern crate pest;

extern crate rand;

mod parse;

use pest::*;
use parse::*;

#[derive(Debug)]
pub struct Roller<'a> {
    roll: &'a str,
    total: i32,
}

impl<'a> Roller<'a> {
     fn new(roll: &str) -> Roller {
         let mut parser = Rdp::new(StringInput::new(roll));
         parser.expression();

         Roller{
             roll: roll,
             total: parser.compute()
         }
     }

     fn reroll(&mut self) {
         let mut parser = Rdp::new(StringInput::new(self.roll));
         parser.expression();

         self.total = parser.compute();
     }
}

#[test]
fn it_works() {
    let mut roller = Roller::new("4d6");
    let orig = roller.total;

    assert!((4 <= roller.total) && (roller.total <= 24));

    roller.reroll();

    assert!(orig != roller.total);
}