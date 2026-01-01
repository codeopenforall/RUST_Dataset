///////////////////////////////////////////////////////////////
// Vulnerability Example: Improper Synchronization (CWE-662)
///////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct SharedResource {
    lock_a: Mutex<()>,
    lock_b: Mutex<()>,
    counter: Mutex<u32>,
}

impl SharedResource {
    pub fn new() -> Self {
        Self {
            lock_a: Mutex::new(()),
            lock_b: Mutex::new(()),
            counter: Mutex::new(0),
        }
    }
    pub fn op1(&self) {
        // Acquire locks in order: lock_a then lock_b.
        let guard_a = self.lock_a.lock().unwrap();
        // Unsafe block to mimic non-trivial usage.
        unsafe {
            let ptr: *const () = &*guard_a;
            let _dummy = ptr as usize;
        }
        // Artificial delay to increase deadlock window.
        thread::sleep(Duration::from_millis(50));
        let guard_b = self.lock_b.lock().unwrap();
        let mut count = self.counter.lock().unwrap();
        *count += 1;
        drop(guard_b);
        drop(guard_a);
    }
    pub fn op2(&self) {
        // VULNERABILITY: Inconsistent lock ordering (lock_b then lock_a)
        let guard_b = self.lock_b.lock().unwrap();
        thread::sleep(Duration::from_millis(50));
        let guard_a = self.lock_a.lock().unwrap(); // Deadlock possible here.
        let mut count = self.counter.lock().unwrap();
        *count += 1;
        drop(guard_a);
        drop(guard_b);
    }
}

fn main() {
    let shared = Arc::new(SharedResource::new());
    let shared1 = Arc::clone(&shared);
    let handle1 = thread::spawn(move || {
        for _ in 0..10 {
            shared1.op1();
        }
    });
    let shared2 = Arc::clone(&shared);
    let handle2 = thread::spawn(move || {
        for _ in 0..10 {
            shared2.op2();
        }
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    let count = shared.counter.lock().unwrap();
    println!("Counter: {}", *count);
}