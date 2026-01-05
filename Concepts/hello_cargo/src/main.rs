fn main() {
    let mut items = vec![String::from("one"), String::from("two"), String::from("three"), String::from("four")];

    let mut latter_half = items.split_off(2);

    let mut output = String::new();

    items[0].push_str("_A");
    items[1].push_str("_B");

    for mut item in latter_half {
        item.push_str("_C");
        output.push_str(&item);
    }

    for item in items.iter() {
        output.push_str(item);
    }

    println!("{:?}", items);
    println!("{}", output);
}