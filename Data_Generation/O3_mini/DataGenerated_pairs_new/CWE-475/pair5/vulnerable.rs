use std::sync::{Arc, Mutex};
use std::thread;

struct DataHolder {
    ptr: *mut u8,
    size: usize,
}

impl DataHolder {
    fn new(size: usize) -> Self {
        // Allocate a vector with exactly 'size' bytes.
        let mut buf = Vec::with_capacity(size);
        buf.resize(size, 0);
        let ptr = buf.as_mut_ptr();
        // Prevent deallocation by forgetting the vector.
        std::mem::forget(buf);
        DataHolder { ptr, size }
    }

    // This unsafe function converts the allocated memory back into a Vec
    // to correctly free it.
    unsafe fn release(self) {
        let _ = Vec::from_raw_parts(self.ptr, self.size, self.size);
    }

    // Unsafe function to fill the allocated memory from a slice.
    // BUG: Violates API precondition by copying data.len()+1 bytes into
    // a buffer that was allocated with only data.len() bytes.
    unsafe fn populate(&mut self, data: &[u8]) {
        // The third argument is deliberately off by one causing undefined behavior.
        std::ptr::copy_nonoverlapping(data.as_ptr(), self.ptr, data.len() + 1);
    }
}

fn main() {
    let input = vec![10, 20, 30, 40];
    // Allocate exactly as many bytes as in the input.
    let holder = Arc::new(Mutex::new(DataHolder::new(input.len())));
    let holder_clone = Arc::clone(&holder);
    let input_clone = input.clone();

    // Spawn a thread to populate memory concurrently.
    let handle = thread::spawn(move || {
        let mut dh = holder_clone.lock().unwrap();
        unsafe {
            dh.populate(&input_clone);
        }
    });

    handle.join().unwrap();

    {
        let dh = holder.lock().unwrap();
        unsafe {
            // Read from the allocated memory.
            // This may result in undefined behavior due to the previous memory misuse.
            let slice = std::slice::from_raw_parts(dh.ptr, input.len());
            println!("Data: {:?}", slice);
        }
    }

    // Release allocated memory.
    unsafe {
        // To safely free, we take ownership by reading the DataHolder.
        let temp = {
            let dh = holder.lock().unwrap();
            // Create a duplicate in a temporary variable to call release.
            DataHolder { ptr: dh.ptr, size: dh.size }
        };
        temp.release();
    }
}