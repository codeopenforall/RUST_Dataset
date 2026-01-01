use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;
use std::sync::Arc;
use std::thread;
struct Data {
    raw: *mut u8,
    cap: usize,
}
impl Data {
    unsafe fn create(s: &str) -> Self {
        let cap = mem::size_of_val(&s) + 1;
        let layout = Layout::from_size_align(cap, 1).unwrap();
        let raw = alloc(layout);
        ptr::copy_nonoverlapping(s.as_ptr(), raw, cap);
        Data { raw, cap }
    }
    unsafe fn as_text(&self) -> &str {
        let len = self.cap - 1;
        let slice = std::slice::from_raw_parts(self.raw, len);
        std::str::from_utf8(slice).unwrap()
    }
}
impl Drop for Data {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align(self.cap, 1).unwrap();
            dealloc(self.raw, layout);
        }
    }
}
fn main() {
    let input = "Hello, world!";
    let shared = Arc::new(input.to_string());
    let mut threads = vec![];
    for _ in 0..4 {
        let data_ref = Arc::clone(&shared);
        threads.push(thread::spawn(move || {
            unsafe {
                let item = Data::create(&data_ref);
                assert_eq!(item.as_text(), "Hello, world!", "Mismatch in buffer content");
            }
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
}
