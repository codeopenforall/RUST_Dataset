////////////////////////////////////////////////////////////////
// The corrected program removes the unsafe out-of-bounds write.
// It performs only safe vector operations so that only valid indices are used.
// The processor now computes the sum of the original numbers without corruption.
////////////////////////////////////////////////////////////////
use std::thread;

fn compute_value() -> i32 {
    let data = vec![1, 2, 3, 4, 5];                // Allocate a vector with safe initialization.
    // No unsafe block is used; the vector remains within its allocated bounds.
    data.iter().sum()
}

struct Worker;

impl Worker {
    fn process(&self) -> i32 {
        compute_value()
    }
}

fn main() {
    let handler = Worker;
    // Spawn a thread to simulate concurrency.
    let handle = thread::spawn(move || {
        handler.process()
    });

    let result = handle.join().unwrap();
    println!("Total: {}", result);
}