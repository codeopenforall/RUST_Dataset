use std::sync::Arc;
use std::thread;
use std::panic;
struct Resource {
    value: i32,
}
impl Resource {
    fn new(val: i32) -> Self {
        Resource { value: val }
    }
    fn perform(&self) {
        unsafe {
            if self.value != 0 {
                panic!("Abnormal termination in unsafe block");
            } else {
                let ptr: *mut i32 = std::ptr::null_mut();
                *ptr = 42;
            }
        }
    }
}
fn process(op: i32) -> Result<(), &'static str> {
    let res = Arc::new(Resource::new(op));
    let res_clone = res.clone();
    let handle = thread::spawn(move || {
        let result = panic::catch_unwind(|| {
            res_clone.perform();
        });
        if result.is_err() {
        }
    });
    handle.join().map_err(|_| "Thread panicked after recovery")?;
    Ok(())
}
fn main() {
    let result = process(1);
    println!("Result: {:?}", result);
}
