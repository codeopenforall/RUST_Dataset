use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq)]
enum MyError {
    Timeout,
    Cancelled,
}

struct TaskRunner {
    cancel: *mut bool,
}

impl TaskRunner {
    fn new(flag: &mut bool) -> TaskRunner {
        TaskRunner { cancel: flag as *mut bool }
    }

    fn run(&self) -> Result<u32, MyError> {
        unsafe {
            // Improper exceptional condition check:
            // When the cancellation flag is set, instead of returning an error,
            // it erroneously returns a success value.
            if *self.cancel {
                return Ok(100); // Vulnerability: returning success instead of cancellation error.
            }
        }
        // Simulate some work.
        thread::sleep(Duration::from_millis(10));
        Err(MyError::Timeout)
    }
}

fn main() {
    let mut flag = true;
    let runner = TaskRunner::new(&mut flag);
    match runner.run() {
        Ok(val) => println!("Operation completed successfully with: {}", val),
        Err(e) => println!("Operation failed with error: {:?}", e),
    }
}