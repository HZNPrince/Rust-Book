use std::io::Result;

fn main() {
    type Kilometers = i32;

    let num = 5;
    let num2: Kilometers = 7;

    println!("{}", num + num2);

    println!("{}", is_equal("Hello", "World"))
}
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, bug: &[u8]) -> Result<()>;
}

fn is_equal<T: Eq>(t1: &T, t2: &T) -> bool {
    t1 == t2
}
