use std::sync::{Arc, Mutex};
use std::thread;

const ITER: i32 = 10000;
const NUM_THREADS: i32 = 4;

struct SharedData {
    value: Mutex<i32>,
}

impl SharedData {
    fn new() -> Self {
        SharedData {
            value: Mutex::new(0),
        }
    }

    // Synchronized update using a Mutex.
    fn update(&self) {
        let mut guard = self.value.lock().unwrap();
        *guard += 1;
    }

    fn read(&self) -> i32 {
        *self.value.lock().unwrap()
    }
}

fn main() {
    let data = Arc::new(SharedData::new());
    let mut threads = Vec::new();

    // Spawn multiple threads updating the counter with proper synchronization.
    for _ in 0..NUM_THREADS {
        let shared = Arc::clone(&data);
        let handle = thread::spawn(move || {
            for _ in 0..ITER {
                shared.update();
            }
        });
        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap();
    }

    let result = data.read();
    println!("Final counter: {}", result);
    // The Mutex guarantees the expected final value.
    if result != NUM_THREADS * ITER {
        panic!("Synchronization failure: expected {} but found {}", NUM_THREADS * ITER, result);
    }
}