//
// A corrected version of the program that eliminates the use-after-free vulnerability
// by using safe memory management with Arc instead of raw pointers.
// This prevents memory from being deallocated prematurely.
//
use std::sync::Arc;

pub trait Worker {
    fn evaluate(&self) -> u32;
}

pub struct Processor {
    data: Arc<u32>,
}

impl Processor {
    // Constructs the structure using Arc so that the data remains valid across threads.
    pub fn new(val: u32) -> Self {
        Self {
            data: Arc::new(val),
        }
    }

    // A safe function that returns the value.
    pub fn compute(&self) -> u32 {
        *self.data
    }
}

// The execute function now safely computes the result by using the Arc-managed value.
pub fn execute() -> u32 {
    let job = Processor::new(42);
    job.compute()
}

fn main() {
    // Safe execution: Arc ensures that the value remains valid for all users.
    let res = execute();
    println!("Result is: {}", res);
}