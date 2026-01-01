use std::sync::{Arc, Mutex};
use std::thread;
struct DataBuffer {
    data: Vec<u8>,
}
impl DataBuffer {
    fn new(size: usize) -> Self {
        DataBuffer { data: vec![0; size] }
    }
    fn process(&mut self, index: usize, value: u8) -> u32 {
        unsafe {
            let ptr = self.data.as_mut_ptr();
            *ptr.add(index) = value;
            self.data.set_len(self.data.len() + 1);
        }
        self.data.iter().map(|&v| v as u32).sum()
    }
}
fn main() {
    let shared = Arc::new(Mutex::new(DataBuffer::new(10)));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let mut buf = shared_clone.lock().unwrap();
        let res = buf.process(10, 42);
        println!("Thread result: {}", res);
    });
    {
        let mut buf = shared.lock().unwrap();
        let res = buf.process(5, 13);
        println!("Main thread result: {}", res);
    }
    handle.join().unwrap();
}
