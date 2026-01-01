///////////////////////
// Vulnerable Example
///////////////////////
use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_DROP: AtomicUsize = AtomicUsize::new(0);

struct Resource {
    data: i32,
}

impl Resource {
    fn new(val: i32) -> Self {
        Resource { data: val }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        // Record every drop invocation.
        GLOBAL_DROP.fetch_add(1, Ordering::SeqCst);
    }
}

fn process_resource() {
    unsafe {
        // Allocate a Resource and leak its Box pointer into a raw pointer.
        let boxed = Box::new(Resource::new(100));
        let raw = Box::into_raw(boxed);
        
        // First deallocation: recreates Box and drops the resource.
        {
            let _holder = Box::from_raw(raw);
        }
        // Erroneous second deallocation: double free vulnerability.
        {
            let _holder_dup = Box::from_raw(raw);
        }
    }
}

fn main() {
    process_resource();
    // Output the number of times the resource was dropped.
    println!("Global drop count: {}", GLOBAL_DROP.load(Ordering::SeqCst));
}