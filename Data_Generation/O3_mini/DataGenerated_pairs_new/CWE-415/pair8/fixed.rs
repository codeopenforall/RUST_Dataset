//////////////////////////////////////////////////////////////////////////////////////////////////////
// Corrected Implementation: This version sets the pointer to null after manual deallocation.
// In the Drop implementation, the code checks if the pointer is null before attempting deallocation,
// ensuring that the memory is freed only once. An atomic counter continues to be used for test validation.
//////////////////////////////////////////////////////////////////////////////////////////////////////

use std::sync::atomic::{AtomicUsize, Ordering};

static FREE_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct Resource {
    data: *mut i32,
}

impl Resource {
    pub unsafe fn new(val: i32) -> Self {
        // Allocate memory on the heap.
        let boxed = Box::new(val);
        let ptr = Box::into_raw(boxed);
        Self { data: ptr }
    }

    pub unsafe fn release(&mut self) {
        if !self.data.is_null() {
            // Increment the free counter.
            FREE_COUNT.fetch_add(1, Ordering::SeqCst);
            // Deallocate memory and then nullify the pointer to prevent double free.
            let _ = Box::from_raw(self.data);
            self.data = std::ptr::null_mut();
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        unsafe {
            if !self.data.is_null() {
                // Increment the free counter.
                FREE_COUNT.fetch_add(1, Ordering::SeqCst);
                // Deallocate memory and nullify the pointer.
                let _ = Box::from_raw(self.data);
                self.data = std::ptr::null_mut();
            }
        }
    }
}

fn main() {
    unsafe {
        // Create a new resource.
        let mut res = Resource::new(42);
        // Properly release the resource.
        res.release();
        // Drop is called automatically but sees that the pointer is already null.
    }
}