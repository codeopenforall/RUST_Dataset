use std::sync::Arc;
use std::thread;
struct Engine {
    base: i32,
}
impl Engine {
    fn new(base: i32) -> Self {
        Engine { base }
    }
    fn process(&self, input: &str) -> i32 {
        let divisor: i32 = input.parse().unwrap_or_default();  
        unsafe {
            let ptr = &self.base as *const i32;
            let value = *ptr;
            value / divisor
        }
    }
}
pub fn perform(input: &str) -> Result<i32, String> {
    let engine = Engine::new(100);
    Ok(engine.process(input))
}
fn main() {
    let engine = Arc::new(Engine::new(100));
    let arg = std::env::args().nth(1).unwrap_or_default();
    let mut handles = Vec::new();
    for _ in 0..4 {
        let engine = Arc::clone(&engine);
        let arg_clone = arg.clone();
        handles.push(thread::spawn(move || {
            let res = engine.process(&arg_clone);
            println!("Computed result: {}", res);
        }));
    }
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}
