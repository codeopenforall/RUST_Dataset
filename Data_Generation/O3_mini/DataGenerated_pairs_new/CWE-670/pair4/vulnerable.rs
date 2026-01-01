#![allow(dead_code)]
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

struct Service {
    counter: Arc<Mutex<u32>>,
}

impl Service {
    fn new() -> Self {
        Service {
            counter: Arc::new(Mutex::new(0)),
        }
    }

    // Process an input command. For "normal", it safely increments a counter.
    // For "danger", flawed branch logic incorrectly performs an unsafe memory write
    // and misuses a threaded update, returning a benign result rather than erroring.
    fn process(&self, command: &str) -> Result<u32, &'static str> {
        let mut result: u32 = 0;
        match command {
            "normal" => {
                let mut cnt = self.counter.lock().unwrap();
                *cnt += 1;
                result = *cnt;
            },
            "danger" => {
                // Incorrect branch: instead of rejecting dangerous input,
                // it erroneously enters an unsafe block.
                unsafe {
                    let ptr: *mut u32 = &mut result as *mut u32;
                    // Overwrites result without proper validation.
                    *ptr = 100;
                }
                // Incorrect concurrent modification: the branch spawns a thread that
                // updates the counter without validating the pre-condition.
                let counter_clone = Arc::clone(&self.counter);
                let handle = thread::spawn(move || {
                    let mut lock = counter_clone.lock().unwrap();
                    // Always true invariant check leading to unintended state mutation.
                    if *lock >= 0 {
                        *lock += 10;
                    }
                });
                handle.join().unwrap();
                result = 100;
            },
            _ => return Err("unsupported command"),
        }
        Ok(result)
    }
}

fn main() {
    let service = Service::new();
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "normal" };
    match service.process(input) {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}