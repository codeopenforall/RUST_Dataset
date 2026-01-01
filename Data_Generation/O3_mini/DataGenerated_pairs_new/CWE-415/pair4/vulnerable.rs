//////////////////////////////
// Vulnerable Code Sample
// CWE-415: Double Free
//////////////////////////////
use std::sync::atomic::{AtomicBool, Ordering};

static FREED: AtomicBool = AtomicBool::new(false);

struct Resource {
    value: i32,
}

impl Drop for Resource {
    fn drop(&mut self) {
        // On first drop, set FREED to true.
        // On subsequent drop, the compare_exchange fails, triggering a panic.
        if FREED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            panic!("double free detected");
        }
    }
}

fn unsafe_release() {
    // Allocate the resource on the heap.
    let mem = Box::new(Resource { value: 42 });
    // Extract the raw pointer from the Box ownership.
    let raw_ptr = Box::into_raw(mem);
    unsafe {
        // First conversion: takes ownership and schedules drop.
        let _first_owner = Box::from_raw(raw_ptr);
        // Second conversion: erroneously creates another owner,
        // leading to a double free when it goes out of scope.
        let _second_owner = Box::from_raw(raw_ptr); // Vulnerability occurs here.
    }
}

fn run() {
    unsafe_release();
}

fn main() {
    run();
}