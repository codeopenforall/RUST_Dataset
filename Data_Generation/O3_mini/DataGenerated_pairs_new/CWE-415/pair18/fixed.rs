/*
In this corrected version the resource's manual deallocation function has been fixed. After freeing the memory,
it sets the internal pointer to null. The Drop implementation now checks the pointer and deallocates only if it
has not already been set to null. This prevents the double free while preserving the intended behavior.
*/

use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_FREE_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct Resource {
    ptr: *mut i32,
}

unsafe impl Send for Resource {}

impl Resource {
    pub fn new(val: i32) -> Self {
        let boxed = Box::new(val);
        let ptr = Box::into_raw(boxed);
        Self { ptr }
    }

    // In the fixed version, after releasing the memory manually, we set the pointer to null.
    pub unsafe fn manual_release(&mut self) {
        if !self.ptr.is_null() {
            let _ = Box::from_raw(self.ptr);
            GLOBAL_FREE_COUNT.fetch_add(1, Ordering::SeqCst);
            self.ptr = std::ptr::null_mut();
        }
    }

    // Same helper function; now calling it after release would be considered an error.
    pub unsafe fn get_value(&self) -> Option<i32> {
        if self.ptr.is_null() {
            None
        } else {
            Some(*self.ptr)
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                let _ = Box::from_raw(self.ptr);
                GLOBAL_FREE_COUNT.fetch_add(1, Ordering::SeqCst);
            }
        }
    }
}

pub fn free_count() -> usize {
    GLOBAL_FREE_COUNT.load(Ordering::SeqCst)
}

pub fn run() {
    let mut res = Resource::new(42);
    unsafe {
        res.manual_release();
    }
    // When the resource goes out of scope, Drop checks the pointer and does not free memory again.
}

fn main() {
    run();
}