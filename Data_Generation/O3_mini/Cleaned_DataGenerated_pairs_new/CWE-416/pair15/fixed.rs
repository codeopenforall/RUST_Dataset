use std::thread;
use std::sync::Arc;
trait Operation {
    fn process(&self) -> i32;
}
struct Handler;
impl Operation for Handler {
    fn process(&self) -> i32 {
        let shared = Arc::new(42);
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            *shared_clone
        });
        handle.join().unwrap()
    }
}
pub fn compute() -> i32 {
    let proc = Handler;
    proc.process()
}
fn main() {
    let result = compute();
    println!("Result: {}", result);
}
