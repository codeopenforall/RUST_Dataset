#![allow(dead_code)]
use std::ptr;
use std::thread;
use std::time::Duration;
static mut GLOBAL_RESOURCE: *mut i32 = ptr::null_mut();
fn acquire_resource() -> *mut i32 {
    unsafe {
        if GLOBAL_RESOURCE.is_null() {                   
            thread::sleep(Duration::from_millis(50));    
            GLOBAL_RESOURCE = Box::into_raw(Box::new(42));
        }
        GLOBAL_RESOURCE
    }
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
