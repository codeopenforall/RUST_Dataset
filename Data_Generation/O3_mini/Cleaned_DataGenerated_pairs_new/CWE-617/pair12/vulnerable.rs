use std::sync::{Arc, Mutex};
use std::thread;
struct Worker {
    counter: usize,
    limit: usize,
}
impl Worker {
    fn new(limit: usize) -> Self {
        Worker { counter: 0, limit }
    }
    fn update(&mut self, new_value: usize) {
        unsafe {
            let ptr = &mut self.counter as *mut usize;
            *ptr = new_value;
        }
        assert!(self.counter <= self.limit, "Value exceeds maximum allowed");
    }
}
pub fn process_input(input: usize) -> Result<(), String> {
    let mut worker = Worker::new(100);
    worker.update(input);
    Ok(())
}
fn main() {
    let worker = Arc::new(Mutex::new(Worker::new(100)));
    let worker_clone = Arc::clone(&worker);
    let handle = thread::spawn(move || {
        let mut worker = worker_clone.lock().unwrap();
        worker.update(150);
    });
    let _ = handle.join();
    println!("Completed processing in vulnerable version.");
}
