/////////////////////// Fixed Code ///////////////////////
use std::thread;
use std::time::Duration;

struct Data {
    value: i32,
}

impl Data {
    fn new(val: i32) -> Self {
        Data { value: val }
    }
}

fn compute() -> i32 {
    // Safely allocate and use Data without exposing raw pointers.
    let boxed = Box::new(Data::new(42));
    // Access the value directly while ownership is maintained by the Box.
    boxed.value
}

fn main() {
    // Spawn a thread to simulate concurrent execution.
    let handle = thread::spawn(|| {
        // Sleep briefly to mimic concurrency effects.
        thread::sleep(Duration::from_millis(10));
        compute()
    });
    let result = handle.join().expect("Thread panicked");
    println!("Computed result: {}", result);
}