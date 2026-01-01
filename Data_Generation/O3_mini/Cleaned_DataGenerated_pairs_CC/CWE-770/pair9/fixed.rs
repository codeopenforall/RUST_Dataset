use std::sync::{Arc, Mutex};
use std::thread;
const MAX_ALLOC: usize = 100_000; 
struct Allocator {
    data: Arc<Mutex<Vec<u8>>>,
}
impl Allocator {
    fn new() -> Self {
        Allocator {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }
    fn allocate_with_limit(&self, n: usize) -> Result<(), &'static str> {
        if n > MAX_ALLOC {
            return Err("Request exceeds maximum allowed allocation size");
        }
        let data_arc = self.data.clone();
        thread::spawn(move || {
            unsafe {
                let mut vec = data_arc.lock().unwrap();
                let len = vec.len();
                vec.reserve(n);
                for i in 0..n {
                    let ptr = vec.as_mut_ptr().add(len + i);
                    std::ptr::write(ptr, 1u8);
                    vec.set_len(len + i + 1);
                }
            }
        }).join().unwrap();
        Ok(())
    }
}
fn main() {
    let alloc = Allocator::new();
    match alloc.allocate_with_limit(50_000) {
        Ok(_) => println!("Allocation completed"),
        Err(e) => println!("Allocation error: {}", e),
    }
}
