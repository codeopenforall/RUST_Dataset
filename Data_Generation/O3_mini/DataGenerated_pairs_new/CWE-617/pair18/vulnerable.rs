/* 
This code simulates a complex unsafe update in a concurrent context.
It uses an unsafe block with unchecked pointer arithmetic and a reachable assertion, 
allowing attacker-controlled values to trigger a panic (DoS) when the invariant is violated.
*/
use std::sync::{Arc, Mutex};
use std::thread;

struct Container {
    buffer: Vec<u32>,
}

impl Container {
    fn new() -> Self {
        // Initialize buffer with 10 elements set to 0.
        Container { buffer: vec![0; 10] }
    }

    // This method unsafely updates an element at a given index with the provided value.
    fn update(&mut self, index: usize, value: u32) {
        unsafe {
            // FLAW: Using unchecked pointer arithmetic with attacker influenced index.
            let ptr = self.buffer.as_mut_ptr();
            *ptr.add(index) = value;
        }
        // FLAW: Reachable assertion on an attacker influenced state.
        // If an attacker supplies a value >= 1000, this assertion panics, leading to DoS.
        assert!(self.buffer[index] < 1000, "Invariant violated: value must be less than 1000");
    }
}

fn process(input: u32) {
    let shared = Arc::new(Mutex::new(Container::new()));
    let mut threads = vec![];

    // Spawn three threads. One thread uses attacker-provided input.
    for i in 0..3 {
        let shared_clone = Arc::clone(&shared);
        let inp = input;
        let handle = thread::spawn(move || {
            let mut guard = shared_clone.lock().unwrap();
            if i == 1 {
                // Attacker-controlled update.
                guard.update(2, inp);
            } else {
                guard.update(i, 42);
            }
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }
}

fn main() {
    // In a real-world scenario, the attacker might influence this value.
    // Here we deliberately use a value that violates the invariant.
    process(1500);
}