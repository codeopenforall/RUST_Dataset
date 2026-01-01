/*
This code instantiates a resource whose memory is allocated on the heap and managed manually using unsafe code.
It defines a structure that wraps a raw pointer. There are two explicit deallocation calls: one triggered by a manual
call and the other via the Drop trait. Because the manual deallocation does not nullify the pointer, the Drop code
attempts to free the memory a second time (a double free) when the Resource instance goes out of scope.
The code uses concurrency markers (an unsafe impl of Send) and a global atomic counter to record each deallocation.
This simulates a realistic double free vulnerability (CWE-415) observed in some unsafe Rust patterns.
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

    // This function manually frees the allocated memory.
    // Note: It frees the memory without marking that it has been freed, leaving `ptr` unchanged.
    pub unsafe fn manual_release(&mut self) {
        if !self.ptr.is_null() {
            // Free the memory (first free)
            let _ = Box::from_raw(self.ptr);
            GLOBAL_FREE_COUNT.fetch_add(1, Ordering::SeqCst);
            // Vulnerability: pointer remains non-null, so Drop will free it again.
        }
    }

    // A helper function that unsafely retrieves the value pointed to.
    // (For illustration; in real-world it might trigger use-after-free if called after manual_release.)
    pub unsafe fn get_value(&self) -> i32 {
        *self.ptr
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                // Second free occurs here in Drop when the resource goes out of scope.
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
    // Create a resource and manually release it.
    let mut res = Resource::new(42);
    unsafe {
        res.manual_release();
    }
    // When main exits the Resource will be dropped, triggering a second free.
}

fn main() {
    run();
}