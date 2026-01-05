struct ImportantExcerpt<'a>{
    part: &'a str,
}

fn largest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x > y {x} else{y}
}

fn main() {
    let name = String::from("Prince");
    let surname = String::from("Mehta");
    let result;
    {
        result = largest(&name.as_str(), surname.as_str());
    }
    println!("The largest is {result}");

    let novel = String::from("Hey this is Prince , how are you ... hope your doing fine ?");
    let last_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt{
        part: last_sentence,
    };

}
