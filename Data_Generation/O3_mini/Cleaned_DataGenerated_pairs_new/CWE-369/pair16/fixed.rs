use std::sync::Arc;
use std::thread;
struct Processor;
impl Processor {
    fn operate(&self, num: i32, den: i32) -> Option<i32> {
        if den == 0 {
            None
        } else {
            Some(num / den)
        }
    }
}
fn main() {
    let proc = Arc::new(Processor);
    let proc_clone = Arc::clone(&proc);
    let handle = thread::spawn(move || {
        match proc_clone.operate(100, 0) {
            Some(result) => println!("Result: {}", result),
            None => println!("Error: Division by zero avoided"),
        }
    });
    let _ = handle.join();
}
