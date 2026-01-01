//////////////////////////////
// Vulnerable Code Example
//////////////////////////////
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

struct SharedData {
    count: UnsafeCell<i32>,
}

unsafe impl Sync for SharedData {} // Unsafely marking as Sync without proper synchronization

impl SharedData {
    fn new(val: i32) -> Self {
        SharedData {
            count: UnsafeCell::new(val),
        }
    }

    fn increment(&self) {
        // Improper synchronization: unsynchronized read-modify-write operation.
        unsafe {
            let old = *self.count.get();
            thread::yield_now(); // Increase chance of race conditions.
            *self.count.get() = old + 1;
        }
    }

    fn get(&self) -> i32 {
        unsafe { *self.count.get() }
    }
}

fn run_logic() -> i32 {
    let shared = Arc::new(SharedData::new(0));
    let mut handles = vec![];

    // Spawn multiple threads that concurrently modify the shared counter.
    for _ in 0..10 {
        let s = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                s.increment();
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    shared.get()
}

fn main() {
    let final_val = run_logic();
    println!("Final count: {}", final_val);
}