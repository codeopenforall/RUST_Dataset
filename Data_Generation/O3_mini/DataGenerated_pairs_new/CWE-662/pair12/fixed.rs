#!/usr/bin/env rust
// Fixed Example: Consistent Lock Ordering to Prevent Deadlock (CWE-662)
// This Rust program uses two shared resources protected by Mutexes.
// Both threads acquire the locks in the same order ensuring no deadlock occurs.
// Unsafe blocks remain only for low-level pointer manipulations.
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

    // Thread 1: locks resource A then resource B.
    let a1 = Arc::clone(&res_a);
    let b1 = Arc::clone(&res_b);
    let h1 = thread::spawn(move || {
        let lock_a = a1.lock().unwrap();
        // Small delay remains but lock ordering is maintained.
        thread::sleep(Duration::from_millis(50));
        let lock_b = b1.lock().unwrap();
        unsafe {
            let ptr = &*lock_a as *const Data as *mut Data;
            (*ptr).value += 1;
        }
    });

    // Thread 2: now also locks resource A then resource B.
    // Consistent lock ordering prevents deadlock.
    let a2 = Arc::clone(&res_a);
    let b2 = Arc::clone(&res_b);
    let h2 = thread::spawn(move || {
        let lock_a = a2.lock().unwrap();
        thread::sleep(Duration::from_millis(50));
        let lock_b = b2.lock().unwrap();
        unsafe {
            let ptr = &*lock_b as *const Data as *mut Data;
            (*ptr).value -= 1;
        }
    });

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