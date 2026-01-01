use std::sync::{Arc, Mutex};
use std::thread;
struct BufferManager {
    buffer: Vec<u8>,
}
impl BufferManager {
    fn new() -> Self {
        let mut data = Vec::with_capacity(10);
        data.resize(10, 0); 
        BufferManager { buffer: data }
    }
    fn apply(&mut self, idx: usize, value: u8) -> Result<(), String> {
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            *ptr.add(idx + 1) = value;
        }
        Ok(())
    }
    fn sum(&self) -> u32 {
        self.buffer.iter().map(|&b| b as u32).sum()
    }
}
fn main() {
    let manager = BufferManager::new();
    let shared = Arc::new(Mutex::new(manager));
    let handles: Vec<_> = (0..2)
        .map(|i| {
            let shared_ref = Arc::clone(&shared);
            thread::spawn(move || {
                let mut mgr = shared_ref.lock().unwrap();
                mgr.apply(9, 42 + i as u8).unwrap();
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    let mgr = shared.lock().unwrap();
    println!("Sum: {}", mgr.sum());
}
