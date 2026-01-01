use std::thread;
use std::sync::Arc;

trait Operation {
    fn process(&self) -> i32;
}

struct Handler;

impl Operation for Handler {
    fn process(&self) -> i32 {
        // Allocate an integer on the heap using an Arc to maintain shared ownership.
        let shared = Arc::new(42);
        // Clone the Arc to pass ownership into the thread.
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            // The shared value is safely accessed within the spawned thread.
            *shared_clone
        });
        // Wait for the thread to finish and retrieve the value.
        handle.join().unwrap()
    }
}

pub fn compute() -> i32 {
    let proc = Handler;
    proc.process()
}

fn main() {
    let result = compute();
    println!("Result: {}", result);
}