#!/usr/bin/env rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
struct Data {
    value: i32,
}
fn run() -> (i32, i32) {
    let res_a = Arc::new(Mutex::new(Data { value: 0 }));
    let res_b = Arc::new(Mutex::new(Data { value: 100 }));
    let a1 = Arc::clone(&res_a);
    let b1 = Arc::clone(&res_b);
    let h1 = thread::spawn(move || {
        let lock_a = a1.lock().unwrap();
        thread::sleep(Duration::from_millis(50));
        let lock_b = b1.lock().unwrap();
        unsafe {
            let ptr = &*lock_a as *const Data as *mut Data;
            (*ptr).value += 1;
        }
    });
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
