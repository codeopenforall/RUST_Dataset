//////////////////////////////////////////////
// Vulnerable Code: Double Free Example (CWE-415)
// This example manually deallocates memory using unsafe code.
// It erroneously frees the same allocated memory twice.
//////////////////////////////////////////////
#![allow(unused)]
use std::sync::atomic::{AtomicBool, Ordering};

struct Data {
    value: u32,
}

// Global flag to simulate detection of double free.
static mut FREED: bool = false;

// Manually free the memory pointed by ptr.
// On the second call with the same pointer, a double free occurs.
unsafe fn free_data(ptr: *mut Data) {
    if FREED {
        panic!("Double free detected!");
    }
    // Convert the raw pointer back into a Box and drop it.
    Box::from_raw(ptr);
    FREED = true;
}

pub fn app_run() {
    // Reset the flag for deterministic behavior.
    unsafe {
        FREED = false;
    }
    // Allocate memory on the heap.
    let boxed = Box::new(Data { value: 42 });
    // Extract a raw pointer and intentionally leak ownership.
    let raw = Box::into_raw(boxed);
    unsafe {
        free_data(raw); // First deallocation.
        free_data(raw); // Second deallocation: Double free vulnerability.
    }
}

fn main() {
    app_run();
}