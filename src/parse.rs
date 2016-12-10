use pest::*;
use rand::{thread_rng, Rng};

impl_rdp! {
    grammar! {
        expression = _{
            { ["("] ~ expression ~ [")"] | number }
            addition       = { plus  | minus }
            multiplication = { times | slash }
            die_roll       = { roll }
        }
        number = @{ (["0"] | ['1'..'9'] ~ ['0'..'9']*) }
        plus   =  { ["+"] }
        minus  =  { ["-"] }
        times  =  { ["*"] }
        slash  =  { ["/"] }
        roll   =  { ["d"] }

        whitespace = _{ [" "] }
    }

    process! {
        compute(&self) -> i32 {
            (&number: number) => number.parse::<i32>().unwrap(),
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
                        let mut rng = thread_rng();

                        (0..left).map(|_| rng.gen_range(1, right + 1)).fold(0, |acc, x| acc + x)    
                    },
                    _ => unreachable!()
                }
            }
        }
    }
}