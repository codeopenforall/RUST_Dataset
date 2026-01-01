/////////////////////// Vulnerable Code ///////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

struct Processor {
    src: Arc<Vec<u8>>,
    dst: Arc<Mutex<Vec<u8>>>,
    expected_len: usize,
}

impl Processor {
    fn new(src: Vec<u8>, dst: Vec<u8>) -> Self {
        let expected_len = src.len();
        Processor {
            src: Arc::new(src),
            dst: Arc::new(Mutex::new(dst)),
            expected_len,
        }
    }

    // Unsafe routine that simulates a low-level memory copy.
    // Returns 0 on success and -1 on error.
    unsafe fn simulated_copy(&self, src_ptr: *const u8, dst_ptr: *mut u8, len: usize) -> i32 {
        if len != self.expected_len {
            // Simulate a failure if the requested length doesn't match expectation.
            return -1;
        }
        ptr::copy_nonoverlapping(src_ptr, dst_ptr, len);
        0
    }

    // Performs the data transfer operation.
    // Vulnerability: the return value from simulated_copy is misinterpreted.
    // A non-zero return (indicating error) is falsely treated as success.
    fn run(&self) -> i32 {
        let src_ptr = self.src.as_ptr();
        let mut dst_lock = self.dst.lock().unwrap();
        let dst_ptr = dst_lock.as_mut_ptr();
        let len = self.src.len();
        let status: i32;
        unsafe {
            status = self.simulated_copy(src_ptr, dst_ptr, len);
        }
        // Vulnerability: Incorrect check of the function return value.
        // The function returns 0 on success, but here non-zero is indicated as success.
        if status != 0 { // <-- Vulnerability: wrong comparison operator.
            0
        } else {
            -1
        }
    }
}

fn main() {
    // The input is chosen such that the length matches (i.e. success should occur)
    let src = vec![1u8, 2, 3, 4];
    let dst = vec![0u8; 4];
    let processor = Processor::new(src, dst);

    // Spawn a thread to execute the transfer concurrently.
    let proc_clone = processor;
    let handle = thread::spawn(move || {
        // In a correct implementation, a successful transfer returns 0.
        // Due to the vulnerability, a successful copy is misinterpreted.
        let ret = proc_clone.run();
        std::process::exit(ret);
    });

    handle.join().unwrap();
}