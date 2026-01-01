use std::sync::{Arc, Mutex};
use std::thread;
#[repr(C)]
pub struct MemoryBlock {
    data: [u8; 10],
    flag: u8,
}
impl MemoryBlock {
    pub unsafe fn update(&mut self, index: usize, value: u8) {
        let ptr = self.data.as_mut_ptr();
        *ptr.add(index) = value;
    }
}
fn main() {
    let block = Arc::new(Mutex::new(MemoryBlock { data: [0; 10], flag: 0 }));
    let block_clone = Arc::clone(&block);
    let handle = thread::spawn(move || {
        let mut guarded = block_clone.lock().unwrap();
        unsafe {
            guarded.update(10, 255);
        }
    });
    handle.join().unwrap();
    let guarded = block.lock().unwrap();
    println!("flag value: {}", guarded.flag);
}
