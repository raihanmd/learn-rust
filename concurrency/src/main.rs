use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("Spawned thread: {}", i);
            thread::sleep(std::time::Duration::from_millis(150));
        }
    });

    for i in 1..=5 {
        println!("Hi number {} from main thread", i);
        thread::sleep(std::time::Duration::from_millis(200));
    }

    // Its wait till spawned thread finish
    handle.join().unwrap();

    channel();
}

#[test]
fn thread_with_fnmut() {
    let v = vec![1, 2, 3, 4, 5];

    let thread_handle = thread::spawn(move || {
        println!("v Value: {:?}", v);
    });

    thread_handle.join().unwrap();
}

fn channel() {
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone(); // Clone the transmitter to send messages from another thread

    thread::spawn(move || {
        let msg = vec!["(1) Hello", "(1) from", "(1) thread"];

        for v in msg {
            tx.send(v.to_string()).unwrap();
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        let msg = vec!["(2) Hello", "(2) from", "(2) thread"];

        for v in msg {
            tx2.send(v.to_string()).unwrap();
            thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    // let received = rx.try_recv().unwrap();

    // .recv() will block main thread until data received
    // let received = rx.recv().unwrap();

    for received in rx {
        println!("Received: {}", received);
    }
}

#[test]
fn mutex() {
    use std::sync::{Arc, Mutex};

    let m = Arc::new(Mutex::new(5));

    {
        let mut num = m.lock().unwrap();

        // when MutexGuard out of scope auto unlock

        *num += 1;
    }

    let mut handles = vec![];

    for _ in 1..=10 {
        let m = Arc::clone(&m);
        handles.push(thread::spawn({
            move || {
                let mut num = m.lock().unwrap();
                *num += 1;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final m = {:?}", *m.lock().unwrap());
}
