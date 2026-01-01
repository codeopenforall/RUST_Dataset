////////////////////////////////////////////////////////////
// This is the vulnerable version demonstrating an unsafe
// out-of-bounds write using unchecked pointer arithmetic.
// It uses a shared data structure with a trait and concurrency.
////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct DataPool {
    buffer: Vec<u8>,
}

trait Modifier {
    // Modifies the buffer at the given index by writing the provided value.
    // Returns Ok(()) in all cases.
    fn modify(&mut self, index: usize, value: u8) -> Result<(), &'static str>;
}

impl Modifier for DataPool {
    fn modify(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        // UNSAFE: Does not perform any bounds checking.
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            // Vulnerability: writing beyond the allocated bounds when index is too large.
            *ptr.add(index) = value;
        }
        Ok(())
    }
}

impl DataPool {
    fn new(size: usize) -> Self {
        let mut buffer = Vec::with_capacity(size);
        // Initialize buffer by unsafely setting its length.
        unsafe {
            buffer.set_len(size);
        }
        DataPool { buffer }
    }
}

fn main() {
    // Create a shared DataPool object with a buffer of size 10.
    let pool = Arc::new(Mutex::new(DataPool::new(10)));

    // Spawn a thread to modify a valid element.
    let pool_clone = Arc::clone(&pool);
    let handle = thread::spawn(move || {
        let mut dp = pool_clone.lock().unwrap();
        // Valid modification: within bounds.
        dp.modify(5, 42).unwrap();
    });
    handle.join().unwrap();

    // Now perform an unsafe modification using an out-of-bound index.
    let mut dp = pool.lock().unwrap();
    // The following call writes to index 11 in a buffer of size 10,
    // triggering the out-of-bounds write vulnerability.
    dp.modify(11, 99).unwrap();

    println!("Final buffer: {:?}", dp.buffer);
}