//////////////////////////////////////////////////////////////
// Corrected Code Example (CWE-193 Fixed)
// The code structure remains largely the same; however, the bug is fixed by
// setting the vector’s length to the full input length.
//////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct DataProcessor;

impl DataProcessor {
    pub fn process(&self, input: &[u8]) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::with_capacity(input.len());
        unsafe {
            std::ptr::copy_nonoverlapping(input.as_ptr(), result.as_mut_ptr(), input.len());
            // Correction: Set the length equal to the full length of the input.
            result.set_len(input.len());
        }
        result
    }
}

fn run(input: &[u8]) -> Vec<u8> {
    let processor = Arc::new(DataProcessor);
    let data = Arc::new(input.to_vec());
    let output = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for _ in 0..4 {
        let proc_clone = Arc::clone(&processor);
        let data_clone = Arc::clone(&data);
        let out_clone = Arc::clone(&output);
        let handle = thread::spawn(move || {
            let res = proc_clone.process(&data_clone);
            let mut guard = out_clone.lock().unwrap();
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
    // Now the output properly contains all 6 bytes.
}