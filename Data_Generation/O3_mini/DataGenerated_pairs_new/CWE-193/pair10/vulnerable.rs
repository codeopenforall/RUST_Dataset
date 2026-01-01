//////////////////////////////////////////////////////////////
// Vulnerable Code Example (CWE-193: Off-by-One Error)
// This program uses unsafe memory copying in a multithreaded context.
// It spawns several threads to process a shared input slice by
// copying its bytes into a new vector. Due to a fencepost mistake,
// the length of the resulting vector is set one byte too short.
//////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct DataProcessor;

impl DataProcessor {
    pub fn process(&self, input: &[u8]) -> Vec<u8> {
        // Allocate a vector with enough capacity.
        let mut result: Vec<u8> = Vec::with_capacity(input.len());
        unsafe {
            // Copy entire slice into uninitialized vector memory.
            std::ptr::copy_nonoverlapping(input.as_ptr(), result.as_mut_ptr(), input.len());
            // Off-by-One: Incorrectly set the length to one less than input.len(),
            // causing the last byte to be omitted from the logical vector.
            result.set_len(input.len() - 1);
        }
        result
    }
}

fn run(input: &[u8]) -> Vec<u8> {
    let processor = Arc::new(DataProcessor);
    let data = Arc::new(input.to_vec());
    let output = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    // Spawn several threads to process the data concurrently.
    for _ in 0..4 {
        let proc_clone = Arc::clone(&processor);
        let data_clone = Arc::clone(&data);
        let out_clone = Arc::clone(&output);
        let handle = thread::spawn(move || {
            let res = proc_clone.process(&data_clone);
            let mut guard = out_clone.lock().unwrap();
            // Overwrite the shared result with the processed data.
            *guard = res;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    Arc::try_unwrap(output).unwrap().into_inner().unwrap()
}

fn main() {
    let input = b"abcdef"; // 6 bytes expected.
    let output = run(input);
    println!("Output length: {}", output.len());
    // The bug results in a length of 5 instead of 6.
}