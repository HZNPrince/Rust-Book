pub struct CustomSmartPointer {
    pub data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping custom smart pointer with data : {}", self.data);
    }
}
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}
