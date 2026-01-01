use std::vec::Vec;
use std::thread;
struct BufferManager {
    buffer: Vec<u32>,
}
impl BufferManager {
    fn new() -> Self {
        BufferManager {
            buffer: vec![1, 1, 1, 1, 1],
        }
    }
    fn update(&mut self, idx: usize, val: u32) -> Result<(), &'static str> {
        if idx >= self.buffer.len() {
            return Err("Index out of bounds");
        }
        self.buffer[idx] = val;
        Ok(())
    }
    fn sum(&self) -> u32 {
        self.buffer.iter().sum()
    }
}
fn main() {
    let mut manager = BufferManager::new();
    let handle = thread::spawn(move || {
        match manager.update(5, 42) {
            Ok(_) => println!("Unexpected update. Buffer state may be corrupted."),
            Err(e) => println!("Error: {}", e),
        }
        println!("Sum: {}", manager.sum());
    });
    handle.join().unwrap();
}
