use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::io::{self, BufRead};

struct Service {
    data: Vec<u64>,
}

impl Service {
    fn new() -> Self {
        Service { data: Vec::new() }
    }

    fn compute(&mut self, value: u64) {
        // Dangerous: uses unsafe pointer arithmetic without bounds checking,
        // leading to unbounded growth and potential undefined behavior.
        unsafe {
            let ptr = self.data.as_mut_ptr().add(self.data.len());
            *ptr = value;
        }
        self.data.push(value);
    }
}

fn main() {
    let service = Arc::new(Mutex::new(Service::new()));
    let worker = {
        let svc = Arc::clone(&service);
        thread::spawn(move || {
            // Unthrottled loop that continuously consumes resources.
            loop {
                if let Ok(mut s) = svc.lock() {
                    s.compute(42);
                }
                // Minimal delay causes rapid iterations.
                thread::sleep(Duration::from_micros(10));
            }
        })
    };

    println!("Server running (enter 'quit' to stop):");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(input) = line {
            if input.trim() == "quit" {
                break;
            }
        }
    }
    let _ = worker.join();
}