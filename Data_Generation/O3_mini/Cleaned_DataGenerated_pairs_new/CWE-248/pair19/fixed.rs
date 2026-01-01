use std::sync::{Arc, Mutex};
use std::thread;
use std::panic;
struct Runner {
    data: Arc<Mutex<Vec<i32>>>,
}
impl Runner {
    fn new() -> Self {
        Self { data: Arc::new(Mutex::new(vec![10, 20, 30])) }
    }
    fn run_internal(&self) -> Result<(), &'static str> {
        let data_ref = self.data.clone();
        let handle = thread::spawn(move || {
            let result = panic::catch_unwind(|| {
                unsafe {
                    let guard = data_ref.lock().unwrap();
                    let ptr = guard.as_ptr();
                    if *ptr < 100 {
                        panic!("Abnormal termination: unsafe access condition met");
                    }
                }
                let _ = 42; 
            });
            result.map_err(|_| ())
        });
        match handle.join() {
            Ok(inner_result) => inner_result.map_err(|_| "Error: Panic caught in thread"),
            Err(_) => Err("Error: Thread panicked unexpectedly"),
        }
    }
}
pub fn app() {
    let runner = Runner::new();
    let _ = runner.run_internal();
}
fn main() {
    app();
}
