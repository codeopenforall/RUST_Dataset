use std::sync::atomic::{AtomicBool, Ordering};
use std::panic;
static FREED: AtomicBool = AtomicBool::new(false);
struct Data {
    ptr: *mut i32,
}
impl Data {
    fn new(val: i32) -> Self {
        let b = Box::new(val);
        Self { ptr: Box::into_raw(b) }
    }
}
impl Clone for Data {
    fn clone(&self) -> Self {
        Data { ptr: self.ptr }
    }
}
impl Drop for Data {
    fn drop(&mut self) {
        if FREED.swap(true, Ordering::SeqCst) {
            panic!("double free detected");
        }
        unsafe {
            let _ = Box::from_raw(self.ptr);
        }
    }
}
fn execute() {
    let original = Data::new(42);
    let duplicate = original.clone();
}
fn main() {
    let result = panic::catch_unwind(|| {
        execute();
    });
    if result.is_err() {
        eprintln!("Error encountered during execution.");
    }
}
