////////////////////////////////////////////////////////////
// Corrected code addressing the double free issue (CWE-415)
// The fix consists of nullifying the internal pointer after manual
// deallocation in order to prevent Drop from deallocating the same
// memory a second time. The overall structure and logic remain
// realistic and using unsafe code to mimic real-world uses.
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
    // After freeing, the pointer is set to null so that Drop
    // will not attempt to free the memory again.
    unsafe fn manual_drop(&mut self) {
        if !self.ptr.is_null() {
            if FREE_COUNT.fetch_add(1, Ordering::SeqCst) >= 1 {
                panic!("Double free detected");
            }
            let _ = Box::from_raw(self.ptr);
            // Prevent a double free by clearing the pointer.
            self.ptr = ptr::null_mut();
        }
    }
}

impl Drop for Resource {
    // The Drop implementation frees the resource only if the pointer
    // is not null. Because manual_drop sets the pointer to null after
    // deallocating, Drop will not attempt a second free.
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
        // Manually free the resource and nullify the pointer.
        res.manual_drop();
        // When 'res' goes out of scope, Drop checks the pointer, finds it null,
        // and does nothing.
    }
}