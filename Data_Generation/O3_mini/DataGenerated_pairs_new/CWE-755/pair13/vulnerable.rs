//////////////////////////////////////////////
// Vulnerable Code Example
//////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Handler {
    value: u64,
}

impl Handler {
    // The constructor improperly swallows parsing errors by using unwrap_or_default.
    fn new(input: &str) -> Self {
        // Vulnerability: if parsing fails, unwrap_or_default returns 0
        let parsed = input.parse::<u64>().unwrap_or_default(); // CWE-755 issue
        // Unwarranted unsafe pointer usage for realism.
        unsafe {
            let ptr: *const u64 = &parsed;
            let _ = *ptr;
        }
        Handler { value: parsed }
    }

    fn increment(&mut self) {
        unsafe {
            // Unsafe pointer arithmetic for demonstration.
            let raw = &mut self.value as *mut u64;
            *raw = self.value.wrapping_add(1);
        }
    }
}

// Public API function that processes the input.
// On invalid input it silently converts an error into 0 then increments to 1.
fn process_input(input: &str) -> Result<u64, String> {
    let mut handler = Handler::new(input);
    handler.increment();
    Ok(handler.value)
}

fn main() {
    // Emulate concurrency with worker threads.
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut threads = vec![];

    for _ in 0..4 {
        let res_clone = Arc::clone(&results);
        let inp = "invalid";  // Trigger input that should produce an error ideally.
        threads.push(thread::spawn(move || {
            // Vulnerable behavior: instead of erroring, it produces a default value.
            let result = process_input(inp).unwrap();
            let mut guard = res_clone.lock().unwrap();
            guard.push(result);
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }
    let guard = results.lock().unwrap();
    let sum: u64 = guard.iter().sum();
    println!("Sum: {}", sum);
}