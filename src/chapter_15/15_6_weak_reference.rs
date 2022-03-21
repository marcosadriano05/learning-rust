// To prevent leak memory when use Rc and RefCell smart pointers where two data references each other,
// is necessary to use Weak reference. A weak reference is presente on Rc weak_count, even if a
// weak_count has its value greater than zero, its value can be droped, but not if strong_count is
// greater than zero.
// If two data references each other with Strong reference, this cause memory leak. In this case,
// the recomendation is that a data "A" strong references to data "B" and the data "B" weak references to "A".
// To weak refence a data with Rc smart pointer, is used the downgrade method, to strong reference
// is used the clone method.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

// Tree data struct
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // Reference to the parente is weak, because when the parent is droped, its children are drop too.
    children: RefCell<Vec<Rc<Node>>>, // Reference is strong
}

fn main() {
    // Node without parent and children
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    {
        // Node without parent, but one child
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        // Node leaf strong_count = 2 and weak_count = 0
        // Is 2 because its value has two variable ownerships: leaf and branch
        // Node branch strong_count = 1 and weak_count = 1
        // If branch is droped, leaf strong_count turn into 1 and parent attribute in None

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    // after branch is droped
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
