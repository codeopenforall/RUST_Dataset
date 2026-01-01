use std::sync::{Arc, Mutex};
use std::thread;

struct Data {
    buffer: Vec<u8>,
}

impl Data {
    fn new(size: usize) -> Self {
        // Allocate a vector with required capacity.
        let mut vec = Vec::with_capacity(size);
        // Intentionally initialize only size-1 elements.
        for i in 0..(size - 1) {
            vec.push(i as u8);
        }
        unsafe {
            // FLAW: Off-by-one error – we set the length to 'size', although only size-1 elements have been initialized.
            vec.set_len(size);
        }
        Data { buffer: vec }
    }

    fn sum(&self) -> u64 {
        // Sum over all elements, including the last uninitialized element.
        self.buffer.iter().map(|&x| x as u64).sum()
    }
}

fn main() {
    // Use thread-safe shared ownership.
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