use std::sync::Arc;
use std::thread;
struct Buffer {
    data: Vec<u8>,
    size: usize, 
}
unsafe impl Send for Buffer {}
unsafe impl Sync for Buffer {}
impl Buffer {
    fn new(size: usize) -> Self {
        let data = vec![0u8; size + 1];
        Buffer { data, size }
    }
    unsafe fn fill(&self, value: u8) {
        let ptr = self.data.as_ptr() as *mut u8;
        for i in 0..self.size {
            *ptr.add(i) = value;
        }
    }
    fn get(&self, index: usize) -> u8 {
        self.data[index]
    }
}
fn main() {
    let buffer = Arc::new(Buffer::new(10));
    let buffer_clone = Arc::clone(&buffer);
    let handle = thread::spawn(move || {
        unsafe {
            buffer_clone.fill(0xAA);
        }
    });
    unsafe {
        buffer.fill(0xAA);
    }
    handle.join().unwrap();
    println!("Finished operation");
}
