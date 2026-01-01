/* 
This Rust code mimics a realistic multi-threaded scenario where an unsafe external API call is used
to simulate an I/O operation. The implementation concurrently spawns a worker thread to call the unsafe
function and update a shared state wrapped in a Mutex. However, due to an improper check for exceptional
conditions (CWE-754), errors such as a timeout are misinterpreted: the error branch uses the wrong error kind
(“WouldBlock” instead of “TimedOut”) and always returns a successful result.
*/
use std::io;
use std::io::ErrorKind;
use std::sync::{Arc, Mutex};
use std::thread;

// Simulated unsafe external API call.
// Returns Ok(u32) on success or an error with TimedOut on failure.
unsafe fn external_call(should_succeed: bool) -> io::Result<u32> {
    if should_succeed {
        Ok(100)
    } else {
        Err(io::Error::new(io::ErrorKind::TimedOut, "operation timed out"))
    }
}

// A trait to abstract processing operations.
pub trait Runner {
    fn process(&self, flag: bool) -> Result<u32, &'static str>;
}

// A struct that owns some shared state and implements the Runner trait.
pub struct Engine {
    pub data: Arc<Mutex<u32>>,
}

impl Runner for Engine {
    fn process(&self, flag: bool) -> Result<u32, &'static str> {
        // Spawn a worker thread to simulate a real concurrent context.
        let handle = {
            let data = Arc::clone(&self.data);
            thread::spawn(move || {
                // Invoke the unsafe external API call.
                let result = unsafe { external_call(flag) };
                let mut d = data.lock().unwrap();
                match result {
                    Ok(val) => {
                        *d = val;
                        Ok(val)
                    }
                    Err(e) => {
                        // Vulnerability: improper check for exceptional condition.
                        // The code erroneously checks for ErrorKind::WouldBlock, and regardless of the error,
                        // treats the failure case as a successful outcome.
                        if e.kind() == ErrorKind::WouldBlock {
                            *d = 1000;
                            Ok(1000)
                        } else {
                            *d = 1000;
                            Ok(1000)
                        }
                    }
                }
            })
        };

        // Wait for the thread to complete and return its result.
        handle.join().unwrap()
    }
}

// Public function used by external code; it creates an Engine and executes the processing.
pub fn process_task(flag: bool) -> Result<u32, &'static str> {
    let engine = Engine {
        data: Arc::new(Mutex::new(0)),
    };
    engine.process(flag)
}

fn main() {
    // Here 'true' simulates a normal successful operation.
    let flag = true;
    match process_task(flag) {
        Ok(val) => println!("Result: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}