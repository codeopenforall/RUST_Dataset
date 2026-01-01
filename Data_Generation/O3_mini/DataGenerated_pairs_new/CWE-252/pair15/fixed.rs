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
// This fixed version properly propagates any error returned by the unsafe function.
fn execute_task(input: &[u8]) -> Result<(), &'static str> {
    let shared = Arc::new(Mutex::new(Buffer { data: [0; 1024] }));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let mut guard = shared_clone.lock().unwrap();
        // Propagate the error from the unsafe operation.
        unsafe { guard.copy_into(input) }
    });
    // Wait for the thread to finish and handle any panics.
    let thread_result = handle.join().map_err(|_| "thread panicked")?;
    // Return the result from the unsafe operation.
    thread_result
}

// Wrapper function intended to be used in tests.
pub fn run_processing(input: &[u8]) -> Result<(), &'static str> {
    execute_task(input)
}

fn main() {
    // Triggering input: oversized data will cause copy_into to return an error.
    // This fixed version correctly detects and propagates the error.
    let input = vec![1u8; 2048];
    match run_processing(&input) {
        Ok(_) => println!("Operation completed successfully (fixed)."),
        Err(err) => {
            println!("Error encountered: {}", err);
            std::process::exit(1);
        }
    }
}
--------------------------------------------------