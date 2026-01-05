struct Id{
    name: String,
    roll_no: i32,
}
fn main(){
    let Prince = Id{
        name: String::from("Prince Rajendra Mehta"),
        roll_no: 46,
    };
    let Rahul = Id{
        name: Prince.name,
        roll_no: 47,
    };

    println!("{}",Prince.name);
}
