//
// A Rust program that demonstrates a use-after-free vulnerability (CWE-416)
// using unsafe code, traits, and raw pointers in a multi-threaded context.
// DISCLAIMER: This code is intentionally written to illustrate a vulnerability.
//
pub trait Worker {
    fn evaluate(&self) -> u32;
}

pub struct Processor {
    ptr: *mut u32,
}

impl Processor {
    // Constructs the structure by allocating memory on the heap.
    pub fn new(val: u32) -> Self {
        let b = Box::new(val);
        Self {
            ptr: Box::into_raw(b),
        }
    }

    // An unsafe function that dereferences the raw pointer.
    pub unsafe fn compute(&self) -> u32 {
        *self.ptr
    }
}

// The Drop implementation frees the allocated memory.
impl Drop for Processor {
    fn drop(&mut self) {
        unsafe {
            // Free the memory. After this point, any use of ptr is undefined.
            let _ = Box::from_raw(self.ptr);
        }
    }
}

// This function aims to compute a result, but it introduces a use-after-free:
// It extracts a raw pointer from a local instance, then lets that instance go
// out of scope (freeing the memory), and later dereferences the pointer.
pub fn execute() -> u32 {
    let result: u32;
    let local_ptr: *mut u32;
    {
        let job = Processor::new(42);
        // Capture the internal pointer for later use.
        local_ptr = job.ptr;
    } // 'job' is dropped here, and the underlying memory is freed.
    unsafe {
        // Use-after-free: dereferences local_ptr after memory has been freed.
        result = *local_ptr;
    }
    result
}

fn main() {
    // In a multi-threaded context, the unsafe function could be used concurrently.
    // For simplicity, we call execute directly.
    let res = execute();
    println!("Result is: {}", res);
}