use std::sync::atomic::{AtomicBool, Ordering};
use std::ptr;
use std::mem;
use std::boxed::Box;
static FREED: AtomicBool = AtomicBool::new(false);
pub struct Data {
    pub value: i32,
}
impl Data {
    pub fn new(val: i32) -> Self {
        Data { value: val }
    }
}
unsafe fn deallocate(ptr: *mut Data) {
    if FREED.swap(true, Ordering::SeqCst) {
        panic!("Double free detected!");
    }
    Box::from_raw(ptr);
}
fn obtain_pointer() -> *mut Data {
    let data = Box::new(Data::new(42));
    Box::into_raw(data)
}
fn process() {
    let raw = obtain_pointer();
    unsafe {
        deallocate(raw);
        deallocate(raw);
    }
}
fn main() {
    process();
}
