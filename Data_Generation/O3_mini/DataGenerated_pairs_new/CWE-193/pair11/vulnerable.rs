//////////////////////////////////////////////
// Vulnerability: Off-by-One in Vector Length //
//////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

struct DataManager {
    data: Arc<Mutex<Vec<u8>>>,
}

impl DataManager {
    // Creates a vector with a pre-allocated capacity.
    fn new(capacity: usize) -> Self {
        let mut v = Vec::with_capacity(capacity);
        // Unsafely set the length so that the memory is “accessible.”
        // (In real code this is inherently dangerous but used here to mimic low‐level allocation.)
        unsafe {
            v.set_len(capacity);
        }
        DataManager {
            data: Arc::new(Mutex::new(v)),
        }
    }

    // Copies input bytes into the allocated buffer.
    // Vulnerability: Off-by-one error in the length passed to `set_len`
    // causes an extra, uninitialized byte to be included.
    fn process(&self, input: &[u8]) {
        let mut lock = self.data.lock().unwrap();
        if input.len() > lock.capacity() {
            panic!("Input too large");
        }
        unsafe {
            ptr::copy_nonoverlapping(input.as_ptr(), lock.as_mut_ptr(), input.len());
            // Off-by-one: sets length to input.len() + 1, adding an extra invalid byte.
            lock.set_len(input.len() + 1);
        }
    }

    // Returns a clone of the inner vector.
    fn fetch(&self) -> Vec<u8> {
        let lock = self.data.lock().unwrap();
        lock.clone()
    }
}

pub fn main() {
    let manager = DataManager::new(10);
    let manager_clone = manager.data.clone();

    // Spawn a concurrent reader to mimic real-world concurrent access.
    let reader = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(50));
        let data = manager_clone.lock().unwrap();
        println!("Concurrent read sees: {:?}", &*data);
    });

    let src = vec![1, 2, 3, 4, 5];
    manager.process(&src);

    reader.join().unwrap();
    println!("Operation complete, vector length: {}", manager.fetch().len());
}