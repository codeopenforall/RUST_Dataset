//////////////////////////////////////////////////////////////////////////////////////////////////////
// Vulnerable Implementation: This implementation manually deallocates the memory in the
// "release" method, and then again in the Drop implementation without nullifying the pointer.
// This leads to a double free (CWE-415) once the object goes out of scope. An atomic counter is
// used to simulate counting the free operations for testing purposes.
//////////////////////////////////////////////////////////////////////////////////////////////////////

use std::sync::atomic::{AtomicUsize, Ordering};

static FREE_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct Resource {
    data: *mut i32,
}

impl Resource {
    pub unsafe fn new(val: i32) -> Self {
        // Allocate on the heap and leak ownership by converting to a raw pointer.
        let boxed = Box::new(val);
        let ptr = Box::into_raw(boxed);
        Self { data: ptr }
    }

    pub unsafe fn release(&mut self) {
        if !self.data.is_null() {
            // Increment the free counter.
            FREE_COUNT.fetch_add(1, Ordering::SeqCst);
            // Vulnerable deallocation: free the memory without nullification.
            let _ = Box::from_raw(self.data);
            // Pointer not nullified -> drop() will free it again.
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        unsafe {
            if !self.data.is_null() {
                // Increment the free counter.
                FREE_COUNT.fetch_add(1, Ordering::SeqCst);
                // Vulnerable deallocation: free potentially already freed memory.
                let _ = Box::from_raw(self.data);
            }
        }
    }
}

fn main() {
    unsafe {
        // Create a new resource with an initial value.
        let mut res = Resource::new(42);
        // Manually release the resource.
        res.release();
        // Upon exiting main, Drop::drop is automatically called, triggering double free.
    }
}