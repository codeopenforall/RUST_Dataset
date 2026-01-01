//////////////////////////////
// Corrected demonstration code
//////////////////////////////
use std::sync::Arc;
use std::thread;

struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    // Safely reads a value by ensuring that the computed pointer does not underflow.
    // For index 0, or any index that would be out-of-bounds, it returns a safe default value.
    fn read_value(&self, index: usize) -> u32 {
        if index == 0 {
            // Prevent underflow: do not perform pointer arithmetic that results in a negative offset.
            0
        } else if index - 1 < self.data.len() {
            unsafe {
                let ptr = self.data.as_ptr().offset((index as isize) - 1);
                *ptr
            }
        } else {
            // For any index beyond the permissible range, return a safe default value.
            0
        }
    }
}

fn main() {
    let shared = Arc::new(Buffer { data: vec![10, 20, 30] });
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let result = shared_clone.read_value(0);
        println!("Result: {}", result);
    });
    handle.join().unwrap();
}