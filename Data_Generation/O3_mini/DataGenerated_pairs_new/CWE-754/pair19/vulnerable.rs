use std::sync::mpsc;
use std::time::Duration;
use std::thread;

// A processing contract used by our executor.
trait Processor {
    fn run(&self, input: &str) -> Result<i32, &'static str>;
}

// A simple executor that performs the computation.
struct Executor;

impl Processor for Executor {
    fn run(&self, input: &str) -> Result<i32, &'static str> {
        compute(input)
    }
}

// Performs a computation that spawns a worker thread and waits for the result.
// If the worker takes too long (e.g. when input == "delay"), a timeout occurs.
// Vulnerability: On timeout, the error condition is improperly handled.
// Instead of returning an error, the timeout branch erroneously performs an unsafe memory read.
fn compute(input: &str) -> Result<i32, &'static str> {
    let (tx, rx) = mpsc::channel();
    let input_owned = input.to_string();

    thread::spawn(move || {
        // Simulate a delayed operation.
        if input_owned == "delay" {
            thread::sleep(Duration::from_millis(200));
        }
        // Compute a simple result.
        let val = input_owned.len() as i32;
        let _ = tx.send(val);
    });

    match rx.recv_timeout(Duration::from_millis(100)) {
        Ok(v) => Ok(v),
        Err(err) => {
            // Vulnerable: Incorrectly distinguishing error conditions.
            // It only checks for a disconnected channel and mistakenly treats timeout as benign.
            if let mpsc::RecvTimeoutError::Disconnected = err {
                Ok(0)
            } else {
                // Unsafe block: Dereferencing uninitialized memory when a timeout occurs.
                unsafe {
                    let bogus = std::mem::MaybeUninit::<i32>::uninit().assume_init();
                    Ok(bogus)
                }
            }
        }
    }
}

fn main() {
    let executor = Executor;
    let args: Vec<String> = std::env::args().collect();
    // Use provided command line argument or default to "test".
    let input = if args.len() > 1 { args[1].as_str() } else { "test" };
    match executor.run(input) {
        Ok(result) => println!("Success: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}