use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
struct Context {
    cancel_flag: Arc<AtomicBool>,
    timeout_flag: Arc<AtomicBool>,
}
impl Context {
    fn new(cancel: bool, timeout: bool) -> Self {
        Context {
            cancel_flag: Arc::new(AtomicBool::new(cancel)),
            timeout_flag: Arc::new(AtomicBool::new(timeout)),
        }
    }
    fn execute(&self) -> Result<i32, &'static str> {
        if !self.cancel_flag.load(Ordering::SeqCst) {
            unsafe {
                let raw_ptr = Box::into_raw(Box::new(256));
                let result = *raw_ptr; 
                Box::from_raw(raw_ptr);
                Ok(result)
            }
        } else {
            Err("Operation cancelled")
        }
    }
}
fn main() {
    let ctx = Context::new(false, true);
    match ctx.execute() {
        Ok(val) => println!("Success: Result is {}", val),
        Err(err) => println!("Error: {}", err),
    }
}
