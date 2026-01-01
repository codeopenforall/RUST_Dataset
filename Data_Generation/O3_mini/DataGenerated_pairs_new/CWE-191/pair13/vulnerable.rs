////////////// Vulnerable Code //////////////
use std::sync::{Arc, Mutex, Barrier};
use std::thread;

pub struct Container {
    pub value: u32,
}

impl Container {
    // This method subtracts an amount unsafely without checking for underflow.
    pub unsafe fn reduce(&mut self, amount: u32) {
        // Vulnerability: unchecked subtraction leads to integer underflow.
        self.value = self.value - amount;
    }
}

pub fn execute(amount: u32) -> u32 {
    // Start with a small value; subtracting a larger amount causes an underflow.
    let data = Arc::new(Mutex::new(Container { value: 10 }));
    let barrier = Arc::new(Barrier::new(2));

    let data_clone = Arc::clone(&data);
    let barrier_clone = Arc::clone(&barrier);
    let handle = thread::spawn(move || {
        barrier_clone.wait();
        let mut guard = data_clone.lock().unwrap();
        unsafe {
            // Unsafe subtraction without bounds check.
            guard.reduce(amount);
        }
    });

    barrier.wait();
    handle.join().unwrap();

    let guard = data.lock().unwrap();
    guard.value
}

fn main() {
    let result = execute(20);
    // In vulnerable code, subtracting 20 from 10 underflows.
    println!("Final value: {}", result);
}