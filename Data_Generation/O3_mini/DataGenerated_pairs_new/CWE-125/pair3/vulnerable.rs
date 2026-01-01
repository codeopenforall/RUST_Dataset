use std::sync::{Arc, Mutex};
use std::thread;

pub struct DataHolder {
    data: Vec<u32>,
}

impl DataHolder {
    pub fn new(v: Vec<u32>) -> Self {
        DataHolder { data: v }
    }

    // This function performs an unsafe retrieval without bounds checking.
    // It uses pointer arithmetic with as_ptr().add(idx) to read an element
    // without verifying that idx is within the valid range of the vector.
    // If idx is out of bounds, this may read arbitrary memory.
    pub fn retrieve(&self, idx: usize) -> Option<u32> {
        unsafe { Some(*self.data.as_ptr().add(idx)) }
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
            let index = i * 2; // For i>=2, index will be out of bounds.
            let result = holder.retrieve(index);
            println!("Thread {} read: {:?}", i, result);
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
}