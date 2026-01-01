use std::thread;
use std::panic::{catch_unwind, AssertUnwindSafe};
struct Manager;
impl Manager {
    fn new() -> Self {
        Self {}
    }
    fn process(&self) -> Result<(), String> {
        let th = thread::spawn(|| {
            unsafe {
                let mut value: i32 = 0;
                let raw_ptr: *mut i32 = &mut value;
                let result = catch_unwind(AssertUnwindSafe(|| {
                    if *raw_ptr == 0 {
                        panic!("abnormal termination: unexpected condition encountered");
                    }
                }));
                result.map_err(|_| "panic caught inside thread".to_string())
            }
        });
        match th.join() {
            Ok(inner_result) => inner_result,
            Err(_) => Err("thread panicked and join error occurred".to_string()),
        }
    }
}
fn run_system() {
    let mgr = Manager::new();
    if let Err(e) = mgr.process() {
        println!("Error handled: {}", e);
    }
}
fn main() {
    run_system();
}
