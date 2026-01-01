use std::sync::{Arc, Mutex};
use std::thread;
struct DataBuffer {
    data: Vec<u32>,
}
impl DataBuffer {
    fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        data.resize(size, 0);
        Self { data }
    }
    fn populate(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = i as u32;
        }
    }
    fn total(&self) -> u32 {
        self.data.iter().sum()
    }
}
fn compute() -> u32 {
    let shared = Arc::new(Mutex::new(DataBuffer::new(10)));
    let shared_clone = Arc::clone(&shared);
    let handler = thread::spawn(move || {
        let mut guard = shared_clone.lock().expect("failed to lock");
        guard.populate();
    });
    handler.join().expect("Thread panicked");
    let guard = shared.lock().expect("failed to lock after join");
    guard.total()
}
fn main() {
    let result = compute();
    println!("Result: {}", result);
}
