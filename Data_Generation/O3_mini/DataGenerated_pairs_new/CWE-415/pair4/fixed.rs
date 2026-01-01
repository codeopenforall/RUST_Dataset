//////////////////////////////
// Corrected Code Sample
// CWE-415 Mitigation: Double Free Prevention
//////////////////////////////
use std::sync::atomic::{AtomicBool, Ordering};

static FREED: AtomicBool = AtomicBool::new(false);

struct Resource {
    value: i32,
}

impl Drop for Resource {
    fn drop(&mut self) {
        // Ensure that the resource is only freed once.
        if FREED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            panic!("double free detected");
        }
    }
}

fn safe_release() {
    // Allocate the resource on the heap.
    let mem = Box::new(Resource { value: 42 });
    // Extract the raw pointer; however, we will only convert it back
    // into a Box once, ensuring proper and singular deallocation.
    let raw_ptr = Box::into_raw(mem);
    unsafe {
        // Reconstruct the Box exactly once. Ownership is uniquely restored.
        let _owner = Box::from_raw(raw_ptr);
        // Do not attempt a second reconstruction.
    }
}

fn run() {
    safe_release();
}

fn main() {
    run();
}