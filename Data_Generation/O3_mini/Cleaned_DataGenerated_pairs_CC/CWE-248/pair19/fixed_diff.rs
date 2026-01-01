use std::panic;
struct Runner {
impl Runner {
    fn run_internal(&self) -> Result<(), &'static str> {
            let result = panic::catch_unwind(|| {
                unsafe {
                    let guard = data_ref.lock().unwrap();
                    let ptr = guard.as_ptr();
                    if *ptr < 100 {
                        panic!("Abnormal termination: unsafe access condition met");
                    }
                let _ = 42; 
            });
            result.map_err(|_| ())
        match handle.join() {
            Ok(inner_result) => inner_result.map_err(|_| "Error: Panic caught in thread"),
            Err(_) => Err("Error: Thread panicked unexpectedly"),
        }
    let runner = Runner::new();
    let _ = runner.run_internal();
