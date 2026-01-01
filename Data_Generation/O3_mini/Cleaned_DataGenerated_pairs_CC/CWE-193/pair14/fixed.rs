use std::sync::{Arc, Mutex};
use std::thread;
struct Data {
    buffer: Vec<u8>,
}
impl Data {
    fn new(size: usize) -> Self {
        let mut vec = Vec::with_capacity(size);
        for i in 0..size {
            vec.push(i as u8);
        }
        Data { buffer: vec }
    }
    fn sum(&self) -> u64 {
        self.buffer.iter().map(|&x| x as u64).sum()
    }
}
fn main() {
    let shared = Arc::new(Mutex::new(Data::new(10)));
    let mut threads = vec![];
    for _ in 0..4 {
        let clone = shared.clone();
        threads.push(thread::spawn(move || {
            let guard = clone.lock().unwrap();
            println!("Total: {}", guard.sum());
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
}
