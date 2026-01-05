fn main() {
    let num = 10;
    println!("adding {} by 1 = {}", num, add_one::add_one(num));

    let number = add_two::randomizer(num);
    println!("{}", number);
}
