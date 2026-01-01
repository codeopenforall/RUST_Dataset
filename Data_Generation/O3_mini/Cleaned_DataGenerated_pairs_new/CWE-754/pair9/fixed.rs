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
            if *self.cancel {
                return Err(MyError::Cancelled);
            }
        }
        thread::sleep(Duration::from_millis(10));
        Err(MyError::Timeout)
    }
}
fn main() {
    let mut flag = true;
    let runner = TaskRunner::new(&mut flag);
    match runner.run() {
        Ok(val) => println!("Operation completed with value: {}", val),
        Err(e) => println!("Operation failed with error: {:?}", e),
    }
}
