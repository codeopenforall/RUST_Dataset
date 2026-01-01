//////////////////////////
// Vulnerable Code Sample
//////////////////////////
use std::env;
use std::thread;
use std::panic;

pub struct Engine;

impl Engine {
    // This recursive function uses an unsafe block to read raw bytes from the input slice.
    // It does not enforce any limit on the recursion depth.
    pub unsafe fn explore(&self, data: *const u8, len: usize, idx: usize) -> i32 {
        if idx >= len {
            return 0;
        }
        // Unsafe read of the byte at index without bounds check.
        let byte = *data.add(idx);
        // If the byte is an opening parenthesis, we recursively call explore.
        // There is NO protection against deeply nested inputs.
        if byte == b'(' {
            // The recursive call is made in an unsafe block without checking for max depth.
            return 1 + self.explore(data, len, idx + 1);
        } else {
            return 0;
        }
    }

    // Runs the recursive exploration by starting at index 0.
    pub fn journey(&self, input: &[u8]) -> i32 {
        unsafe { self.explore(input.as_ptr(), input.len(), 0) }
    }
}

// Public API to evaluate the input.
// Returns Ok(result) if completed normally. However, for deeply nested inputs it may trigger a stack overflow.
pub fn evaluate(input: &str) -> Result<i32, &'static str> {
    let engine = Engine;
    let inp = input.as_bytes().to_vec(); // move input to owned vector
    // We spawn a thread to isolate the recursive call.
    let handler = thread::spawn(move || {
        // This call may overflow the stack if the input is maliciously deep (CWE-674).
        engine.journey(&inp)
    });
    // Join the thread; if a panic occurred (such as stack overflow), we propagate an error.
    match handler.join() {
        Ok(res) => Ok(res),
        Err(_) => Err("Unexpected recursive overflow occurred"),
    }
}

fn main() {
    // Reads input from first command line argument or uses a default deep nested string.
    let input = env::args().nth(1).unwrap_or_else(|| "(".repeat(10000));
    match evaluate(&input) {
        Ok(res) => println!("Computation successful, result: {}", res),
        Err(err) => println!("Computation failed with error: {}", err),
    }
}