//////////////////// Vulnerable Version ////////////////////
use std::cell::RefCell;

// A simple structure representing a resource.
struct Resource {
    data: i32,
}

// Global flag used to simulate deallocation tracking.
// In real code, this flag does not exist and double free results in undefined behavior.
static mut FREED: bool = false;

// This function manually deallocates the resource pointed by 'ptr'.
// It does not prevent a double free: if called twice with the same pointer, it triggers a panic.
unsafe fn deallocate(ptr: *mut Resource) {
    if FREED {
        // Second deallocation triggers panic simulating a double free scenario.
        panic!("Double free detected");
    } else {
        FREED = true;
        // Reconstruct the Box and drop it to free memory.
        let _ = Box::from_raw(ptr);
    }
}

// Execute the resource lifecycle.
fn execute() {
    // Allocate the resource on the heap.
    let resource = Box::new(Resource { data: 42 });
    // Convert the Box into a raw pointer so that manual deallocation is possible.
    let raw_ptr = Box::into_raw(resource);
    unsafe {
        // First correct deallocation.
        deallocate(raw_ptr);
        // Erroneous second deallocation leading to a double free vulnerability (CWE-415).
        deallocate(raw_ptr);
    }
}

// Main entry point.
fn main() {
    execute();
}