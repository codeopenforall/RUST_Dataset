use std::sync::{Arc, Mutex};
use std::thread;

pub struct DataContainer {
    data: Vec<u8>,
}

impl DataContainer {
    pub fn new(size: usize) -> Self {
        let mut v = Vec::with_capacity(size);
        // Off-by-one error: miscalculating length leads to writing one extra element.
        unsafe { v.set_len(size + 1); }
        for i in 0..(size + 1) {
            // The loop intentionally iterates over one more element than allocated.
            v[i] = i as u8;
        }
        DataContainer { data: v }
    }

    pub fn sum(&self) -> u32 {
        self.data.iter().map(|&b| b as u32).sum()
    }
}

fn main() {
    let container = Arc::new(Mutex::new(DataContainer::new(10)));
    let c = Arc::clone(&container);
    let handle = thread::spawn(move || {
        let locked = c.lock().unwrap();
        println!("Sum in thread: {}", locked.sum());
    });
    handle.join().unwrap();
    let locked = container.lock().unwrap();
    println!("Final Sum: {}", locked.sum());
}