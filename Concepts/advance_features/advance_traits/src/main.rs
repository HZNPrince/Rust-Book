use std::ops::Add;

#[derive(PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Adding two different types
#[derive(Debug, Copy, Clone, PartialEq)]
struct Centimeters(u32);
#[derive(Debug, Copy, Clone, PartialEq)]
struct Millimeters(u32);

impl Add<Centimeters> for Millimeters {
    type Output = Millimeters;
    fn add(self, rhs: Centimeters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

fn main() {
    assert_eq!(
        Point { x: 5, y: 10 } + Point { x: 10, y: 5 },
        Point { x: 15, y: 15 }
    );

    let centi = Centimeters(5);
    let milli = Millimeters(10000);

    let add_units = milli + centi;

    println!("{:?}", add_units)
}
