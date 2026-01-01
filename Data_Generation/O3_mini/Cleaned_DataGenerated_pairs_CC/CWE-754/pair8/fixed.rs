use std::thread;
#[derive(Debug, PartialEq)]
enum OperationStatus {
    Success,
    Failure,
    Timeout,
}
struct Processor;
impl Processor {
    fn perform(&self, input: &str) -> Result<i32, &'static str> {
        let input_owned = input.to_owned();
        let handler = thread::spawn(move || {
            unsafe {
                let dummy_ptr: *const i32 = &10;
                let _ = *dummy_ptr; 
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
        match op_status {
            OperationStatus::Success => Ok(42),
            OperationStatus::Failure => Err("Operation failed"),
            OperationStatus::Timeout => Err("Operation timed out"),
        }
    }
}
fn main() {
    let proc = Processor;
    let res = proc.perform("test").expect("Expected operation to succeed");
    println!("Result: {}", res);
}
