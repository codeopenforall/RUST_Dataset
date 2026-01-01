///////////////////////////////////////////////
// Vulnerable Version - CWE-190 Integer Overflow
///////////////////////////////////////////////
extern crate libc;
use std::ptr;
use std::sync::Arc;
use std::thread;

struct Manager {
    count: usize,
    size: usize,
}

impl Manager {
    // Vulnerability: using unsafe conversion to u32 and wrapping multiplication
    fn new(count: usize, elem_size: usize) -> Manager {
        // The following unsafe block converts the operands to u32,
        // causing a wraparound when the expected product (in usize) exceeds u32::MAX.
        let total = unsafe {
            (count as u32).wrapping_mul(elem_size as u32) as usize
        };
        Manager { count, size: total }
    }

    // Allocate a buffer using libc malloc.
    fn allocate(&self) -> *mut u8 {
        unsafe {
            let mem = libc::malloc(self.size) as *mut u8;
            if mem.is_null() {
                panic!("Allocation failed");
            }
            mem
        }
    }

    // Fill the allocated buffer concurrently.
    fn fill_buffer(&self, buf: *mut u8) {
        let shared_buf = Arc::new(buf);
        let mut handles = vec![];
        // Spawn threads to simulate concurrent writes.
        for i in 0..4 {
            let arc_buf = Arc::clone(&shared_buf);
            handles.push(thread::spawn(move || {
                unsafe {
                    // Each thread writes 1000 bytes.
                    for j in 0..1000 {
                        // Calculate offset index; this loop does not check the allocation's true bounds.
                        let offset = i * 1000 + j;
                        // Write a dummy value.
                        ptr::write(arc_buf.add(offset), 0xAAu8);
                    }
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
    }
}

fn main() {
    // This input causes multiplication overflow when computed in u32.
    let mgr = Manager::new(2_000_000_000, 4);
    let buf = mgr.allocate();
    mgr.fill_buffer(buf);
    println!("[Vulnerable] Computed allocation size: {}", mgr.size);
}