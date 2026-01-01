//////////////////////////////////////////////
// Fixed Code for Race Condition (CWE-362)
//////////////////////////////////////////////
#![allow(dead_code)]
use std::ptr;
use std::sync::Once;
use std::thread;

// Global pointer and a Once instance are used to ensure thread-safe initialization.
static mut GLOBAL_RESOURCE: *mut i32 = ptr::null_mut();
static INIT: Once = Once::new();

/// This function safely initializes the global resource using Once to guarantee
/// that initialization only happens one time, even under concurrent calls.
fn acquire_resource() -> *mut i32 {
    // Call_once ensures that the closure is executed exactly once in a thread-safe manner.
    INIT.call_once(|| unsafe {
        // Allocation happens here once and is safe from concurrent access.
        GLOBAL_RESOURCE = Box::into_raw(Box::new(42));
    });
    unsafe { GLOBAL_RESOURCE }
}

fn run() {
    // Spawn two threads concurrently that safely acquire the shared resource.
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

    // With proper synchronization, both threads should receive the same resource pointer.
    println!("Resource addresses: {} and {}", res1, res2);
}

fn main() {
    run();
}