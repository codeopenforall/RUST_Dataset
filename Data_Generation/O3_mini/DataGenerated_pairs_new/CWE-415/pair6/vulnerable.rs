////////////////////////////////////////////////////////////
// Vulnerable code for a double free issue (CWE-415)
// This code uses unsafe manual deallocation with a global counter
// to emulate a double free scenario. In this example, a resource is
// allocated on the heap, manually freed by an unsafe function, and
// later freed again in the Drop implementation. The global counter
// “FREE_COUNT” simulates detection by panicking on the second free.
////////////////////////////////////////////////////////////
#![allow(unused)]
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};

static FREE_COUNT: AtomicUsize = AtomicUsize::new(0);

struct Resource {
    ptr: *mut i32,
}

impl Resource {
    // Create a new resource by allocating an i32 on the heap.
    unsafe fn new(val: i32) -> Self {
        let b = Box::new(val);
        Self { ptr: Box::into_raw(b) }
    }
    // Manually free the resource.
    // This routine does not clear the internal pointer, so when the
    // instance is dropped, the same memory is freed again.
    unsafe fn manual_drop(&mut self) {
        if !self.ptr.is_null() {
            // Simulate a free counter check. The first call increments FREE_COUNT
            // to 1. The subsequent free (in Drop) will see a count >= 1 and panic.
            if FREE_COUNT.fetch_add(1, Ordering::SeqCst) >= 1 {
                panic!("Double free detected");
            }
            // Free the allocated memory unsafely.
            let _ = Box::from_raw(self.ptr);
        }
    }
}

impl Drop for Resource {
    // The Drop implementation frees the resource if it has not been null.
    // Because manual_drop does not nullify the pointer, Drop erroneously
    // attempts to free the memory a second time, causing a double free.
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                if FREE_COUNT.fetch_add(1, Ordering::SeqCst) >= 1 {
                    panic!("Double free detected");
                }
                let _ = Box::from_raw(self.ptr);
            }
        }
    }
}

fn main() {
    unsafe {
        // Allocate a resource.
        let mut res = Resource::new(100);
        // Manually free the resource.
        res.manual_drop();
        // When 'res' goes out of scope, Drop runs and attempts to free the same memory.
    }
}