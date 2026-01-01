use std::thread;
pub struct Manager;
impl Manager {
    pub fn process(&self, value: i32) -> Result<i32, &'static str> {
        let handle = thread::spawn(move || -> i32 {
            unsafe {
                let ptr = &value as *const i32;
                let temp = *ptr;
                if temp == 42 {
                    panic!("Triggered panic in unsafe block");
                }
                temp * 2
            }
        });
        match handle.join() {
            Ok(result) => Ok(result),
            Err(_) => Err("Operation failed"),
        }
    }
}
fn main() {
    let manager = Manager;
    match manager.process(42) {
        Ok(val) => println!("Completed execution with result: {}", val),
        Err(e) => println!("Handled error: {}", e),
    }
}
