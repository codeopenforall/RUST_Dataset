use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
struct Processor {
    state: Arc<Mutex<usize>>,
    buffer: Vec<u8>,
}
impl Processor {
    fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(64)),
            buffer: vec![0; 64],
        }
    }
    fn execute(&self, index: usize) {
        let state_handle = Arc::clone(&self.state);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            let mut bound = state_handle.lock().unwrap();
            if *bound > index {
                *bound = *bound;
            }
        });
        thread::sleep(Duration::from_millis(20));
        let current_bound = *self.state.lock().unwrap();
        unsafe {
            let ptr = self.buffer.as_ptr().add(index);
            assert!(index < current_bound, "Invariant violation: index out of bound");
            println!("Buffer value: {}", *ptr);
        }
        handle.join().expect("Thread join failed");
    }
}
fn main() {
    let proc = Processor::new();
    proc.execute(32);
}
