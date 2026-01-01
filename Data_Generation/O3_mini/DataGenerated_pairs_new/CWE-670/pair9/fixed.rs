////////////////// FIXED CODE //////////////////
use std::sync::Arc;
use std::thread;

type Operation = fn(i32) -> i32;

fn sensitive(val: i32) -> i32 {
    // Critical operation that must not be executed in safe mode.
    val * 10
}

fn nonsensitive(val: i32) -> i32 {
    // Benign computation.
    val + 5
}

struct Dispatcher {
    table: [Operation; 2],
    safe_mode: bool,
}

impl Dispatcher {
    fn new(safe_mode: bool) -> Self {
        Dispatcher {
            table: [nonsensitive, sensitive],
            safe_mode,
        }
    }

    // Core processing method.
    // Correction: When in safe mode, always use the nonsensitive function regardless of untrusted input.
    fn process(&self, input: i32) -> i32 {
        if self.safe_mode {
            // In safe mode, force the benign operation.
            self.table[0](input)
        } else {
            // When not in safe mode, use the untrusted input to choose the operation safely.
            // Even here, we perform bounds checking.
            let index = if input % 2 == 0 { 0 } else { 1 };
            // Safe lookup with explicit indexing.
            self.table[index](input)
        }
    }
}

fn main() {
    // Activate safe mode to ensure sensitive operation is not executed.
    let dispatcher = Arc::new(Dispatcher::new(true));
    let mut handles = vec![];

    for _ in 0..4 {
        let disp = Arc::clone(&dispatcher);
        handles.push(thread::spawn(move || {
            // Using the same test input.
            let result = disp.process(3);
            println!("Outcome: {}", result);
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}