use std::sync::Arc;
use std::thread;
struct Processor;
impl Processor {
    fn operate(&self, num: i32, den: i32) -> Option<i32> {
        unsafe {
            Some(num / den)
        }
    }
}
fn main() {
    let proc = Arc::new(Processor);
    let proc_clone = Arc::clone(&proc);
    let handle = thread::spawn(move || {
        let result = proc_clone.operate(100, 0);
        println!("Result: {:?}", result);
    });
    let _ = handle.join();
}
