use std::thread;
use std::time::Duration;

// To guarantee the threads will run "at same time", the sleep with 1 milisecond duration stop
// both threads to both execute. If the main thread finish, so the spawn thread is aborted. To
// prevent this, the handle variable with type JoinHandle is used to wait the thread wich its handle
// finished with the join method.
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Spawn thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // -------------------------
    // To pass data from one thread to another, is necessary to move the ownership.
    // To do this, the move keyword is used in the closure, if its is not make, the closure
    // will get the borrowind of the data, it's cause problems, so the compiler show the error.
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Vector: {:?}", v);
    });

    handle.join().unwrap();
}
