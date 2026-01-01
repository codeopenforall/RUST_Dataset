//////////////////////////////////////////////////////////////////////////////////////////////////////
// This modified Rust program implements proper input validation to ensure that the user-provided byte count
// does not exceed the bounds of the fixed buffer. It uses safe conversion mechanisms to prevent memory violations.
// The fix addresses CWE-020 by adding an explicit bounds check before unsafely reconstructing the string.
//////////////////////////////////////////////////////////////////////////////////////////////////////

use std::sync::Arc;
use std::env;
use std::thread;

fn process_data(n: usize) -> Result<String, &'static str> {
    let buf: Arc<[u8; 10]> = Arc::new(*b"0123456789");

    // FIX: Validate that the requested length does not exceed the available buffer length.
    if n > buf.len() {
        return Err("Input length exceeds allowed range");
    }

    let shared_buf = buf.clone();
    let handle = thread::spawn(move || {
        // We can now safely use unsafe block since we’ve validated the length.
        unsafe {
            let slice = std::slice::from_raw_parts(shared_buf.as_ptr(), n);
            // Using from_utf8 here for added safety (but it should always succeed given the buffer).
            std::str::from_utf8(slice).unwrap()
        }
    });

    handle.join().map(|s| s.to_string()).map_err(|_| "Thread failed")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let num = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(8)
    } else {
        8
    };

    match process_data(num) {
        Ok(s) => println!("Result: {}", s),
        Err(e) => println!("Error: {}", e),
    }
}