///////////////////////////////////////////////
// Fixed Version - CWE-190 Integer Overflow Mitigation
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
    // Corrected: use checked multiplication in usize to detect overflow.
    fn new(count: usize, elem_size: usize) -> Manager {
        // Using checked_mul ensures that if the multiplication overflows,
        // the program will panic rather than silently produce a wrong value.
        let total = count.checked_mul(elem_size).expect("Integer overflow detected");
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
        for i in 0..4 {
            let arc_buf = Arc::clone(&shared_buf);
            handles.push(thread::spawn(move || {
                unsafe {
                    for j in 0..1000 {
                        let offset = i * 1000 + j;
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
    // The same input; however, the multiplication is safely performed.
    let mgr = Manager::new(2_000_000_000, 4);
    let buf = mgr.allocate();
    mgr.fill_buffer(buf);
    println!("[Fixed] Computed allocation size: {}", mgr.size);
}