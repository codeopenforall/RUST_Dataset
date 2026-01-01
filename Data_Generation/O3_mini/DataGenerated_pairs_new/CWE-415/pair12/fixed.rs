//////////////////// Fixed Version ////////////////////
struct Resource {
    data: i32,
}

// Global flag used to simulate deallocation tracking.
static mut FREED: bool = false;

// The function now guarantees that the resource is deallocated only once.
// It checks the flag and avoids deallocating if the resource has already been freed.
unsafe fn deallocate_once(ptr: *mut Resource) {
    if !FREED {
        FREED = true;
        let _ = Box::from_raw(ptr);
    }
    // If the resource has already been freed, do nothing.
}

// Execute the resource lifecycle without triggering double free.
fn execute() {
    let resource = Box::new(Resource { data: 42 });
    let raw_ptr = Box::into_raw(resource);
    unsafe {
        // Correctly deallocate the resource just once.
        deallocate_once(raw_ptr);
        // Note: The second call is intentionally omitted to prevent double free.
    }
}

// Main entry point.
fn main() {
    execute();
}