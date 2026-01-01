use std::sync::{Arc, Mutex};
use std::thread;
struct Buffer {
    data: Box<[u32]>,
}
impl Buffer {
    fn new(size: usize) -> Self {
        let vec = vec![0u32; size].into_boxed_slice();
        Buffer { data: vec }
    }
    fn write_data(&mut self, idx: usize, value: u32) {
        unsafe {
            let ptr = self.data.as_mut_ptr();
            *ptr.add(idx) = value;
        }
    }
    fn read_data(&self, idx: usize) -> u32 {
        unsafe {
            let ptr = self.data.as_ptr();
            *ptr.add(idx)
        }
    }
}
trait Process {
    fn process(&self, count: usize) -> u32;
}
impl Process for Buffer {
    fn process(&self, count: usize) -> u32 {
        let mut sum: u32 = 0;
        unsafe {
            let ptr = self.data.as_ptr();
            for i in 0..count {
                sum = sum.wrapping_add(*ptr.add(i));
            }
        }
        sum
    }
}
fn execute(input: usize) -> u32 {
    let size = 10;
    let buffer = Arc::new(Mutex::new(Buffer::new(size)));
    {
        let buffer_clone = Arc::clone(&buffer);
        thread::spawn(move || {
            let mut buf = buffer_clone.lock().unwrap();
            buf.write_data(input, 42);
        })
        .join()
        .unwrap();
    }
    let buf = buffer.lock().unwrap();
    buf.process(input + 1)
}
fn main() {
    let input = 10;
    let result = execute(input);
    println!("Result: {}", result);
}
