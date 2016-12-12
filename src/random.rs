use rand::{thread_rng, Rng};

pub fn roll_dice_raw(num: i32, sides: u32) -> i32 {
    let mut rng = thread_rng();

    (0..num.abs()).map(|_| rng.gen_range(1, sides as i32 + 1)).fold(0, |acc, x| acc + x)
}