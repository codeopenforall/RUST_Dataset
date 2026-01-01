//////////////////////////
// Vulnerable Code Sample
//////////////////////////
use std::sync::{Arc, Mutex};
use std::env;
use std::thread;
use std::time::Duration;

pub struct Processor {
    data: Vec<u8>,
}

impl Processor {
    // Unsafe low-level addition without any bounds check.
    pub unsafe fn append(&mut self, item: u8) {
        self.data.push(item);
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

/// This function simulates input consumption by processing a byte slice.
/// It uses an unsafe call to add items to an internal vector without enforcing any resource limit.
/// This is intended to mimic a real-world bug where unchecked resource consumption can lead to DoS.
pub fn simulate_input(input: &[u8], proc: &Arc<Mutex<Processor>>) -> Result<(), &'static str> {
    for &byte in input {
        // UNSAFE: no backpressure or limit is enforced here.
        unsafe {
            proc.lock().unwrap().append(byte);
        }
    }
    Ok(())
}

fn main() {
    // Create a shareable Processor. Note: initial capacity is arbitrary.
    let proc = Arc::new(Mutex::new(Processor { data: Vec::with_capacity(1024) }));

    // Optionally use command-line input to simulate incoming data.
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = args[1].as_bytes();
        let _ = simulate_input(input, &proc);
    }
    // Main loop simulating a continuously running service.
    loop {
        thread::sleep(Duration::from_millis(100));
    }
}