use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};

fn main() {
    let v = vec![100, 2213, 31112, 4122, 22215];

    let th = thread::spawn(move || {
        for i in v.iter() {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    th.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(100));
    }

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    let (tx, rx) = mpsc::channel();
    let tx_c = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("T1: Hi"),
            String::from("T1: from"),
            String::from("T1: the"),
            String::from("T1: thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::sleep(Duration::from_millis(500));

    thread::spawn(move || {
        let vals = vec![
            String::from("T2: Hi"),
            String::from("T2: from"),
            String::from("T2: the"),
            String::from("T2: thread"),
        ];

        for val in vals {
            tx_c.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_t = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter_t.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}