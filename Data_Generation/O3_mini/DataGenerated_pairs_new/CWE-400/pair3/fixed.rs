//////////////////////////
// Corrected Code Sample
//////////////////////////
use std::sync::{Arc, Mutex};
use std::env;
use std::thread;
use std::time::Duration;

const MAX_CAPACITY: usize = 1024; // Enforce a safety limit on resource consumption.

pub struct Processor {
    data: Vec<u8>,
}

impl Processor {
    // The safe method now checks the length against a maximum capacity.
    pub fn append_checked(&mut self, item: u8) -> Result<(), &'static str> {
        if self.data.len() >= MAX_CAPACITY {
            return Err("Resource limit reached");
        }
        self.data.push(item);
        Ok(())
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

/// This function simulates input consumption by processing a byte slice.
/// It now enforces a strict limit on the size of the internal vector to prevent uncontrolled resource consumption.
pub fn simulate_input(input: &[u8], proc: &Arc<Mutex<Processor>>) -> Result<(), &'static str> {
    for &byte in input {
        let mut locked = proc.lock().unwrap();
        // Enforce resource limitation by using append_checked.
        locked.append_checked(byte)?;
    }
    Ok(())
}

fn main() {
    // Create a shared Processor with an enforced capacity limit.
    let proc = Arc::new(Mutex::new(Processor { data: Vec::with_capacity(MAX_CAPACITY) }));
    
    // Optionally use command-line input to simulate incoming data.
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = args[1].as_bytes();
        match simulate_input(input, &proc) {
            Ok(_) => {},
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    // Main loop simulating a continuously running service.
    loop {
        thread::sleep(Duration::from_millis(100));
    }
}