use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// One form to send values to diferent threads is creating a channel with the transmitter and receiver.
// The transmitter is passing to the thread with its ownership and send data to the thread where
// its correspondent receiver is.
fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vec = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("spawn"),
            String::from("thread"),
        ];

        for v in vec {
            tx.send(v).unwrap();
        }
    });

    for receiver in rx {
        println!("{}", receiver);
    }

    // -------------------------
    // Multiple transmitters can send data to its respective receiver.
    // The thread sleep is to show better the comunication between the spawn threads and main thread.
    println!("--------------");

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vec = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("spawn"),
            String::from("thread"),
        ];

        for v in vec {
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vec = vec![
            String::from("Another"),
            String::from("message"),
            String::from("from"),
            String::from("other"),
            String::from("thread"),
        ];

        for v in vec {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for receiver in rx {
        println!("{}", receiver);
    }
}
