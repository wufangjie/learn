use crate::dbgt;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn pass_message_passing() {
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![]; // seems useless

    let mut count = 0;
    for i in 0..3 {
        let tx2 = tx.clone();
        handles.push(thread::spawn(move || {
            for j in 0..3 {
                tx2.send(10u32.pow(i) + j).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        }));
    }

    drop(tx); // NOTE: this is important, all senders must be drop
    for msg in rx {
        dbg!(&msg);
        count += msg;
    }
    // rx.recv().unwrap(); // used to block and get a message
    assert_eq!(342, count);
}

#[test]
pub fn test_shared_state_single_thread() {
    let x = Mutex::new(5);
    {
        // NOTE: lock() will block current thread
        // unwrap() for another thread holding the lock panicked without drop,
        // MutexGuard's Drop trait will release the lock
        let mut y = x.lock().unwrap(); // mutable reference
        *y = 42;
        println!("{:?}", x); // Mutex { data: <locked>, poisoned: false, .. }
        dbgt!(&y);
    }
    println!("{:?}", x)
}

#[test]
fn test_shared_state() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    // let (tx, rx) = mpsc::channel();

    for _ in 0..10 {
        let temp = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut y = temp.lock().unwrap();
            *y += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    assert_eq!(10, *counter.lock().unwrap());
}

#[test]
#[ignore]
fn test_deadlock() {
    let a = Arc::new(Mutex::new(5));
    let b = Arc::new(Mutex::new(42));
    let mut handles = vec![];

    for i in 0..2usize {
        let pair = [Arc::clone(&a), Arc::clone(&b)];
        handles.push(thread::spawn(move || {
            let a = pair[i].lock().unwrap();
            println!("thread {}, Got a: {}", i + 1, a);
            thread::sleep(Duration::from_secs(3));
            let b = pair[1 - i].lock().unwrap();
            println!("thread {}, Got b: {}", i + 1, b);
        }));
    }

    for h in handles.into_iter() {
        h.join().unwrap();
    }
}
