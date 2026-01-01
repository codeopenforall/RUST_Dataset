////////////////////////////////////////////////////////////////
// Vulnerable version (CWE-020: Improper Input Validation)
// This program reads an index from the command‐line and uses it to
// access an element from a container using unsafe pointer arithmetic.
// It does not validate that the index is within bounds, so user‐supplied
// values outside of the valid range (0..length-1) may lead to undefined behavior.
////////////////////////////////////////////////////////////////

use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

trait DataAccess {
    fn read(&self, idx: usize) -> i32;
}

struct Container {
    data: Vec<i32>,
}

impl Container {
    fn new() -> Self {
        // Construct a container with 10 sequential integers.
        Container { data: (1..=10).collect() }
    }
}

impl DataAccess for Container {
    fn read(&self, idx: usize) -> i32 {
        unsafe {
            // FLAW: No validation of idx against data.len()
            // If idx is out of bounds, this leads to undefined behavior.
            *self.data.as_ptr().add(idx)
        }
    }
}

fn main() {
    // Expect the index as the first command-line argument.
    let args: Vec<String> = env::args().collect();
    let idx: usize = args
        .get(1)
        .unwrap_or(&"0".to_string())
        .parse()
        .expect("Invalid number provided");
    let container = Arc::new(Container::new());
    let result = Arc::new(Mutex::new(0));

    let cont_clone = Arc::clone(&container);
    let res_clone = Arc::clone(&result);

    let handle = thread::spawn(move || {
        // Unsafe read without proper bounds validation.
        let val = cont_clone.read(idx);
        let mut res = res_clone.lock().unwrap();
        *res = val;
    });

    handle.join().expect("Thread panicked");
    let final_value = *result.lock().unwrap();
    println!("Value: {}", final_value);
}