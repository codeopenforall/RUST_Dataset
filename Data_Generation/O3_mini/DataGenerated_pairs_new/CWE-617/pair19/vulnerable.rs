/* Vulnerable Rust code for CWE-617: Reachable Assertion with unsafe pointer arithmetic.
   This version spawns a thread and uses an unsafe block to read a value from an internal
   buffer based on an attacker-influenced index. If the index is such that the retrieved
   value is 3 or greater, an assertion is triggered, causing a panic (potenially leading to DoS). */
use std::env;
use std::sync::Arc;
use std::thread;

struct Processor {
    inner: Arc<Vec<u8>>,
}

impl Processor {
    fn new() -> Self {
        // Sample buffer; attacker can control the index to read.
        // The buffer elements [1, 2, 3, 4, 5] are arranged such that accessing index 2 or above
        // yields a value (3 or higher) that will trigger the assertion.
        Processor {
            inner: Arc::new(vec![1, 2, 3, 4, 5]),
        }
    }

    fn execute(&self, idx: usize) {
        // Unsafe block performing raw pointer arithmetic without prior validation.
        unsafe {
            let ptr = self.inner.as_ptr();
            // attacker-controlled index leads to a reachable assertion if the value is too high.
            let value = *ptr.add(idx);
            // This assertion is reachable with attacker influence.
            // If value >= 3, the assertion fails, causing a panic.
            assert!(value < 3, "Assertion triggered: encountered value {} which is too high", value);
            // Continue processing (in a realistic scenario, further logic would be here).
            println!("Processed value: {}", value);
        }
    }
}

fn main() {
    // Attacker-controlled input via command-line argument.
    let args: Vec<String> = env::args().collect();
    let idx: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };
    let proc_obj = Processor::new();

    // Spawn a thread to simulate concurrent processing.
    let shared = proc_obj.inner.clone();
    let handler = thread::spawn(move || {
        let local_proc = Processor { inner: shared };
        local_proc.execute(idx);
    });
    handler.join().unwrap();

    println!("Main completed in the vulnerable version");
}