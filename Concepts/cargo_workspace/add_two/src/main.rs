#[derive(Debug)]
struct Lifetimes<'a> {
    reference_var: &'a mut str,
}

fn main() {
    let a;
    {
        let x = 5;
        a = x
    }
    println!("a: {}", a);

    let lifetime;
    {
        let mut string = String::from("Hello");
        lifetime = Lifetimes {
            reference_var: &mut string,
        };

        println!("{:?}", lifetime)
    }
}
