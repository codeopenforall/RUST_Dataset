/////////////////////////////////////////////
// Corrected Example - Proper Memory Safety  //
// CWE-415 Mitigation: Avoiding Double Free    //
/////////////////////////////////////////////

use std::sync::atomic::{AtomicBool, Ordering};

static FREED: AtomicBool = AtomicBool::new(false);

struct Resource {
    value: i32,
}

impl Drop for Resource {
    fn drop(&mut self) {
        // The drop implementation detects if the resource is dropped twice.
        if FREED.swap(true, Ordering::SeqCst) {
            panic!("Double free detected in drop!");
        }
    }
}

fn process() {
    let boxed = Box::new(Resource { value: 42 });
    // Obtain a raw pointer and relinquish ownership.
    let raw_ptr = Box::into_raw(boxed);
    unsafe {
        // Correct handling: Convert the raw pointer back to a Box exactly once.
        let recovered = Box::from_raw(raw_ptr);
        drop(recovered);
    }
}

fn main() {
    process();
    println!("Completed corrected main function");
}