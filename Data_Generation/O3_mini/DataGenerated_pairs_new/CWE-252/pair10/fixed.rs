/////////////////////////////////////////////////////////////
// Fixed Example: Proper Handling of the Return Value (CWE-252)
// The corrected version properly validates the input size before performing
// the unsafe memory operation and propagates any errors encountered.
/////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::io::{Result, Error, ErrorKind};
use std::thread;

struct DataProcessor {
    data: Arc<Mutex<Vec<u8>>>,
}

impl DataProcessor {
    // Unsafe function with additional safety checks.
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

    // In the corrected implementation, process() performs a pre-check on the input length
    // and propagates the error from add_data properly.
    fn process(&self, input: &[u8]) -> Result<()> {
        {
            let vec_guard = self.data.lock().unwrap();
            if input.len() > vec_guard.len() {
                return Err(Error::new(ErrorKind::Other, "Buffer overflow"));
            }
        }
        unsafe { self.add_data(input) }
    }
}

fn main() {
    let data = Arc::new(Mutex::new(vec![0u8; 10])); // Buffer size is 10 bytes.
    let processor = DataProcessor { data: data.clone() };
    let clone_processor = DataProcessor { data: data.clone() };

    // Spawn a thread that intentionally uses an input larger than the buffer.
    let thread_handle = thread::spawn(move || {
        let res = clone_processor.process(&[1,2,3,4,5,6,7,8,9,10,11]); // 11 bytes.
        // This assertion confirms that the error is correctly propagated.
        assert!(res.is_err(), "Thread: Invalid input did not produce an error as expected.");
    });

    // Main thread uses valid input.
    let res_valid = processor.process(&[100,101,102]);
    assert!(res_valid.is_ok());
    thread_handle.join().unwrap();
    println!("Processing complete safely.");
}