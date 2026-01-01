////////////////////////////////////
// Vulnerability Example in Rust //
// CWE-475: Undefined Behavior for   //
// passing invalid parameters to     //
// an unsafe API (copy_nonoverlapping)
////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Processor;

impl Processor {
    // Executes an unsafe memory copy using the user-specified length.
    // The destination buffer is allocated with 5 valid bytes and 1 extra byte
    // used as a marker. If the requested copy length exceeds 5, the marker
    // gets overwritten, violating the API preconditions.
    fn run(&self, req: usize) -> bool {
        let allowed = 5;
        let source = vec![1u8; 10]; // Source buffer of fixed 10 bytes.
        // Destination buffer of 6 bytes: first 5 are valid data, the 6th is a marker.
        let shared_buf = Arc::new(Mutex::new(vec![0u8; 6]));
        {
            let mut buf = shared_buf.lock().unwrap();
            buf[5] = 42; // Marker is set to a known value.
        }

        // Spawn a thread that performs the unsafe memory copy.
        let buf_clone = Arc::clone(&shared_buf);
        let handle = thread::spawn(move || {
            let mut buf = buf_clone.lock().unwrap();
            // Vulnerability: the user-provided length 'req' is used
            // without checking that it does not exceed the destination's capacity.
            unsafe {
                std::ptr::copy_nonoverlapping(source.as_ptr(), buf.as_mut_ptr(), req);
            }
        });
        handle.join().unwrap();

        // After the copy, verify that the marker remains intact.
        let buf = shared_buf.lock().unwrap();
        buf[5] == 42
    }
}

fn main() {
    let proc = Processor;
    // Trigger vulnerability by requesting a copy of 6 bytes,
    // which exceeds the allowed 5 bytes and overwrites the marker.
    if proc.run(6) {
        println!("Operation successful: Marker intact");
    } else {
        println!("Operation failed: Marker corrupted");
    }
}