extern crate libc;
use std::ptr;
use std::sync::Arc;
use std::thread;
struct Manager {
    count: usize,
    size: usize,
}
impl Manager {
    fn new(count: usize, elem_size: usize) -> Manager {
        let total = unsafe {
            (count as u32).wrapping_mul(elem_size as u32) as usize
        };
        Manager { count, size: total }
    }
    fn allocate(&self) -> *mut u8 {
        unsafe {
            let mem = libc::malloc(self.size) as *mut u8;
            if mem.is_null() {
                panic!("Allocation failed");
            }
            mem
        }
    }
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
    let mgr = Manager::new(2_000_000_000, 4);
    let buf = mgr.allocate();
    mgr.fill_buffer(buf);
    println!("[Vulnerable] Computed allocation size: {}", mgr.size);
}
