//////////////////////////////
// Fixed Rust Code
//////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Data {
    // Protect access to the counter with a Mutex.
    value: Mutex<u32>,
}

impl Data {
    fn new(val: u32) -> Self {
        Data {
            value: Mutex::new(val),
        }
    }

    // Increment the counter safely using Mutex locking.
    fn update(&self) {
        let mut lock = self.value.lock().unwrap();
        *lock += 1;
    }

    // Retrieve the current value in a thread-safe manner.
    fn fetch(&self) -> u32 {
        *self.value.lock().unwrap()
    }
}

fn main() {
    let shared = Arc::new(Data::new(0));
    let mut workers = Vec::new();

    // Spawn 10 threads; each performs 1000 synchronized increments.
    for _ in 0..10 {
        let handler = {
            let local = Arc::clone(&shared);
            thread::spawn(move || {
                for _ in 0..1000 {
                    local.update();
                }
            })
        };
        workers.push(handler);
    }

    for worker in workers {
        worker.join().expect("Thread panicked");
    }
    println!("Final value: {}", shared.fetch());
}