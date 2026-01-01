use std::sync::{Arc};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;
struct Processor {
    state: Arc<AtomicUsize>,
    buffer: Vec<u8>,
}
impl Processor {
    fn new() -> Self {
        Self {
            state: Arc::new(AtomicUsize::new(64)),
            buffer: vec![0; 64],
        }
    }
    fn execute(&self, index: usize) {
        let state_handle = Arc::clone(&self.state);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            state_handle.store(0, Ordering::Relaxed); 
        });
        thread::sleep(Duration::from_millis(20));
        unsafe {
            let ptr = self.buffer.as_ptr().add(index); 
            let current_bound = self.state.load(Ordering::Relaxed);
            assert!(index < current_bound, "Invariant violation: index out of bound"); 
            println!("Buffer value: {}", *ptr);
        }
    }
}
fn main() {
    let proc = Processor::new();
    proc.execute(32);
}
