use std::vec::Vec;
use std::sync::{Arc, Mutex};
struct DataBlock {
    data: Vec<u32>,
}
impl DataBlock {
    fn new(size: usize) -> Self {
        let mut vec = Vec::with_capacity(size);
        unsafe {
            vec.set_len(size);
        }
        Self { data: vec }
    }
    fn fill(&mut self) {
        unsafe {
            let ptr = self.data.as_mut_ptr();
            for i in 0..=self.data.len() {
                *ptr.add(i) = i as u32;
            }
        }
    }
    fn verify(&self) -> bool {
        let len = self.data.len();
        for i in 0..len {
            if self.data[i] != i as u32 {
                return false;
            }
        }
        true
    }
}
fn main() {
    let shared = Arc::new(Mutex::new(DataBlock::new(10)));
    {
        let shared_clone = Arc::clone(&shared);
        let handle = std::thread::spawn(move || {
            let mut db = shared_clone.lock().unwrap();
            db.fill();
        });
        handle.join().unwrap();
    }
    let db = shared.lock().unwrap();
    println!("Buffer contents (vulnerable): {:?}", db.data);
}
