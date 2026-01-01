#!/usr/bin/env rust
// Vulnerable Example: Improper Lock Ordering Causing Deadlock (CWE-662)
// This Rust program uses two shared resources protected by Mutexes.
// Two threads acquire the locks in different orders leading to a potential deadlock.
// Unsafe blocks are used to modify the data without further synchronization checks.
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Data {
    value: i32,
}

fn run() -> (i32, i32) {
    // Two shared resources.
    let res_a = Arc::new(Mutex::new(Data { value: 0 }));
    let res_b = Arc::new(Mutex::new(Data { value: 100 }));

    // Thread 1: locks resource A, then resource B.
    let a1 = Arc::clone(&res_a);
    let b1 = Arc::clone(&res_b);
    let h1 = thread::spawn(move || {
        let lock_a = a1.lock().unwrap();
        // Intentional delay to increase concurrency issues.
        thread::sleep(Duration::from_millis(100));
        let lock_b = b1.lock().unwrap();
        // Unsafe block simulating low-level pointer operations.
        unsafe {
            let ptr = &*lock_a as *const Data as *mut Data;
            (*ptr).value += 1;
        }
        // Locks are released at the end of scope.
    });

    // Thread 2: locks resource B first then resource A (inconsistent order).
    let a2 = Arc::clone(&res_a);
    let b2 = Arc::clone(&res_b);
    let h2 = thread::spawn(move || {
        let lock_b = b2.lock().unwrap();
        // Intentional delay to increase chance of deadlock.
        thread::sleep(Duration::from_millis(100));
        let lock_a = a2.lock().unwrap();
        unsafe {
            let ptr = &*lock_b as *const Data as *mut Data;
            (*ptr).value -= 1;
        }
    });

    // Wait for both threads. In a real deadlock, join may block indefinitely.
    // For testing, we assume the presence of a synchronization issue.
    h1.join().unwrap();
    h2.join().unwrap();

    let final_a = res_a.lock().unwrap().value;
    let final_b = res_b.lock().unwrap().value;
    (final_a, final_b)
}

fn main() {
    let (a, b) = run();
    println!("Resource A: {}, Resource B: {}", a, b);
}