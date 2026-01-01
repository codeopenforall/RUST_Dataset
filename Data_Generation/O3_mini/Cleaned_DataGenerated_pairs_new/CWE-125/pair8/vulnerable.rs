use std::sync::Arc;
use std::thread;
struct Buffer {
    data: Vec<u32>,
}
impl Buffer {
    fn new(n: usize) -> Self {
        let mut data = Vec::with_capacity(n);
        for i in 0..n {
            data.push(i as u32);
        }
        Buffer { data }
    }
    fn unsafe_read(&self, idx: usize) -> u32 {
        unsafe { *self.data.get_unchecked(idx) }
    }
    pub fn read_value_public(&self, idx: usize) -> Option<u32> {
        Some(unsafe { *self.data.get_unchecked(idx) })
    }
}
fn main() {
    let buffer = Arc::new(Buffer::new(10));
    let mut handles = vec![];
    for i in 0..5 {
        let buf_clone = Arc::clone(&buffer);
        handles.push(thread::spawn(move || {
            let index = if i == 2 { 12 } else { i };
            buf_clone.unsafe_read(index)
        }));
    }
    for handle in handles {
        let res = handle.join().unwrap();
        println!("Output: {}", res);
    }
}
