#![allow(dead_code)]
use std::thread;
trait Action {
    fn execute(&self) -> i32;
}
struct Holder {
    ptr: *const i32,
}
impl Holder {
    fn new(value: i32) -> Self {
        let b = Box::new(value);
        let ptr = Box::into_raw(b);
        Holder { ptr }
    }
    unsafe fn deallocate(&mut self) {
        let _ = Box::from_raw(self.ptr as *mut i32);
    }
    unsafe fn read(&self) -> i32 {
        *self.ptr
    }
}
impl Action for Holder {
    fn execute(&self) -> i32 {
        unsafe { self.read() }
    }
}
fn process(holder: &Holder) -> i32 {
    holder.execute()
}
fn main() {
    let mut resource = Holder::new(12345);
    let handle = thread::spawn(|| {
        let dummy = Box::new(99999);
        drop(dummy);
    });
    handle.join().unwrap();
    unsafe {
        resource.deallocate();
        let res = process(&resource);
        println!("Result: {}", res);
    }
}
