// he Box<T> type is a smart pointer because it implements the Deref trait,
// which allows Box<T> values to be treated like references.
// When a Box<T> value goes out of scope, the heap data that the box is pointing to is
// cleaned up as well because of the Drop trait implementation.
// These two traits will be even more important to the functionality provided by the other smart pointer
// types we’ll discuss in the rest of this chapter. Let’s explore these two traits in more detail.

use std::rc::Rc;

use boxes::{
    self,
    List::{Cons, Nil},
};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let c = boxes::CustomSmartPointer {
        data: String::from("Hello"),
    };
    let d = boxes::CustomSmartPointer {
        data: String::from("Data"),
    };
    drop(c);

    println!("Smart pointers created !");

    let a = Rc::new(5);
    let b = Rc::clone(&a);
    let a = Rc::clone(&b);
}
