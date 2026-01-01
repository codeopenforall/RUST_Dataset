////////////////////////////////////////////
// Patched Example: Preventing Out-of-Bounds Write
////////////////////////////////////////////
use std::thread;
use std::sync::{Arc, Mutex};

struct Data {
    buf: Vec<u8>,
}

impl Data {
    pub fn new(size: usize) -> Self {
        // Allocate buffer with fixed capacity.
        Data {
            buf: Vec::with_capacity(size),
        }
    }

    // The safe load method checks the input length against the buffer capacity.
    // If the input length exceeds the capacity, it returns an error rather than writing out of bounds.
    pub fn load(&mut self, input: &[u8]) -> Result<(), &'static str> {
        let capacity = self.buf.capacity();
        let count = input.len();
        if count > capacity {
            // Prevent out-of-bound access by erroring out.
            return Err("Input size exceeds allocated buffer capacity");
        }
        let ptr = self.buf.as_mut_ptr();
        // Copy input bytes into the buffer within bounds.
        for i in 0..count {
            // Safe because count <= capacity.
            unsafe { *ptr.add(i) = input[i]; }
        }
        // Now safely update the vector length.
        unsafe {
            self.buf.set_len(count);
        }
        Ok(())
    }

    pub fn read(&self) -> &[u8] {
        &self.buf
    }
}

fn run(input: Vec<u8>) -> Result<Arc<Mutex<Data>>, &'static str> {
    let shared = Arc::new(Mutex::new(Data::new(10))); // Buffer capacity is 10.
    let shared_clone = Arc::clone(&shared);
    let handler = thread::spawn(move || {
        let mut data = shared_clone.lock().unwrap();
        // Use the safe load method and panic if it fails.
        if let Err(e) = data.load(&input) {
            panic!("{}", e);
        }
    });
    handler.join().map_err(|_| "Thread panicked")?;
    Ok(shared)
}

fn main() {
    // Triggering input: 12 bytes (exceeds capacity) to test the guard.
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    match run(input) {
        Ok(shared) => {
            let data = shared.lock().unwrap();
            println!("Buffer content: {:?}", data.read());
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}