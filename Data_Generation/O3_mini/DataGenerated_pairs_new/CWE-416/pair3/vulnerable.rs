use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;
use std::thread;

/// A simple structure holding an integer value.
struct Data {
    value: u32,
}

/// A container that manages a raw pointer to Data.
struct Holder {
    ptr: *mut Data,
}

impl Holder {
    /// Allocates a new instance and returns a Holder with a raw pointer.
    fn new(val: u32) -> Self {
        let boxed = Box::new(Data { value: val });
        Self { ptr: Box::into_raw(boxed) }
    }

    /// Unsafely reads the value from the pointed-to Data.
    unsafe fn read(&self) -> u32 {
        (*self.ptr).value
    }
}

impl Drop for Holder {
    fn drop(&mut self) {
        unsafe {
            // Free the allocated Data.
            if !self.ptr.is_null() {
                Box::from_raw(self.ptr);
            }
        }
    }
}

// Global atomic pointer to Data that will be accessed concurrently.
static GLOBAL_PTR: AtomicPtr<Data> = AtomicPtr::new(ptr::null_mut());

/// Runs the core operation.
/// It creates a Holder, stores its pointer globally, drops the Holder (freeing the memory)
/// and then spawns a thread that unsafely dereferences the freed pointer.
pub fn run_op() -> u32 {
    let holder = Holder::new(100);
    GLOBAL_PTR.store(holder.ptr, Ordering::SeqCst);
    // Free the memory; the pointer stored in GLOBAL_PTR is now dangling.
    drop(holder);
    // Spawn a thread which uses the dangling pointer, triggering a use-after-free.
    let handle = thread::spawn(|| unsafe {
        let p = GLOBAL_PTR.load(Ordering::SeqCst);
        // Use-after-free vulnerability: dereferencing memory after it has been freed.
        (*p).value
    });
    let result = handle.join().unwrap();
    result
}

fn main() {
    let res = run_op();
    println!("Result: {}", res);
}