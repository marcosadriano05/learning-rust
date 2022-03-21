use std::ops::Deref;

fn main() {
    let x = 5;
    let y = &x; // reference
    assert_eq!(5, x);
    assert_eq!(5, *y); // dereference

    // -----------------
    let x = 5;
    let y = Box::new(x); // copy the x value and alocates in Heap memory
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // -----------------
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        // implementing Deref trait to allow dereference
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
