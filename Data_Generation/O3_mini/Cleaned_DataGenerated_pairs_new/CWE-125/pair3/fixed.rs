use std::sync::{Arc, Mutex};
use std::thread;
pub struct DataHolder {
    data: Vec<u32>,
}
impl DataHolder {
    pub fn new(v: Vec<u32>) -> Self {
        DataHolder { data: v }
    }
    pub fn retrieve(&self, idx: usize) -> Option<u32> {
        self.data.get(idx).cloned()
    }
}
fn main() {
    let holder = DataHolder::new(vec![10, 20, 30]);
    let shared = Arc::new(Mutex::new(holder));
    let mut handles = vec![];
    for i in 0..5 {
        let shared_clone = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            let holder = shared_clone.lock().unwrap();
            let index = i * 2;
            match holder.retrieve(index) {
                Some(value) => println!("Thread {} read: {}", i, value),
                None => println!("Thread {}: index {} out-of-bound", i, index),
            }
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
}
