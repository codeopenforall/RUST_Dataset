///////////////////////////
// Vulnerable Code Start //
///////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Shared {
    a: i32,
    b: i32,
}

fn run() -> i32 {
    let shared = Arc::new(Mutex::new(Shared { a: 0, b: 0 }));
    let lock1 = Arc::new(Mutex::new(()));
    let lock2 = Arc::new(Mutex::new(()));

    let s1 = Arc::clone(&shared);
    let l1 = Arc::clone(&lock1);
    let l2 = Arc::clone(&lock2);
    let th1 = thread::spawn(move || {
        // Acquiring locks in order: lock1 then lock2
        let _guard1 = l1.lock().unwrap();
        thread::sleep(Duration::from_millis(50));
        let _guard2 = l2.lock().unwrap();
        let mut data = s1.lock().unwrap();
        data.a += 1;
        data.b += 1;
    });

    let s2 = Arc::clone(&shared);
    let l1_2 = Arc::clone(&lock1);
    let l2_2 = Arc::clone(&lock2);
    let th2 = thread::spawn(move || {
        // Vulnerability: Inconsistent lock ordering (lock2 then lock1)
        let _guard1 = l2_2.lock().unwrap(); // Vulnerable locking operation (line 28)
        thread::sleep(Duration::from_millis(50));
        let _guard2 = l1_2.lock().unwrap(); // Vulnerable locking operation (line 30)
        let mut data = s2.lock().unwrap();
        data.a += 1;
        data.b += 1;
    });

    th1.join().unwrap();
    th2.join().unwrap();

    let result = shared.lock().unwrap();
    result.a + result.b
}

fn main() {
    let res = run();
    println!("Result: {}", res);
}
/////////////////////////
// Vulnerable Code End //
/////////////////////////