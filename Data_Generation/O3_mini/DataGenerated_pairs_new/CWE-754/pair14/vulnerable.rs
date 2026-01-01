//////////// Vulnerable Code Begin ////////////
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

trait Operate {
    fn process(&self) -> Result<u32, &'static str>;
}

struct Procedure {
    // delay in milliseconds for the spawned task to compute result
    compute_delay: u64,
    // maximum wait time (in milliseconds) for the result to be ready
    timeout: u64,
}

impl Operate for Procedure {
    fn process(&self) -> Result<u32, &'static str> {
        // Create a channel to receive computation
        let (tx, rx) = mpsc::channel();
        let delay = self.compute_delay;
        // Spawn a thread to simulate heavy computation
        thread::spawn(move || {
            // simulate delayed computation
            thread::sleep(Duration::from_millis(delay));
            let value: u32 = 256;
            tx.send(value).unwrap();
        });
        // Wait for result with a specified timeout
        let _start = Instant::now();
        let outcome = rx.recv_timeout(Duration::from_millis(self.timeout));
        // Unsafe block: Improperly handling exceptional conditions.
        // Instead of checking for a timeout error, it misinterprets any Err value by
        // reading from a pointer to a constant value, effectively treating timeouts as success.
        unsafe {
            match outcome {
                Ok(result) => Ok(result),
                Err(_) => {
                    // Vulnerability: Misinterpreting the timeout by assuming a valid result exists.
                    // It reads from a pointer to a constant instead of propagating the error.
                    let bogus_ptr: *const u32 = &256;
                    let bogus_val = bogus_ptr.read();
                    Ok(bogus_val)
                }
            }
        }
    }
}

fn main() {
    let proc = Procedure { compute_delay: 200, timeout: 100 };
    // In this configuration, the spawned computation takes longer than the allowed timeout.
    // However, due to improper error checking, the function treats the timeout as success.
    match proc.process() {
        Ok(val) => println!("Outcome: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}
//////////// Vulnerable Code End ////////////