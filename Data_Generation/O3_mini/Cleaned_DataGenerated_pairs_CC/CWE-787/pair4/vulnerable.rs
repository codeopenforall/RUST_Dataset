use std::sync::{Mutex, Arc};
use std::thread;
struct Buffer {
    data: Vec<u32>,
}
impl Buffer {
    fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        unsafe {
            data.set_len(size);
        }
        Buffer { data }
    }
    unsafe fn store(&mut self, index: usize, value: u32) {
        *self.data.get_unchecked_mut(index) = value;
    }
}
trait Summable {
    fn total(&self) -> u32;
}
struct Worker {
    buffer: Buffer,
}
impl Summable for Worker {
    fn total(&self) -> u32 {
        self.buffer.data.iter().sum()
    }
}
impl Worker {
    fn new(size: usize) -> Self {
        Worker { buffer: Buffer::new(size) }
    }
    fn run(&mut self) {
        unsafe {
            self.buffer.store(self.buffer.data.len(), 100);
        }
    }
}
fn main() {
    let worker = Arc::new(Mutex::new(Worker::new(5)));
    let worker_clone = Arc::clone(&worker);
    let handle = thread::spawn(move || {
        let mut w = worker_clone.lock().unwrap();
        w.run();
    });
    handle.join().unwrap();
    let total = worker.lock().unwrap().total();
    println!("Computed total: {}", total);
}
