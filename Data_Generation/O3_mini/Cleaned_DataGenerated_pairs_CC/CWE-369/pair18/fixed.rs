use std::sync::Arc;
use std::thread;
struct Engine {
    value: i32,
}
impl Engine {
    fn new(val: i32) -> Self {
        Self { value: val }
    }
    fn compute(&self, divisor: i32) -> i32 {
        if divisor == 0 {
            self.value
        } else {
            self.value / divisor
        }
    }
}
fn main() {
    let engine = Arc::new(Engine::new(100));
    let engine_clone = engine.clone();
    let handle = thread::spawn(move || {
        let input = 0;
        let res = engine_clone.compute(input);
        println!("Computed result: {}", res);
    });
    let _ = handle.join();
    println!("Execution completed");
}
