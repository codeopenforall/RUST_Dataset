///////////////////////////////////////////////////////////////////////////////
// In this corrected version, the race condition is fixed by using proper locking.
// The shared counter is protected by a Mutex and shared across threads via Arc,
// preventing concurrent, unsynchronized updates.
///////////////////////////////////////////////////////////////////////////////

use std::sync::{Mutex, Arc};
use std::thread;

struct Counter {
    value: Mutex<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter { value: Mutex::new(0) }
    }

    fn get(&self) -> i32 {
        *self.value.lock().unwrap()
    }

    fn add(&self, delta: i32) {
        let mut guard = self.value.lock().unwrap();
        *guard += delta;
    }
}

fn run() -> i32 {
    let counter = Arc::new(Counter::new());
    let mut threads = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                c.add(1);
            }
        });
        threads.push(handle);
    }

    for t in threads {
        t.join().unwrap();
    }
    
    counter.get()
}

fn main() {
    let result = run();
    println!("Final count is: {}", result);
}