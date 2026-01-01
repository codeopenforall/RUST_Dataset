////////////////// Vulnerable Code //////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Holder {
    data: Vec<u8>,
}

impl Holder {
    fn new(size: usize) -> Self {
        Holder { data: vec![0u8; size] }
    }

    // Update a value in the internal array using unsafe pointer arithmetic.
    // This function does not perform any bounds checking, so an index equal to or
    // greater than the allocated size will access memory out-of-bound,
    // triggering undefined behavior (CWE-119).
    fn update(&mut self, index: usize, value: u8) {
        unsafe {
            // Vulnerable: no bounds checking
            let ptr = self.data.as_mut_ptr();
            // POTENTIAL OVERFLOW: pointer arithmetic used without verifying index < data.len()
            *ptr.add(index) = value;
        }
    }

    fn get(&self, index: usize) -> u8 {
        self.data[index]
    }
}

// A higher-level function that processes the data in a concurrent context.
fn operate(holder: Arc<Mutex<Holder>>, index: usize, value: u8) {
    let mut instance = holder.lock().unwrap();
    instance.update(index, value);
}

fn main() {
    // Create a shared Holder with exactly 10 bytes.
    let instance = Arc::new(Mutex::new(Holder::new(10)));

    // Spawn a thread that will attempt to update position 10 (out-of-bound)
    let instance_thread = Arc::clone(&instance);
    let handle = thread::spawn(move || {
        // This call writes outside the bounds (index == data.len())
        operate(instance_thread, 10, 42);
    });

    // Wait for the thread to complete.
    handle.join().unwrap();

    // Print a valid value to demonstrate the program reached here.
    println!("Stored value at index 9: {}", instance.lock().unwrap().get(9));
}