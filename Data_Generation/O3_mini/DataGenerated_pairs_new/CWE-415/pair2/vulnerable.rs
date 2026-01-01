/////////////////////////////////////////////////////////////
// Vulnerable Version: Contains a double free vulnerability.
// This example manually frees the same memory twice
// by converting a Box into a raw pointer and then creating
// two Box instances from that pointer. This leads to a 
// double free (CWE-415) situation.
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
        // First deallocation.
        drop(Box::from_raw(raw_ptr));
        // Second deallocation: erroneous double free.
        drop(Box::from_raw(raw_ptr));
    }
}

// This function resets the drop counter, runs the vulnerable process,
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