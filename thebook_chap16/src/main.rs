use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::rc::Rc;

fn main() {
    // let handle = std::thread::spawn(move || {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread.", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // handle.join().unwrap();
    // for i in 1..5 {
    //     println!("hi number {} from the main thread.", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // let (tx, rx) = mpsc::channel();
    // let tx2 = mpsc::Sender::clone(&tx);
    
    // thread::spawn(move || {
    //     let val = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];

    //     for v in val {
    //         tx.send(v).unwrap();
    //         thread::sleep_ms(1000);
    //     }
    // });
    
    // thread::spawn(move || {
    //     let val = vec![
    //         String::from("more"),
    //         String::from("messages"),
    //         String::from("for"),
    //         String::from("you"),
    //     ];

    //     for v in val {
    //         tx2.send(v).unwrap();
    //         thread::sleep_ms(1000);
    //     }
    // });

    // for received in rx {
    //     println!("received: {}", received);
    // }

    // let m = Mutex::new(5);

    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }

    // println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(2));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
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
