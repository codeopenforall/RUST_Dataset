use std::sync::{Mutex, Arc};
use std::thread;
struct Buffer {
    data: Vec<u32>,
}
impl Buffer {
    fn new(size: usize) -> Self {
        let data = vec![0; size];
        Buffer { data }
    }
    fn store_safe(&mut self, index: usize, value: u32) -> Result<(), &'static str> {
        if index < self.data.len() {
            self.data[index] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
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
        if let Err(e) = self.buffer.store_safe(0, 100) {
            eprintln!("Error storing value: {}", e);
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
