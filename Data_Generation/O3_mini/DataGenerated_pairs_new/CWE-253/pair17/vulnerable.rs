/* 
   This code simulates a real-world concurrency scenario where a worker object is created and updated 
   using unsafe pointer arithmetic. The function that creates the worker returns a Result, but the code 
   incorrectly unwraps it without proper error checking. In addition, when updating the worker, an error 
   return is misinterpreted: if an error occurs during an update, it is “handled” by calling unwrap 
   in the error branch instead of properly propagating or handling the failure. This misinterpretation 
   of the function return values leads to a vulnerability (CWE-253) where error conditions are treated 
   as successes, potentially causing panics or undefined behavior during concurrent execution.
*/

use std::sync::{Arc, Mutex};
use std::thread;

struct Worker {
    data: Box<[u32]>,
}

impl Worker {
    fn new(size: usize) -> Result<Self, &'static str> {
        if size == 0 {
            Err("Invalid size for Worker initialization")
        } else {
            Ok(Worker { data: vec![1; size].into_boxed_slice() })
        }
    }

    fn update(&mut self, idx: usize, val: u32) -> Result<(), &'static str> {
        if idx >= self.data.len() {
            Err("Index out of bounds")
        } else {
            unsafe {
                let ptr = self.data.as_mut_ptr().add(idx);
                *ptr = val;
            }
            Ok(())
        }
    }
}

fn process_input(input: i32) -> i32 {
    // For negative input, size becomes 0 triggering an error in Worker::new.
    let size = if input < 0 { 0 } else { 10 };
    // Vulnerability: Incorrect check of the function return value; unwrap used without proper handling.
    let worker = Arc::new(Mutex::new(Worker::new(size).unwrap()));

    let mut handles = Vec::new();
    for i in 0..5 {
        let worker_arc = Arc::clone(&worker);
        handles.push(thread::spawn(move || {
            let mut guard = worker_arc.lock().unwrap();
            let ret = guard.update((i * 2) as usize, (i * 10) as u32);
            // Vulnerability: Misinterpreting error as success.
            if ret.is_err() {
                // Incorrectly calling unwrap() even when an error has occurred.
                ret.unwrap();
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let guard = worker.lock().unwrap();
    // Sum the worker's data.
    guard.data.iter().sum::<u32>() as i32
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input: i32 = if args.len() > 1 {
        args[1].parse().unwrap_or(1)
    } else {
        1
    };
    let result = process_input(input);
    println!("Result: {}", result);
}