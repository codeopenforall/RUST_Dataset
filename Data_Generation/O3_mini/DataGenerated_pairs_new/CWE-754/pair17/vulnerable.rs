use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq)]
enum ProcError {
    Timeout,
    Other,
}

struct Worker;

impl Worker {
    fn new() -> Self {
        Worker {}
    }

    // Simulates a concurrent process that may “time out”
    fn process(&self, val: i32) -> Result<*mut i32, ProcError> {
        let handle = thread::spawn(move || {
            // For even input, simulate proper computation.
            // For odd input, simulate a timeout via a panic.
            if val % 2 == 0 {
                Box::new(val * 2)
            } else {
                // Delay then force a panic to simulate a timeout condition.
                thread::sleep(Duration::from_millis(50));
                panic!("Operation timed out");
            }
        });

        // Join the thread. A panic is interpreted as a timeout error here.
        match handle.join() {
            Ok(data) => Ok(Box::into_raw(data)),
            Err(_) => Err(ProcError::Timeout),
        }
    }

    // Improper check for exceptional conditions:
    // Instead of propagating the Timeout error, it recovers by treating it as a normal outcome.
    fn execute(&self, val: i32) -> Result<i32, ProcError> {
        match self.process(val) {
            Ok(ptr) => {
                let res = unsafe { *ptr };
                // Free the allocated memory.
                unsafe { Box::from_raw(ptr) };
                Ok(res)
            },
            Err(e) => {
                // Vulnerability: Instead of returning an error when a timeout occurs,
                // the error condition is misinterpreted and a default value is used.
                if let ProcError::Timeout = e {
                    let default_box = Box::new(100);
                    let default_ptr = Box::into_raw(default_box);
                    let res = unsafe { *default_ptr };
                    // Note: The allocated memory is intentionally leaked here.
                    Ok(res)
                } else {
                    Ok(0)
                }
            }
        }
    }
}

fn main() {
    let worker = Worker::new();
    // Use an odd value to trigger a simulated timeout.
    // In this faulty version, a Timeout error is converted into a default value.
    match worker.execute(1) {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {:?}", e),
    }
}