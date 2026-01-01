#![allow(unused)]
use std::ptr;
struct Data {
    value: u32,
}
struct Owner {
    ptr: *mut Data,
}
impl Owner {
    fn new(val: u32) -> Self {
        let boxed = Box::new(Data { value: val });
        Owner { ptr: Box::into_raw(boxed) }
    }
    unsafe fn release(&mut self) {
        if !self.ptr.is_null() {
            Box::from_raw(self.ptr);
            self.ptr = ptr::null_mut();
        }
    }
}
impl Drop for Owner {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                Box::from_raw(self.ptr);
                self.ptr = ptr::null_mut();
            }
        }
    }
}
pub fn app_run() {
    let mut owner = Owner::new(42);
    unsafe {
        owner.release();
    }
}
fn main() {
    app_run();
}
