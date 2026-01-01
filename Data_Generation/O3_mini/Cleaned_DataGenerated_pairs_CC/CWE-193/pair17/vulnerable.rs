use std::sync::{Arc, Mutex};
use std::thread;
struct Buffer {
    data: Vec<u32>,
}
impl Buffer {
    fn new(size: usize) -> Buffer {
        Buffer {
            data: Vec::with_capacity(size),
        }
    }
    fn fill(&mut self) {
        let size = self.data.capacity();
        unsafe {
            self.data.set_len(size);
            let ptr = self.data.as_mut_ptr();
            for i in 0..=size { 
                *ptr.add(i) = i as u32;
            }
        }
    }
    fn sum(&self) -> u32 {
        self.data.iter().sum()
    }
}
pub fn process() -> u32 {
    let mut buffer = Buffer::new(10);
    buffer.fill();
    buffer.sum()
}
fn main() {
    let result = process();
    println!("Result: {}", result);
    let shared = Arc::new(Mutex::new(Buffer {
        data: Vec::with_capacity(10),
    }));
    {
        let mut temp = shared.lock().unwrap();
        temp.fill();
    }
    let mut handles = Vec::new();
    for _ in 0..5 {
        let shared_clone = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            let lock = shared_clone.lock().unwrap();
            lock.sum()
        }));
    }
    for handle in handles {
        let res = handle.join().unwrap();
        println!("Thread result: {}", res);
    }
}
