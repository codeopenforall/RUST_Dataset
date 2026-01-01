--------------------------------------------------
use std::sync::{Arc, Mutex};
use std::thread;

struct Buffer {
    data: [u8; 1024],
}

impl Buffer {
    // This unsafe function attempts to copy the provided bytes into the buffer.
    // It returns an error if the input is too large.
    unsafe fn copy_into(&mut self, src: &[u8]) -> Result<(), &'static str> {
        if src.len() > self.data.len() {
            return Err("source too long");
        }
        std::ptr::copy_nonoverlapping(src.as_ptr(), self.data.as_mut_ptr(), src.len());
        Ok(())
    }
}

// Function that spawns a thread to perform the unsafe copy.
// The error returned by the unsafe copy is ignored.
fn execute_task(input: &[u8]) -> Result<(), &'static str> {
    let shared = Arc::new(Mutex::new(Buffer { data: [0; 1024] }));
    let shared_clone = Arc::clone(&shared);
    // Spawn a thread that performs the unsafe operation.
    let handle = thread::spawn(move || {
        let mut guard = shared_clone.lock().unwrap();
        // Vulnerability: the return value of the unsafe operation is discarded.
        unsafe {
            let _ = guard.copy_into(input);
        }
    });
    // Wait for the thread to finish.
    let _ = handle.join();
    // The error from copy_into is never propagated.
    Ok(())
}

// Wrapper function intended to be used in tests.
pub fn run_processing(input: &[u8]) -> Result<(), &'static str> {
    execute_task(input)
}

fn main() {
    // Triggering input: oversized data will cause copy_into to return an error,
    // but the error is unchecked.
    let input = vec![1u8; 2048];
    // The error is ignored, so main continues normally.
    let _ = run_processing(&input);
    println!("Operation completed (vulnerable).");
}
--------------------------------------------------