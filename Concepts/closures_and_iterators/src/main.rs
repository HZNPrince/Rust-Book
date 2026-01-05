use std::thread;

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut red_color = 0;
        let mut blue_color = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => red_color += 1,
                ShirtColor::Blue => blue_color += 1,
            }
        }
        if red_color > blue_color {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!("User preffered: {:?}, got : {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!("User2 preffered: {:?}, got: {:?}", user_pref2, giveaway2);

    let example_closure = |x| x;
    let s = example_closure("Hello".to_string());

    let s = String::from("Hello");
    let f = |_| ();
    f(s);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(|| println!("From thread: {list:?}"))
        .join()
        .unwrap();

    // Iterable
    let mut v1 = vec![1, 2, 3, 4];
    let mut v2 = v1.iter();
    println!("{}", v1[1]);
    let v3 = v2.next().unwrap();

    fn add_one(x: i32) -> i32 {
        x + 1
    }
    fn add_two(x: i32) -> i32 {
        x + 2
    }

    let mut f1: fn(i32) -> i32 = add_one;
    f1 = add_two;

    let mut data = String::from("Hello");
    let closure = || println!("{}", data);

    closure();
    closure();

    data.push_str("World");

    fn execute_once<F: FnOnce()>(f: F) {
        f();
    }
}
