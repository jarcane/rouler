#![allow(dead_code)]

#[macro_use]
extern crate pest;

extern crate rand;

mod parse;

use pest::*;
use parse::*;

//#[derive(Debug)]
pub struct Roller<'a> {
    roll: parse::Rdp<pest::StringInput<'a>>,
    total: i32,
}

impl<'a> Roller<'a> {
     fn new(roll: &str) -> Roller {
         let mut parser = Rdp::new(StringInput::new(roll));
         parser.expression();

         let total = parser.compute();

         Roller{
             roll: parser,
             total: total
         }
     }

     fn reroll(&mut self) -> i32 {
         self.total = self.roll.compute();

         self.total
     }

     fn total(&self) -> i32 {
         self.total
     }
}

#[test]
fn it_works() {
    let mut roller = Roller::new("4d6");

    assert!((4 <= roller.total()) && (roller.total() <= 24));
    
    assert!(roller.total() != roller.reroll());
}