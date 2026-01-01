///////////////////////////////////////////////
// Vulnerable code for off-by-one error CWE-193 //
///////////////////////////////////////////////
use std::vec::Vec;
use std::sync::{Arc, Mutex};

struct DataBlock {
    data: Vec<u32>,
}

impl DataBlock {
    fn new(size: usize) -> Self {
        // Allocate vector with capacity equal to size.
        let mut vec = Vec::with_capacity(size);
        // Unsafe: Pretend the vector is fully initialized.
        unsafe {
            // Set length exactly to size.
            vec.set_len(size);
        }
        Self { data: vec }
    }

    fn fill(&mut self) {
        unsafe {
            // Get raw pointer to the underlying data.
            let ptr = self.data.as_mut_ptr();
            // Off-by-one: Loop iterates from 0 up to and including len, causing one extra write.
            for i in 0..=self.data.len() {
                // Writing i into each element.
                *ptr.add(i) = i as u32;
            }
        }
    }

    fn verify(&self) -> bool {
        // Check that each element holds its index value.
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
    // Use an Arc<Mutex<>> to mimic concurrent access patterns.
    let shared = Arc::new(Mutex::new(DataBlock::new(10)));
    {
        let shared_clone = Arc::clone(&shared);
        let handle = std::thread::spawn(move || {
            let mut db = shared_clone.lock().unwrap();
            // Fill the data block unsafely (contains off-by-one vulnerability).
            db.fill();
        });
        handle.join().unwrap();
    }
    let db = shared.lock().unwrap();
    println!("Buffer contents (vulnerable): {:?}", db.data);
}