////////////////// VULNERABLE CODE //////////////////
use std::sync::Arc;
use std::thread;

type Operation = fn(i32) -> i32;

// A sensitive operation that must not be invoked when in safe mode.
fn sensitive(val: i32) -> i32 {
    // Simulate a critical operation.
    val * 10
}

// A non‐sensitive operation.
fn nonsensitive(val: i32) -> i32 {
    // Perform a benign computation.
    val + 5
}

struct Dispatcher {
    table: [Operation; 2],
    safe_mode: bool,
}

impl Dispatcher {
    // Constructor: safe_mode true means the sensitive operation must be suppressed.
    fn new(safe_mode: bool) -> Self {
        Dispatcher {
            // Index 0: benign, Index 1: sensitive.
            table: [nonsensitive, sensitive],
            safe_mode,
        }
    }

    // Core processing method.
    // Vulnerability: despite safe_mode being active, the selection of the function pointer is controlled solely by untrusted input.
    fn process(&self, input: i32) -> i32 {
        unsafe {
            // Determining index based solely on untrusted input.
            // For even numbers, index 0 (nonsensitive) is chosen.
            // For odd numbers, index 1 (sensitive) is chosen.
            let index = if input % 2 == 0 { 0 } else { 1 };
            // The vulnerable flaw: Ignoring the safe_mode flag and directly using the untrusted input-derived index.
            let func_ptr = *self.table.get_unchecked(index as usize);
            func_ptr(input)
        }
    }
}

fn main() {
    // In this configuration, safe_mode is activated.
    // Expectation: sensitive() must never be called.
    let dispatcher = Arc::new(Dispatcher::new(true));
    let mut handles = vec![];

    for _ in 0..4 {
        let disp = Arc::clone(&dispatcher);
        handles.push(thread::spawn(move || {
            // Using a test input that is odd (e.g., 3) triggers the sensitive branch.
            let result = disp.process(3);
            println!("Outcome: {}", result);
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}