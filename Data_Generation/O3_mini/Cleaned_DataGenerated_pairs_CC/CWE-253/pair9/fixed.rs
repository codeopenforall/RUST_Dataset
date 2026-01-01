use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
struct Handler {
    shared: Arc<Mutex<i32>>,
}
impl Handler {
    unsafe fn obtain(ptr: *const i32) -> Option<i32> {
        if ptr.is_null() {
            None
        } else {
            Some(*ptr)
        }
    }
    fn operate(&self) -> Result<i32, &'static str> {
        let data = {
            let lock = self.shared.lock().unwrap();
            *lock
        };
        let result: Option<i32> = unsafe { Self::obtain(&data as *const i32) };
        if let Some(val) = result {
            Ok(val)
        } else {
            Err("Operation failed")
        }
    }
    fn execute(&self) -> Result<i32, &'static str> {
        let clone_shared = self.shared.clone();
        let handler = thread::spawn(move || {
            let temp = {
                let lock = clone_shared.lock().unwrap();
                *lock
            };
            let res = unsafe { Self::obtain(&temp as *const i32) };
            if let Some(val) = res {
                Ok(val)
            } else {
                Err("Thread operation failed")
            }
        });
        handler.join().unwrap()
    }
}
fn main() {
    let instance = Handler { shared: Arc::new(Mutex::new(42)) };
    match instance.operate() {
        Ok(val) => println!("Operation succeeded with value {}", val),
        Err(err) => println!("Operation error: {}", err),
    }
    match instance.execute() {
        Ok(val) => println!("Execution succeeded with value {}", val),
        Err(err) => println!("Execution error: {}", err),
    }
}
