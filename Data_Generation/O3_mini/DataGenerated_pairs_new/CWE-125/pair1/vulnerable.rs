//////////////////////////////
// Vulnerability demonstration code
//////////////////////////////
use std::sync::Arc;
use std::thread;

struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    // Performs an unsafe read by subtracting 1 from the computed pointer.
    // If called with index 0, the pointer arithmetic underflows causing an out‐of‐bounds read.
    fn read_value(&self, index: usize) -> u32 {
        // Vulnerability: no bounds-checking; subtracts 1 unconditionally.
        unsafe {
            let ptr = self.data.as_ptr().offset(index as isize - 1);
            *ptr
        }
    }
}

fn main() {
    let shared = Arc::new(Buffer { data: vec![10, 20, 30] });
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        // Trigger the potential out-of-bounds read by using index 0.
        let result = shared_clone.read_value(0);
        println!("Result: {}", result);
    });
    handle.join().unwrap();
}