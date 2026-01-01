///////////////////////////////////////////////////////////////
// This Rust code contains an integer overflow vulnerability.
// It computes a required allocation size by multiplying two u32
// values in an unsafe block without checking for overflow.
// When count * element_size exceeds u32::MAX, the multiplication
// is performed in u32 arithmetic, causing a wraparound and a
// miscalculated allocation size. This can lead to writing beyond
// the allocated buffer.
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

    // Vulnerable computation: multiplication is done in u32 arithmetic.
    // If (count * element_size) exceeds u32::MAX, it wraps around.
    fn compute_size(&self) -> usize {
        // WARNING: The multiplication happens in u32 and is then cast to usize.
        // This does not detect overflow.
        (self.count.wrapping_mul(self.element_size)) as usize
    }

    fn process(&self) {
        let size = self.compute_size();
        // Create a layout using the (possibly wrapped) size.
        let layout = unsafe { Layout::from_size_align_unchecked(size, 8) };
        unsafe {
            // Allocate zeroed memory for size bytes.
            let ptr = alloc_zeroed(layout);
            // Write to every byte of the allocated buffer.
            // The intended semantics is to fill count*element_size bytes
            // but due to overflow, this loop may be far shorter than expected.
            for i in 0..size {
                *ptr.add(i) = 1;
            }
            dealloc(ptr, layout);
        }
        println!("Processing complete. Computed size: {}", size);
    }
}

fn main() {
    // The parameters below are chosen such that:
    // safe multiplication (u64) would yield: 2_000_000_000 * 3 = 6_000_000_000,
    // but computing in u32 will wrap modulo 2^32.
    let handler = Arc::new(Handler::new(2_000_000_000, 3));
    let flag = Arc::new(Mutex::new(false));

    let mut handles = Vec::new();
    for _ in 0..4 {
        let h = Arc::clone(&handler);
        let f = Arc::clone(&flag);
        let t = thread::spawn(move || {
            h.process();
            let mut done = f.lock().unwrap();
            *done = true;
        });
        handles.push(t);
    }

    for t in handles {
        t.join().unwrap();
    }
    assert!(*flag.lock().unwrap());
    println!("Main finished execution.");
}