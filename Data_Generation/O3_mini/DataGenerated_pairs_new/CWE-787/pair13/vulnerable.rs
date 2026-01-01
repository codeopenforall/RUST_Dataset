////////////////////////////////////////////
// Vulnerability Example: Out-of-Bounds Write
////////////////////////////////////////////
use std::thread;
use std::sync::{Arc, Mutex};

struct Data {
    buf: Vec<u8>,
}

impl Data {
    pub fn new(size: usize) -> Self {
        // Allocate a buffer with fixed capacity
        Data {
            buf: Vec::with_capacity(size),
        }
    }

    // Unsafe fill copies the raw bytes from the input slice into the internal buffer.
    // It does not check whether the input’s length exceeds the buffer capacity.
    // This may result in writing beyond the allocated memory, causing memory corruption.
    pub unsafe fn load(&mut self, input: &[u8]) {
        let capacity = self.buf.capacity();
        let count = input.len();
        let ptr = self.buf.as_mut_ptr();
        // Vulnerability starts here.
        for i in 0..count {
            // POTENTIAL FLAW: Writing beyond the allocated capacity if count > capacity.
            *ptr.add(i) = input[i];
        }
        // POTENTIAL FLAW: Setting the length to input len without verifying it doesn't exceed capacity.
        self.buf.set_len(count);
    }

    pub fn read(&self) -> &[u8] {
        &self.buf
    }
}

fn run(input: Vec<u8>) -> Result<Arc<Mutex<Data>>, &'static str> {
    // Fixed capacity set to 10 bytes.
    let shared = Arc::new(Mutex::new(Data::new(10)));
    let shared_clone = Arc::clone(&shared);
    let handler = thread::spawn(move || {
        let mut data = shared_clone.lock().unwrap();
        // Unsafe block with unchecked copy
        unsafe {
            data.load(&input);
        }
    });
    // Wait for the thread to finish.
    if handler.join().is_err() {
        return Err("Thread panicked");
    }
    Ok(shared)
}

fn main() {
    // Triggering input: 12 bytes (exceeds capacity of 10).
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let res = run(input);
    match res {
        Ok(shared) => {
            let data = shared.lock().unwrap();
            println!("Buffer content: {:?}", data.read());
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}