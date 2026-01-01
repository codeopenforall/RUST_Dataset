/////////////////////////////////////////////////////////////
// Vulnerability Example: Unchecked Return Value (CWE-252)
// This code simulates a situation where an unsafe function returns an
// error when input data exceeds the buffer size, but the caller ignores
// the error. In a multithreaded context, the unchecked error may lead to
// unexpected behavior or memory corruption.
/////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::io::{Result, Error, ErrorKind};
use std::thread;

struct DataProcessor {
    data: Arc<Mutex<Vec<u8>>>,
}

impl DataProcessor {
    // Unsafe function performing pointer arithmetic on a locked vector.
    // Returns an error if the input length is greater than the buffer.
    unsafe fn add_data(&self, input: &[u8]) -> Result<()> {
        let mut vec_guard = self.data.lock().unwrap();
        if input.len() > vec_guard.len() {
            return Err(Error::new(ErrorKind::Other, "Buffer overflow"));
        }
        let raw_ptr = vec_guard.as_mut_ptr();
        for (i, &value) in input.iter().enumerate() {
            *raw_ptr.add(i) = value;
        }
        Ok(())
    }

    // The process function calls the unsafe add_data function.
    // Vulnerability: the function ignores the Result returned by add_data.
    fn process(&self, input: &[u8]) -> Result<()> {
        unsafe {
            // The error from add_data is discarded, even if a buffer overflow occurred.
            let _ = self.add_data(input);
        }
        // Always returns Ok(()), masking any errors.
        Ok(())
    }
}

fn main() {
    let data = Arc::new(Mutex::new(vec![0u8; 10])); // Buffer of fixed size 10.
    let processor = DataProcessor { data: data.clone() };
    let clone_processor = DataProcessor { data: data.clone() };

    // Spawn a thread that supplies input exceeding the buffer size.
    let thread_handle = thread::spawn(move || {
        let res = clone_processor.process(&[1,2,3,4,5,6,7,8,9,10,11]); // 11 bytes.
        // The caller expects an error, but the error is silently dropped.
        assert!(res.is_ok(), "Thread: Error should have been propagated, but it was ignored.");
    });

    // Main thread uses valid input.
    let res_valid = processor.process(&[100,101,102]);
    assert!(res_valid.is_ok());
    thread_handle.join().unwrap();
    println!("Processing complete.");
}