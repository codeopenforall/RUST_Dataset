#![allow(unused)]
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};
static FREE_COUNT: AtomicUsize = AtomicUsize::new(0);
struct Resource {
    ptr: *mut i32,
}
impl Resource {
    unsafe fn new(val: i32) -> Self {
        let b = Box::new(val);
        Self { ptr: Box::into_raw(b) }
    }
    unsafe fn manual_drop(&mut self) {
        if !self.ptr.is_null() {
            if FREE_COUNT.fetch_add(1, Ordering::SeqCst) >= 1 {
                panic!("Double free detected");
            }
            let _ = Box::from_raw(self.ptr);
        }
    }
}
impl Drop for Resource {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                if FREE_COUNT.fetch_add(1, Ordering::SeqCst) >= 1 {
                    panic!("Double free detected");
                }
                let _ = Box::from_raw(self.ptr);
            }
        }
    }
}
fn main() {
    unsafe {
        let mut res = Resource::new(100);
        res.manual_drop();
    }
}
