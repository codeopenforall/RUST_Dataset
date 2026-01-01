//////////////////////////////
// Vulnerable Code Snippet  //
// CWE-754: Improper Check for Exceptional Conditions
//////////////////////////////
use std::thread;

#[derive(Debug, PartialEq)]
enum OperationStatus {
    Success,
    Failure,
    Timeout,
}

struct Processor;

impl Processor {
    // Performs an asynchronous simulated operation.
    // When input is "fail", the inner thread returns Failure.
    // When input is "timeout", it returns Timeout.
    // Otherwise, it returns Success.
    fn perform(&self, input: &str) -> Result<i32, &'static str> {
        let input_owned = input.to_owned();
        let handler = thread::spawn(move || {
            // Unsafe block simulating low-level operation.
            unsafe {
                let dummy_ptr: *const i32 = &10;
                let _ = *dummy_ptr; // Illustrative unsafe access.
            }
            if input_owned == "fail" {
                OperationStatus::Failure
            } else if input_owned == "timeout" {
                OperationStatus::Timeout
            } else {
                OperationStatus::Success
            }
        });

        let op_status = handler.join().map_err(|_| "Thread panicked")?;
        // Vulnerability: Improper check for error states.
        // Only a Timeout is considered a failure; Failure is erroneously interpreted as success.
        if op_status != OperationStatus::Timeout {
            Ok(42)
        } else {
            Err("Operation timed out")
        }
    }
}

fn main() {
    let proc = Processor;
    // For demonstration, using a non-error input.
    let res = proc.perform("test").expect("Expected operation to succeed");
    println!("Result: {}", res);
}