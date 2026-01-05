use rand::{self, Rng};

pub fn randomizer(x: i32) -> f64 {
    let num: f64 = rand::rng().random();
    println!("The number passed is {}", x);
    num
}
