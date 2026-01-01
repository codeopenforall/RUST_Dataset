//////////////////////////////////////////////////////////
// Vulnerable code snippet demonstrating improper input validation
// against CWE-020 using unsafe pointer arithmetic in a concurrent context.
//////////////////////////////////////////////////////////
use std::env;
use std::sync::Arc;
use std::thread;

struct Handler {
    data: Box<[u8]>,
}

impl Handler {
    fn new() -> Self {
        // Initialize a buffer of 100 bytes with non-zero dummy data.
        let mut vec = vec![1u8; 100];
        vec[50] = 255; // arbitrary modification
        Handler {
            data: vec.into_boxed_slice(),
        }
    }

    fn execute(&self, input: &str) -> Result<(), &'static str> {
        // Parse user input as a hexadecimal string to determine the starting index.
        let idx = usize::from_str_radix(input, 16).map_err(|_| "parse error")?;
        let mut buffer = vec![0u8; 10];
        // BEGIN UNSAFE BLOCK: Failing to validate that idx+buffer.len() is within bounds.
        unsafe {
            // If idx + 10 exceeds the allocated data length, this will access out-of-bounds memory.
            let src = self.data.as_ptr().add(idx);
            std::ptr::copy_nonoverlapping(src, buffer.as_mut_ptr(), 10);
        }
        // END UNSAFE BLOCK

        // Process the copied data.
        let sum: u8 = buffer.iter().fold(0, |acc, &x| acc.wrapping_add(x));
        if sum == 0 {
            Err("data sum is zero")
        } else {
            Ok(())
        }
    }
}

fn run_app() {
    // Use the first command line argument as input; default is "00" which is safe.
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "00" };
    let handler = Arc::new(Handler::new());

    // Spawn a concurrent thread using the handler.
    let handler_clone = Arc::clone(&handler);
    let input_clone = input.to_string();
    let handle = thread::spawn(move || {
        match handler_clone.execute(&input_clone) {
            Ok(_)  => println!("Concurrent execution succeeded."),
            Err(e) => println!("Concurrent execution failed: {}", e),
        }
    });

    // Main thread execution.
    match handler.execute(input) {
        Ok(_)  => println!("Main execution succeeded."),
        Err(e) => println!("Main execution failed: {}", e),
    }

    handle.join().unwrap();
}

fn main() {
    run_app();
}