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
    fn process(&self, val: i32) -> Result<*mut i32, ProcError> {
        let handle = thread::spawn(move || {
            if val % 2 == 0 {
                Box::new(val * 2)
            } else {
                thread::sleep(Duration::from_millis(50));
                panic!("Operation timed out");
            }
        });
        match handle.join() {
            Ok(data) => Ok(Box::into_raw(data)),
            Err(_) => Err(ProcError::Timeout),
        }
    }
    fn execute(&self, val: i32) -> Result<i32, ProcError> {
        match self.process(val) {
            Ok(ptr) => {
                let res = unsafe { *ptr };
                unsafe { Box::from_raw(ptr) };
                Ok(res)
            },
            Err(e) => {
                Err(e)
            }
        }
    }
}
fn main() {
    let worker = Worker::new();
    match worker.execute(1) {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error encountered: {:?}", e),
    }
}
