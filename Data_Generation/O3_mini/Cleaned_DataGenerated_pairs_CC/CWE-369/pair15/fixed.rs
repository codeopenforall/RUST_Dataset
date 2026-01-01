use std::sync::{Arc, Mutex};
use std::thread;
struct Engine {
    value: i32,
}
impl Engine {
    fn new(val: i32) -> Self {
        Self { value: val }
    }
    fn exec(&self, divisor: i32) -> Result<i32, &'static str> {
        if divisor == 0 {
            return Err("division by zero");
        }
        unsafe {
            let result = self.value / divisor;
            Ok(result)
        }
    }
}
fn main() {
    let engine = Arc::new(Mutex::new(Engine::new(100)));
    let engine_clone = Arc::clone(&engine);
    let handle = thread::spawn(move || {
        let engine = engine_clone.lock().unwrap();
        let res = engine.exec(0);
        assert!(res.is_err(), "Should return error on division by zero");
    });
    let _ = handle.join();
    println!("Operation completed safely.");
}
