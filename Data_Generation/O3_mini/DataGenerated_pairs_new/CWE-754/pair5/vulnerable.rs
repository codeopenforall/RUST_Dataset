//////////////////////////////////////////////
// A complex example demonstrating a improper
// check for exceptional conditions (CWE-754)
// using channels, threads, unsafe pointers, and
// shared state. The error from a channel receive
// operation is mishandled by treating a timeout as
// a non-critical condition.
//////////////////////////////////////////////
use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Processor {
    shared: Arc<Mutex<u32>>,
}

impl Processor {
    fn new() -> Self {
        Processor {
            shared: Arc::new(Mutex::new(0)),
        }
    }

    // This method spawns a thread that performs an unsafe
    // read operation and then waits on a channel with a timeout.
    // The vulnerability: when recv_timeout returns a timeout error,
    // it is treated as a non-critical condition by returning Ok(0)
    // instead of reporting the error.
    fn execute(&self) -> Result<u32, &'static str> {
        let (_tx, rx) = mpsc::channel::<u32>();
        let shared_clone = Arc::clone(&self.shared);

        // Spawn a thread that does an unsafe operation.
        let _handle = thread::spawn(move || {
            unsafe {
                // Simulated unsafe read from a pointer derived from shared data.
                let data = shared_clone.lock().unwrap();
                let ptr = data.to_le_bytes().as_ptr();
                // Dummy unsafe operation
                std::ptr::read_volatile(ptr);
            }
            // Does not send any value to the channel.
        });

        // Use a short timeout to force a timeout condition.
        let timeout = Duration::from_millis(50);
        match rx.recv_timeout(timeout) {
            Ok(val) => Ok(val),
            Err(e) => match e {
                // VULNERABILITY: Incorrectly ignores the timeout condition,
                // returning a default value rather than propagating an error.
                RecvTimeoutError::Timeout => {
                    println!("Ignoring timeout condition");
                    Ok(0)
                },
                RecvTimeoutError::Disconnected => Err("Channel disconnected"),
            },
        }
    }
}

fn main() {
    let instance = Processor::new();
    match instance.execute() {
        Ok(val) => println!("Operation completed with: {}", val),
        Err(e) => eprintln!("Error encountered: {}", e),
    }
}