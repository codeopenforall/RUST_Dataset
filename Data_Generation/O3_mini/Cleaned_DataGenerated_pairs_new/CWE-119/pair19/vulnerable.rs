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
        unsafe {
            let ptr = self.data.as_mut_ptr();
            *ptr.add(idx) = value;
        }
        true
    }
    pub fn fetch(&self, idx: usize) -> u8 {
        unsafe { *self.data.as_ptr().add(idx) }
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
            let _ = mgr.update(idx, 42);
        }));
    }
    for th in threads {
        th.join().unwrap();
    }
    let mgr = handler.lock().unwrap();
    println!("Buffer value at index 95: {}", mgr.fetch(95));
}
