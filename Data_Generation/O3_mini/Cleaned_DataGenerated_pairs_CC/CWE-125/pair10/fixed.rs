use std::env;
use std::sync::Arc;
use std::thread;
struct Buffer {
    data: Vec<u8>,
}
impl Buffer {
    pub fn new(data: Vec<u8>) -> Self {
        Buffer { data }
    }
    pub fn fetch(&self, idx: usize) -> u8 {
        if idx == 0 || idx > self.data.len() {
            panic!("Index out of bounds");
        }
        self.data[idx - 1]
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let index: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };
    let shared = Arc::new(Buffer::new(vec![10, 20, 30, 40]));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let val = shared_clone.fetch(index);
        println!("Thread Fetched value: {}", val);
    });
    handle.join().unwrap();
}
