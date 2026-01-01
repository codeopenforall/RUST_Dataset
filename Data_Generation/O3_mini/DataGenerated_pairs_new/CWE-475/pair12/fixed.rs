////////////////////////////////////////
// Corrected Example in Rust         //
// Fixing CWE-475 by enforcing proper  //
// bounds on unsafe memory operations. //
////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Processor;

impl Processor {
    // Executes a safe memory copy by limiting the copy length to the destination's capacity.
    // If the user-specified length exceeds the allowed size, it is clamped to the maximum valid value.
    fn run(&self, req: usize) -> bool {
        let allowed = 5;
        let source = vec![1u8; 10]; // Source buffer.
        // Destination buffer: 5 bytes for data and 1 extra byte for the marker.
        let shared_buf = Arc::new(Mutex::new(vec![0u8; 6]));
        {
            let mut buf = shared_buf.lock().unwrap();
            buf[5] = 42; // Establish the marker.
        }

        // Enforce bounds safety by clamping the copy length to 'allowed'.
        let actual = if req > allowed { allowed } else { req };

        // Spawn a thread to perform the memory copy with the safe, clamped size.
        let buf_clone = Arc::clone(&shared_buf);
        let handle = thread::spawn(move || {
            let mut buf = buf_clone.lock().unwrap();
            unsafe {
                std::ptr::copy_nonoverlapping(source.as_ptr(), buf.as_mut_ptr(), actual);
            }
        });
        handle.join().unwrap();

        // Verify that the marker has not been altered.
        let buf = shared_buf.lock().unwrap();
        buf[5] == 42
    }
}

fn main() {
    let proc = Processor;
    // Even if the input is 6, the implementation clamps it to 5, preserving the marker.
    if proc.run(6) {
        println!("Operation successful: Marker intact");
    } else {
        println!("Operation failed: Marker corrupted");
    }
}