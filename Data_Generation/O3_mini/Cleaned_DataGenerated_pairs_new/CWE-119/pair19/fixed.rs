use std::sync::{Arc, Mutex};
use std::thread;
pub struct MemoryHandler {
    data: Vec<u8>,
}
impl MemoryHandler {
    pub fn new() -> Self {
        MemoryHandler { data: vec![0u8; 100] }
    }
    pub fn update(&mut self, idx: usize, value: u8) -> bool {
        if idx < self.data.len() {
            unsafe {
                *self.data.get_unchecked_mut(idx) = value;
            }
            true
        } else {
            false
        }
    }
    pub fn fetch(&self, idx: usize) -> u8 {
        if idx < self.data.len() {
            unsafe { *self.data.get_unchecked(idx) }
        } else {
            0
        }
    }
}
fn main() {
    let handler = Arc::new(Mutex::new(MemoryHandler::new()));
    let mut threads = vec![];
    for i in 0..10 {
        let h = Arc::clone(&handler);
        threads.push(thread::spawn(move || {
            let mut mgr = h.lock().unwrap();
            let idx = 95 + i;
            let result = mgr.update(idx, 42);
            if !result {
                eprintln!("Attempted to update invalid index: {}", idx);
            }
        }));
    }
    for th in threads {
        th.join().unwrap();
    }
    let mgr = handler.lock().unwrap();
    println!("Buffer value at index 95: {}", mgr.fetch(95));
}
