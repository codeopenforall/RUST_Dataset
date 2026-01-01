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
    fn update(&mut self, new_value: usize) -> Result<(), String> {
        if new_value > self.limit {
            return Err("Attempt to set value above allowed maximum".to_owned());
        }
        self.counter = new_value;
        Ok(())
    }
}
pub fn process_input(input: usize) -> Result<(), String> {
    let mut worker = Worker::new(100);
    worker.update(input)
}
fn main() {
    let worker = Arc::new(Mutex::new(Worker::new(100)));
    let worker_clone = Arc::clone(&worker);
    let handle = thread::spawn(move || {
        let mut worker = worker_clone.lock().unwrap();
        let res = worker.update(150);
        assert!(res.is_err(), "Expected error for input exceeding limit.");
    });
    let _ = handle.join();
    println!("Completed processing in fixed version.");
}
