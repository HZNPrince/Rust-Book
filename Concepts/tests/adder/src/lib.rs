pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 0 {
            panic!("Guess value must be greater than or equal to 1, got {value}.")
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.")
        }
        Guess { value }
    }
}

pub fn add_two(num: i32) -> i32 {
    num + 2
}

pub fn greetings(name: &str) -> String {
    format!("Hello {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 7,
        };
        let smaller = Rectangle {
            width: 6,
            height: 6,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 14,
            height: 10,
        };
        let smaller = Rectangle {
            width: 7,
            height: 3,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn adding_two() {
        let num = 15;
        assert_eq!(add_two(num), 17);
    }

    #[test]
    fn greetings_contain_name() {
        let result = greetings("Prince");
        assert!(
            result.contains("Prince"),
            "Greetings did not contain name, value was {result}"
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn guess_value_range_test() {
        Guess::new(200);
    }
}
