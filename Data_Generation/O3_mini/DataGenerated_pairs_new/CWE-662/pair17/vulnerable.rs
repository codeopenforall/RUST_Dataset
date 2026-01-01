use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Shared {
    lock1: Mutex<u32>,
    lock2: Mutex<u32>,
}

fn main() {
    let shared = Arc::new(Shared {
        lock1: Mutex::new(0),
        lock2: Mutex::new(0),
    });

    let s1 = shared.clone();
    let t1 = thread::spawn(move || {
        // Acquire locks in one order
        let _g1 = s1.lock1.lock().unwrap();
        thread::sleep(Duration::from_millis(100));
        // Using an unsafe block to bypass proper synchronization
        let raw: *mut u32;
        {
            // Acquire the second lock here (lock2)
            let mut g2 = s1.lock2.lock().unwrap();
            raw = &mut *g2 as *mut u32;
        }
        // Dangerously update the value via a raw pointer.
        unsafe {
            *raw += 1;
        }
    });

    let s2 = shared.clone();
    let t2 = thread::spawn(move || {
        // Acquire locks in the reverse order, causing potential deadlock.
        let _g2 = s2.lock2.lock().unwrap();
        thread::sleep(Duration::from_millis(100));
        let raw: *mut u32;
        {
            // Acquire the other mutex (lock1) afterwards.
            let mut g1 = s2.lock1.lock().unwrap();
            raw = &mut *g1 as *mut u32;
        }
        unsafe {
            *raw += 1;
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
    let v1 = *shared.lock1.lock().unwrap();
    let v2 = *shared.lock2.lock().unwrap();
    println!("Results: {} and {}", v1, v2);
}