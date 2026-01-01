//////////////////////////////////////////////
// Vulnerable Code for Race Condition (CWE-362)
//////////////////////////////////////////////
#![allow(dead_code)]
use std::ptr;
use std::thread;
use std::time::Duration;

// Global pointer used to simulate shared resource initialization without proper synchronization.
// This global is accessed unsafely and may be concurrently initialized.
static mut GLOBAL_RESOURCE: *mut i32 = ptr::null_mut();

/// This function unsafely initializes the global resource if it has not been set.
/// It uses a check-then-act pattern without proper atomic guarantees and
/// includes an intentional sleep to widen the race window.
fn acquire_resource() -> *mut i32 {
    unsafe {
        if GLOBAL_RESOURCE.is_null() {                   // Vulnerable Check (Race Condition)
            // Intentional delay to increase the likelihood of a race.
            thread::sleep(Duration::from_millis(50));    
            // Allocating the resource unsafely without locking; race condition possible here.
            GLOBAL_RESOURCE = Box::into_raw(Box::new(42));
        }
        GLOBAL_RESOURCE
    }
}

fn run() {
    // Spawn two threads concurrently that attempt to initialize the shared resource.
    let handle1 = thread::spawn(|| {
        let ptr1 = acquire_resource();
        // Return the pointer as usize for easier comparison.
        ptr1 as usize
    });

    let handle2 = thread::spawn(|| {
        let ptr2 = acquire_resource();
        ptr2 as usize
    });

    let res1 = handle1.join().expect("Thread 1 panicked");
    let res2 = handle2.join().expect("Thread 2 panicked");

    // In a correctly synchronized singleton, these pointers should be identical.
    // Due to the race condition, they can be different.
    println!("Resource addresses: {} and {}", res1, res2);
}

fn main() {
    run();
}