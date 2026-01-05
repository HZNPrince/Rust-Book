use std::fs::{self, File};
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
fn main(){

    let text = "Prince mehta ajsd;kabdwa  da sdanqlajwd";
    let texty = text.lines().next()?.chars().last()?;
}