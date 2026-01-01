/////////////////////////////////////////////////////////////
// Corrected Version: Fixes the double free issue by ensuring
// that the allocated memory is deallocated exactly once.
/////////////////////////////////////////////////////////////

use std::sync::atomic::{AtomicUsize, Ordering};

static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

struct Data {
    value: u32,
}

impl Drop for Data {
    fn drop(&mut self) {
        DROP_COUNT.fetch_add(1, Ordering::SeqCst);
    }
}

fn process_data() {
    // Allocate an instance on the heap.
    let data = Box::new(Data { value: 42 });
    // Convert the smart pointer to a raw pointer.
    let raw_ptr = Box::into_raw(data);

    unsafe {
        // Correctly reconstitute the Box and allow it to be dropped.
        drop(Box::from_raw(raw_ptr));
        // No second deallocation occurs.
    }
}

// This function resets the drop counter, runs the corrected process,
// and returns the number of times the memory has been deallocated.
pub fn run_simulation() -> usize {
    DROP_COUNT.store(0, Ordering::SeqCst);
    process_data();
    DROP_COUNT.load(Ordering::SeqCst)
}

fn main() {
    let drops = run_simulation();
    println!("Drop count: {}", drops);
}