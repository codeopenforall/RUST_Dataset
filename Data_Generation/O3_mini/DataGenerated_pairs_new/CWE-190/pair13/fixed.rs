///////////////////////////////////////////////////////////////
// The revised code fixes the integer overflow vulnerability by using
// checked multiplication on u32 values and converting the result to usize.
// If the multiplication overflows, the program will panic rather than miscalculate
// the allocation size. This ensures that the allocated buffer matches the true
// intended size, preventing out-of-bounds memory writes.
///////////////////////////////////////////////////////////////
use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::sync::{Arc, Mutex};
use std::thread;

struct Handler {
    count: u32,
    element_size: u32,
}

impl Handler {
    fn new(count: u32, element_size: u32) -> Self {
        Handler { count, element_size }
    }

    // Corrected computation: Uses checked multiplication to ensure no overflow occurs.
    fn compute_size(&self) -> usize {
        // Perform multiplication in u64 then confirm it fits in u32, if intended.
        let product = self.count.checked_mul(self.element_size)
            .expect("Integer multiplication overflow detected");
        product as usize
    }

    fn process(&self) {
        let size = self.compute_size();
        // Create a layout using the correct size.
        let layout = Layout::from_size_align(size, 8).expect("Invalid layout");
        unsafe {
            let ptr = alloc_zeroed(layout);
            // Write to every byte of the allocated buffer.
            for i in 0..size {
                *ptr.add(i) = 1;
            }
            dealloc(ptr, layout);
        }
        println!("Processing complete. Computed size: {}", size);
    }
}

fn main() {
    // The parameters below are the same as in the vulnerable version.
    // Now, if an overflow would occur, checked_mul will detect it and panic.
    let handler = Arc::new(Handler::new(2_000_000_000, 3));
    let flag = Arc::new(Mutex::new(false));

    let mut handles = Vec::new();
    for _ in 0..4 {
        let h = Arc::clone(&handler);
        let f = Arc::clone(&flag);
        let t = thread::spawn(move || {
            // In this corrected code, process() will panic if overflow is detected,
            // ensuring that no unsafe allocation is performed.
            h.process();
            let mut done = f.lock().unwrap();
            *done = true;
        });
        handles.push(t);
    }

    for t in handles {
        t.join().expect("Thread failed");
    }
    assert!(*flag.lock().unwrap());
    println!("Main finished execution.");
}