use std::sync::{Arc, Mutex};
use std::thread;
#[repr(C)]
pub struct MemoryBlock {
    data: [u8; 10],
    flag: u8,
}
impl MemoryBlock {
    pub fn update(&mut self, index: usize, value: u8) {
        if index < self.data.len() {
            unsafe {
                let ptr = self.data.as_mut_ptr();
                *ptr.add(index) = value;
            }
        } else {
        }
    }
}
fn main() {
    let block = Arc::new(Mutex::new(MemoryBlock { data: [0; 10], flag: 0 }));
    let block_clone = Arc::clone(&block);
    let handle = thread::spawn(move || {
        let mut guarded = block_clone.lock().unwrap();
        guarded.update(10, 255);
    });
    handle.join().unwrap();
    let guarded = block.lock().unwrap();
    println!("flag value: {}", guarded.flag);
}
