use rand::{thread_rng, Rng};

pub fn roll_dice_raw(num: i64, sides: u64) -> i64 {
    let mut rng = thread_rng();

    (0..num.abs()).map(|_| rng.gen_range(1, sides as i64 + 1)).fold(0, |acc, x| acc + x)
}