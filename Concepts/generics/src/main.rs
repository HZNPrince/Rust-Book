struct Point<T, U>{
    x: T,
    y: U
}
impl <T, U> Point<T, U> {
    fn x(&self) -> &T{
        &self.x
    }
    fn y(&self) -> &U{
        &self.y
    }
}

impl<U> Point<f32, U>{
    fn distance_from_origin(&self)-> f32{
        let x = self.x.powi(3) + self.x;
        x
    }
}

enum Result<T, E>{
    Ok(T),
    Err(E)
}

fn main() {
    let coords1 = Point{
        x: 4.42,
        y: String::from("Prince")
    };
    println!("The value of x is :{}", Point::distance_from_origin(&coords1));
    println!("The value of y is :{}", coords1.y());
    
}
