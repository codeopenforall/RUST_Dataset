////////////////////////////////////////////////////////////////////////////////
// Vulnerable Version
////////////////////////////////////////////////////////////////////////////////
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

// Global counter used in custom deallocation to simulate detection of double free.
static FREED: AtomicUsize = AtomicUsize::new(0);

// Custom deallocation function that unsafely converts a raw pointer back to Box and drops it.
// If the function is called more than once for the same pointer, it panics to simulate a crash.
unsafe fn custom_release(ptr: *mut i32) {
    if FREED.fetch_add(1, Ordering::SeqCst) >= 1 {
        panic!("Double free detected");
    }
    drop(Box::from_raw(ptr));
}

struct Resource {
    ptr: *mut i32,
}

unsafe impl Send for Resource {}
unsafe impl Sync for Resource {}

impl Resource {
    // Allocates a new heap integer and leaks its pointer.
    fn new(val: i32) -> Self {
        let boxed = Box::new(val);
        let ptr = Box::into_raw(boxed);
        Resource { ptr }
    }

    // Releases the resource by converting the raw pointer back to Box and dropping it.
    // In this vulnerable version, the pointer is not marked as freed.
    fn release(&self) {
        unsafe {
            custom_release(self.ptr);
        }
    }
}

impl Drop for Resource {
    // When the object is dropped, it calls the same unsafe deallocation.
    // Because release() does not cancel the drop action, the same memory is freed twice.
    fn drop(&mut self) {
        unsafe {
            custom_release(self.ptr);
        }
    }
}

fn main() {
    let res = Arc::new(Resource::new(42));
    let res_clone = Arc::clone(&res);
    let handle = thread::spawn(move || {
        // First deallocation in a spawned thread.
        res_clone.release();
    });
    handle.join().unwrap();
    // When 'res' goes out of scope, Drop is invoked again,
    // resulting in a double free and triggering a panic.
}