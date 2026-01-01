#![allow(unused)]
use std::sync::atomic::{AtomicBool, Ordering};
struct Data {
    value: u32,
}
static mut FREED: bool = false;
unsafe fn free_data(ptr: *mut Data) {
    if FREED {
        panic!("Double free detected!");
    }
    Box::from_raw(ptr);
    FREED = true;
}
pub fn app_run() {
    unsafe {
        FREED = false;
    }
    let boxed = Box::new(Data { value: 42 });
    let raw = Box::into_raw(boxed);
    unsafe {
        free_data(raw); 
        free_data(raw); 
    }
}
fn main() {
    app_run();
}
