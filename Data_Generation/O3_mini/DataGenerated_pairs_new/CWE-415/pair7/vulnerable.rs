/////////////////////////////////////////////
// Vulnerability Example - Double Free Issue //
// CWE-415: Double Free Vulnerability         //
/////////////////////////////////////////////

use std::sync::atomic::{AtomicBool, Ordering};

static FREED: AtomicBool = AtomicBool::new(false);

struct Resource {
    value: i32,
}

impl Drop for Resource {
    fn drop(&mut self) {
        // This custom drop implementation detects if the resource
        // is dropped more than once. The first drop sets the flag,
        // and the second drop triggers a panic.
        if FREED.swap(true, Ordering::SeqCst) {
            panic!("Double free detected in drop!");
        }
    }
}

fn process() {
    let boxed = Box::new(Resource { value: 42 });
    // Obtain a raw pointer and lose ownership.
    let raw_ptr = Box::into_raw(boxed);
    unsafe {
        // First deallocation: convert the raw pointer back and drop it.
        let first = Box::from_raw(raw_ptr);
        drop(first);
        // Vulnerability: The same raw pointer is converted again 
        // to a Box and dropped a second time, causing a double free.
        let second = Box::from_raw(raw_ptr);
        drop(second);
    }
}

fn main() {
    process();
    println!("Completed vulnerable main function");
}