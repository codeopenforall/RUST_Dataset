//////////////////////////////
// Fixed Code Snippet       //
// CWE-754 Resolution
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
    // Correctly performs an asynchronous simulated operation.
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
        // Fixed: Properly check for a successful outcome.
        // Only if the status is Success is the operation considered OK.
        match op_status {
            OperationStatus::Success => Ok(42),
            OperationStatus::Failure => Err("Operation failed"),
            OperationStatus::Timeout => Err("Operation timed out"),
        }
    }
}

fn main() {
    let proc = Processor;
    // For demonstration, using a non-error input.
    let res = proc.perform("test").expect("Expected operation to succeed");
    println!("Result: {}", res);
}