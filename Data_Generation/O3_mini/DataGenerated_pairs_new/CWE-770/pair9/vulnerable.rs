//////////////////////////////////////////
// Vulnerable Rust Code Sample
//////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Allocator {
    data: Arc<Mutex<Vec<u8>>>,
}

impl Allocator {
    fn new() -> Self {
        Allocator {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }
    // This function accepts a number of bytes to allocate.
    // It does not enforce any upper-bound checks.
    // It spawns a thread and uses unsafe pointer arithmetic to write into a Vec.
    fn allocate_with_limit(&self, n: usize) -> Result<(), &'static str> {
        let data_arc = self.data.clone();
        thread::spawn(move || {
            unsafe {
                let mut vec = data_arc.lock().unwrap();
                let len = vec.len();
                // Unbounded reservation and unsafe write without checking maximum allowable allocation.
                vec.reserve(n);
                for i in 0..n {
                    let ptr = vec.as_mut_ptr().add(len + i);
                    // Write value 1 into the newly allocated memory.
                    std::ptr::write(ptr, 1u8);
                    // Manually update the vector length.
                    vec.set_len(len + i + 1);
                }
            }
        }).join().unwrap();
        Ok(())
    }
}

fn main() {
    let alloc = Allocator::new();
    // Here the input value is within an expected range,
    // but no protection is applied if a larger input is provided.
    match alloc.allocate_with_limit(50_000) {
        Ok(_) => println!("Allocation completed"),
        Err(e) => println!("Allocation error: {}", e),
    }
}