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
    fn compute_size(&self) -> usize {
        (self.count.wrapping_mul(self.element_size)) as usize
    }
    fn process(&self) {
        let size = self.compute_size();
        let layout = unsafe { Layout::from_size_align_unchecked(size, 8) };
        unsafe {
            let ptr = alloc_zeroed(layout);
            for i in 0..size {
                *ptr.add(i) = 1;
            }
            dealloc(ptr, layout);
        }
        println!("Processing complete. Computed size: {}", size);
    }
}
fn main() {
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
