///////////////////////////
// Vulnerable Version Code
///////////////////////////
#![allow(unused)]
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

/// A trait defining operations on a resource.
trait Operations {
    /// Perform an explicit release of the underlying data.
    /// Safety: Callers must ensure this function is not called more than once.
    unsafe fn release(&self);
}

/// Structure that manages a raw pointer allocated on the heap.
pub struct Manager {
    ptr: *mut i32,
}

impl Manager {
    /// Allocates memory for an integer and returns a new Manager.
    pub unsafe fn initialize(value: i32) -> Self {
        // Allocate memory using Box and leak it to get a raw pointer.
        let boxed = Box::new(value);
        Manager {
            ptr: Box::into_raw(boxed),
        }
    }
    
    /// Simulate some processing on the resource.
    /// In this example, it simply calls the explicit release.
    pub unsafe fn process(&self) {
        self.release();
    }
}

/// Global counter to simulate deallocation count.
static DEALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

impl Operations for Manager {
    unsafe fn release(&self) {
        // Vulnerability: We free the memory without checking if it has already been freed.
        // The counter is used to detect multiple frees but here we use it only for logging.
        let count = DEALLOC_COUNT.fetch_add(1, Ordering::SeqCst);
        if count >= 1 {
            // In a real-world scenario, double free can result in undefined behavior rather than an immediate panic.
            // Here we simulate the effect by panicking.
            panic!("Double free detected during explicit release (unsafe manual deallocation)!");
        }
        // Manually drop the Box, deallocating memory.
        let _ = Box::from_raw(self.ptr);
    }
}

/// The destructor for Manager.
impl Drop for Manager {
    fn drop(&mut self) {
        unsafe {
            let count = DEALLOC_COUNT.fetch_add(1, Ordering::SeqCst);
            if count >= 1 {
                // Vulnerability: Drop attempts to free memory that might have already been freed.
                panic!("Double free detected in destructor (Drop)!");
            }
            let _ = Box::from_raw(self.ptr);
        }
    }
}

/// Encapsulated simulation of the main business logic.
pub fn simulate() {
    unsafe {
        // Initialize the manager with a sample value.
        let resource = Manager::initialize(100);
        // Explicitly free memory once.
        resource.process();
        // When 'resource' goes out of scope, Drop is called again, leading to double free.
    }
}

fn main() {
    // In a real application, simulate() would be executed as part of the processing pipeline.
    // For demonstration, we call simulate() directly.
    simulate();
}