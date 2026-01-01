use std::thread;
use std::time::Duration;

struct Resource {
    ptr: *mut i32,
}

impl Resource {
    // Allocate an integer on the heap and obtain a raw pointer.
    fn new(val: i32) -> Self {
        let b = Box::new(val);
        let ptr = Box::into_raw(b);
        Resource { ptr }
    }

    // Unsafely deallocates the resource without clearing the internal pointer.
    // This introduces a use‐after‐free vulnerability.
    unsafe fn dispose(&mut self) {
        let _ = Box::from_raw(self.ptr);
    }

    // Unsafely reads the value through a raw pointer.
    // If the pointer was previously freed, this is a use‐after‐free.
    unsafe fn read(&self) -> i32 {
        *self.ptr
    }
}

// Public function that demonstrates the vulnerability.
// It creates a resource initialized to 123, frees it,
// then forces a re-allocation (which may reuse the same memory)
// and finally reads the value via the stale pointer.
pub fn execute() -> i32 {
    let mut res = Resource::new(123);
    unsafe {
        res.dispose();                   // Vulnerability: freeing memory without invalidating pointer.
        // Force a new allocation which may (deterministically, in this test scenario)
        // reuse the just-freed memory.
        let _dummy = Box::new(999);
        res.read()                       // Vulnerability: using memory after free.
    }
}

fn main() {
    let result = execute();
    println!("Value: {}", result);
}