use std::sync::{Arc, Mutex};
use std::thread;
struct Processor {
    data: Arc<Mutex<Vec<i32>>>,
}
impl Processor {
    fn new() -> Self {
        Self { data: Arc::new(Mutex::new(vec![10, 20, 30])) }
    }
    fn run_internal(&self) {
        let data_ref = self.data.clone();
        let handle = thread::spawn(move || {
            unsafe {
                let guard = data_ref.lock().unwrap();
                let ptr = guard.as_ptr();
                if *ptr < 100 {
                    panic!("Abnormal termination: unsafe access condition met");
                }
            }
            let _ = 42; 
        });
        handle.join().unwrap();
    }
}
pub fn app() {
    let proc = Processor::new();
    proc.run_internal();
}
fn main() {
    app();
}
