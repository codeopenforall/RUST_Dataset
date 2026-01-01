//////////////////////////////////////////////
// Corrected Code: Safe Memory Ownership
// This version encapsulates the allocation in a struct that ensures
// the memory is freed only once. The pointer is nulled after explicit free.
//////////////////////////////////////////////
#![allow(unused)]
use std::ptr;

struct Data {
    value: u32,
}

// Owner takes responsibility for the allocated memory.
struct Owner {
    ptr: *mut Data,
}

impl Owner {
    fn new(val: u32) -> Self {
        let boxed = Box::new(Data { value: val });
        Owner { ptr: Box::into_raw(boxed) }
    }

    // Safely release the allocated memory if it hasn't been freed.
    unsafe fn release(&mut self) {
        if !self.ptr.is_null() {
            Box::from_raw(self.ptr);
            self.ptr = ptr::null_mut();
        }
    }
}

// Drop implementation guarantees that memory is freed exactly once.
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
    // The Owner struct encapsulates the life cycle of the allocation.
    let mut owner = Owner::new(42);
    unsafe {
        // Explicit release frees the memory once; the Drop will do nothing.
        owner.release();
    }
}

fn main() {
    app_run();
}