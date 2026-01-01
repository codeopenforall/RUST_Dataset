/////////////////////// Fixed Code ///////////////////////
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
            return -1;
        }
        ptr::copy_nonoverlapping(src_ptr, dst_ptr, len);
        0
    }

    // Performs the data transfer operation.
    // Correction: the return value is now properly interpreted.
    fn run(&self) -> i32 {
        let src_ptr = self.src.as_ptr();
        let mut dst_lock = self.dst.lock().unwrap();
        let dst_ptr = dst_lock.as_mut_ptr();
        let len = self.src.len();
        let status: i32;
        unsafe {
            status = self.simulated_copy(src_ptr, dst_ptr, len);
        }
        // Correct check: 0 indicates success, nonzero indicates failure.
        if status == 0 {
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
        let ret = proc_clone.run();
        std::process::exit(ret);
    });

    handle.join().unwrap();
}