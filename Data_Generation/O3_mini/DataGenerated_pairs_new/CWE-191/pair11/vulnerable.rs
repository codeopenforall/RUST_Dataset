/*
This Rust program simulates a concurrent data access routine that contains an integer 
underflow vulnerability. It uses an unsafe block to compute an index by subtracting a constant 
from a user‐provided input without verifying that the subtraction is valid. When the input 
is less than the constant (5), the subtraction uses wrapping arithmetic producing a huge 
index that leads to an unsafe read from memory.
*/
use std::sync::Arc;
use std::thread;

struct Data {
    arr: Vec<u8>,
}

impl Data {
    fn new() -> Self {
        // Fixed-size array for demonstration.
        Data { arr: vec![10, 20, 30, 40, 50] }
    }
    
    // This method computes an index by subtracting 5 from the input.
    // If input is less than 5, wrapping_sub creates a huge index.
    fn get_at(&self, pos: usize) -> Result<u8, &'static str> {
        // Vulnerability: Using wrapping subtraction may cause an underflow.
        let index = pos.wrapping_sub(5);         // <--- Vulnerable arithmetic (line 16)
        // Unsafe memory access using an unchecked index.
        let value = unsafe { *self.arr.get_unchecked(index) };    // <--- Unsafe dereference (line 17)
        Ok(value)
    }
}

struct Engine {
    data: Arc<Data>,
}

impl Engine {
    fn new(data: Arc<Data>) -> Self {
        Engine { data }
    }

    fn execute(&self, input: usize) -> Result<u8, &'static str> {
        self.data.get_at(input)
    }
}

fn main() {
    let data = Arc::new(Data::new());
    let engine = Engine::new(data.clone());

    // Spawn multiple threads to simulate concurrent access.
    let handles: Vec<_> = (0..3).map(|i| {
        let eng = Engine { data: data.clone() };
        thread::spawn(move || {
            // Using an input less than 5 to trigger the underflow vulnerability.
            // For i==0,1,2, (all less than 5) the computed index wraps around.
            let res = eng.execute(i);
            match res {
                Ok(val) => println!("Thread {} result: {}", i, val),
                Err(e) => println!("Thread {} error: {}", i, e),
            }
        })
    }).collect();

    for handle in handles {
        let _ = handle.join();
    }
}