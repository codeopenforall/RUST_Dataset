///////////////////////////////
// Vulnerable Code Snippet
///////////////////////////////
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

struct Data {
    value: UnsafeCell<u32>,
}

// UNSAFE: Manually asserting Sync on a type that uses UnsafeCell without proper synchronization.
unsafe impl Sync for Data {}

impl Data {
    // This method unsafely increments the inner counter.
    fn increment(&self) {
        unsafe {
            let ptr = self.value.get();
            let curr = *ptr;
            // Simulate a delay to widen the race window.
            thread::yield_now();
            *ptr = curr + 1;
        }
    }

    fn get(&self) -> u32 {
        unsafe { *self.value.get() }
    }
}

// This function spawns several threads and unsafely increments the shared counter.
fn run() -> u32 {
    let shared = Arc::new(Data { value: UnsafeCell::new(0) });
    let mut threads = Vec::new();

    for _i in 0..10 {
        let copy = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                copy.increment();
            }
        });
        threads.push(handle);
    }

    for thr in threads {
        thr.join().unwrap();
    }
    shared.get()
}

fn main() {
    let final_value = run();
    println!("Final count: {}", final_value);
}