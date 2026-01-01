////////////////////////////////////////////////////////////////////////////////
// Corrected Version
////////////////////////////////////////////////////////////////////////////////
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

// Global counter remains to simulate deallocation; proper usage is now enforced.
static FREED: AtomicUsize = AtomicUsize::new(0);

// Same custom deallocation function as before.
unsafe fn custom_release(ptr: *mut i32) {
    if FREED.fetch_add(1, Ordering::SeqCst) >= 1 {
        panic!("Double free detected");
    }
    drop(Box::from_raw(ptr));
}

struct Resource {
    ptr: *mut i32,
    // An atomic flag to mark whether the resource has been freed.
    freed: AtomicBool,
}

unsafe impl Send for Resource {}
unsafe impl Sync for Resource {}

impl Resource {
    // Allocates a new heap integer and returns a new resource with its freed flag set to false.
    fn new(val: i32) -> Self {
        let boxed = Box::new(val);
        let ptr = Box::into_raw(boxed);
        Resource { 
            ptr, 
            freed: AtomicBool::new(false) 
        }
    }

    // Releases the resource only if it has not been freed already.
    fn release(&self) {
        if !self.freed.swap(true, Ordering::SeqCst) {
            unsafe {
                custom_release(self.ptr);
            }
        }
    }
}

impl Drop for Resource {
    // Drop checks the freed flag and only deallocates the memory if release() was not already called.
    fn drop(&mut self) {
        if !self.freed.load(Ordering::SeqCst) {
            unsafe {
                custom_release(self.ptr);
            }
        }
    }
}

fn main() {
    let res = Arc::new(Resource::new(42));
    let res_clone = Arc::clone(&res);
    let handle = thread::spawn(move || {
        // The first and only deallocation marks the resource as freed.
        res_clone.release();
    });
    handle.join().unwrap();
    // When 'res' goes out of scope, Drop observes that the memory has already been freed,
    // preventing a double free.
}