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
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            *ptr.add(idx) = val;          
            self.buffer.set_len(idx + 1); 
        }
        Ok(())
    }
    fn sum(&self) -> u32 {
        self.buffer.iter().sum()
    }
}
fn main() {
    let mut manager = BufferManager::new();
    let handle = thread::spawn(move || {
        let _ = manager.update(5, 42);
        println!("Sum: {}", manager.sum());
    });
    handle.join().unwrap();
}
