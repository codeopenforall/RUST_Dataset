#![allow(dead_code)]
use std::ptr;
use std::sync::Once;
use std::thread;
static mut GLOBAL_RESOURCE: *mut i32 = ptr::null_mut();
static INIT: Once = Once::new();
fn acquire_resource() -> *mut i32 {
    INIT.call_once(|| unsafe {
        GLOBAL_RESOURCE = Box::into_raw(Box::new(42));
    });
    unsafe { GLOBAL_RESOURCE }
}
fn run() {
    let handle1 = thread::spawn(|| {
        let ptr1 = acquire_resource();
        ptr1 as usize
    });
    let handle2 = thread::spawn(|| {
        let ptr2 = acquire_resource();
        ptr2 as usize
    });
    let res1 = handle1.join().expect("Thread 1 panicked");
    let res2 = handle2.join().expect("Thread 2 panicked");
    println!("Resource addresses: {} and {}", res1, res2);
}
fn main() {
    run();
}
