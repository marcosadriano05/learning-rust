enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

// Box smart pointer enables store values on heap memory.
// It permits data structures that compiler don't knows its size can be allocated on heap and referenced to a pointer on stack memory.
fn main() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
