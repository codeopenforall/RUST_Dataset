use std::sync::{Arc, Mutex};
use std::thread;
struct Engine {
    config: i32,
}
impl Engine {
    unsafe fn new(config: i32) -> Self {
        Engine { config }
    }
    fn run(&self) {
        let shared = Arc::new(Mutex::new(self.config));
        let mut handles = Vec::new();
        for _ in 0..4 {
            let s = Arc::clone(&shared);
            let handle = thread::spawn(move || {
                unsafe {
                    let locked = s.lock().unwrap();
                    let ptr = (&*locked) as *const i32;
                    *ptr
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            let _ = handle.join().unwrap();
        }
    }
}
fn process(input: &str) -> Engine {
    let config_val = input.parse::<i32>().unwrap_or_default(); 
    unsafe { Engine::new(config_val) }
}
pub fn initialize(input: &str) -> Result<i32, String> {
    let engine = process(input);
    Ok(engine.config)
}
fn main() {
    let engine = process("bad");
    engine.run();
}
