use std::sync::{Arc, Mutex};
use std::thread;

// Mutex smart pointer provide a way to a thread access data, the thread must first signal the
// Mutex with the lock method, so the thread get exclusive access to the data and other threads
// don't. If the MutexGuard, the result of lock method, is out of scope, the MutexGuard is droped,
// and other threads can get access the data via lock too. The Mutex allow the data change its
// value in runtime, same as RefCell, even the data is immutable.
fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num += 1;
    }

    println!("Mutex m is {:?}", m);

    // ----------------------
    // To be able to manipulate same data in different threads, the Arc smart pointer allows create
    // more than one ownership, similar to Rc smart pointer, but Arc is thread safe.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Cloning counter to give ownership to each thread, the ownership counting is managed by the Arc.
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
