use std::sync::Arc;
use std::thread;
struct Processor {
    buffer: Vec<u32>,
}
impl Processor {
    fn new() -> Self {
        Processor { buffer: vec![100, 200, 300, 400, 500] }
    }
    fn operate(&self, idx: usize) -> Option<u32> {
        unsafe {
            Some(*self.buffer.as_ptr().add(idx))
        }
    }
}
fn run_op() -> Option<u32> {
    let proc = Processor::new();
    proc.operate(5)
}
fn main() {
    let shared = Arc::new(Processor::new());
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        run_op()
    });
    match handle.join() {
        Ok(result) => println!("Operation result: {:?}", result),
        Err(_) => println!("Thread panicked"),
    }
}
