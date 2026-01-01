use std::sync::{Arc, Mutex};
use std::thread;
struct Container {
    buffer: Vec<u32>,
}
impl Container {
    fn new() -> Self {
        Container { buffer: vec![0; 10] }
    }
    fn update(&mut self, index: usize, value: u32) {
        if value >= 1000 {
            return;
        }
        if index < self.buffer.len() {
            self.buffer[index] = value;
        }
    }
}
fn process(input: u32) {
    let shared = Arc::new(Mutex::new(Container::new()));
    let mut threads = vec![];
    for i in 0..3 {
        let shared_clone = Arc::clone(&shared);
        let inp = input;
        let handle = thread::spawn(move || {
            let mut guard = shared_clone.lock().unwrap();
            if i == 1 {
                guard.update(2, inp);
            } else {
                guard.update(i, 42);
            }
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }
}
fn main() {
    process(1500);
}
