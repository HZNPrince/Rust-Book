// Stacks heaps pointers

struct Id<'a> {
    name: &'a str,
    roll_no: i32,
}
fn main() {
    let Yuno = Id {
        name: "Yuno",
        roll_no: 46,
    };
    let Rahul = Id {
        name: Yuno.name,
        roll_no: 47,
    };

    println!("{}", Yuno.name);
}
