enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

// Rc (Reference counted smart pointer) ables that one value take more than one ownership.
// Theres an count inside the pointer that verify how many data have ownership from a value.
// This value only be deallocated if all data that have its ownership are droped.
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a: {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b: {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c: {}", Rc::strong_count(&a));
    }
    println!("count after drop c: {}", Rc::strong_count(&a));
}
