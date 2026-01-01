use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
pub struct Worker {
    pub counter: u32,
    pub finished: bool,
}
impl Worker {
    pub fn new() -> Self {
        Worker { counter: 0, finished: false }
    }
    pub fn process(&mut self) {
        if self.counter < 10 {
            self.counter += 1;
            thread::sleep(Duration::from_millis(1));
            if self.counter == 10 {
                self.finished = true;
            }
        }
    }
}
pub fn run_state() -> (u32, bool) {
    let worker = Arc::new(Mutex::new(Worker::new()));
    let mut threads = vec![];
    for _ in 0..15 {
        let w = Arc::clone(&worker);
        let handle = thread::spawn(move || {
            let mut worker = w.lock().unwrap();
            worker.process();
        });
        threads.push(handle);
    }
    for t in threads {
        t.join().unwrap();
    }
    let final_state = worker.lock().unwrap();
    (final_state.counter, final_state.finished)
}
fn main() {
    let (counter, finished) = run_state();
    println!("Counter: {}, Finished: {}", counter, finished);
}
