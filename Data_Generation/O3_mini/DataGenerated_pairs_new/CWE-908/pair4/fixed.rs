use std::sync::{Arc, Barrier};
use std::thread;

#[derive(Debug)]
struct Data {
    value: i32,
}

impl Data {
    // Safe constructor that fully initializes the object.
    fn new() -> Data {
        Data { value: 100 }
    }
}

pub fn execute() -> i32 {
    let barrier = Arc::new(Barrier::new(2));
    // Properly initialize the shared resource before sharing it.
    let shared = Arc::new(Data::new());
    let barrier_clone = Arc::clone(&barrier);
    let shared_clone = Arc::clone(&shared);

    let handle = thread::spawn(move || {
        barrier_clone.wait();
        // Safely access the fully initialized data.
        let read_val = shared_clone.value;
        read_val
    });

    barrier.wait();
    // No late initialization occurs here.
    handle.join().unwrap()
}

fn main() {
    let res = execute();
    println!("Result: {}", res);
}