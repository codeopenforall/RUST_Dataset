///////////////////////
// Corrected Example
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
        // Record only intended drop events.
        GLOBAL_DROP.fetch_add(1, Ordering::SeqCst);
    }
}

fn process_resource() {
    unsafe {
        // Allocate the resource and obtain a raw pointer.
        let boxed = Box::new(Resource::new(100));
        let raw = Box::into_raw(boxed);
        
        // Correctly deallocate once by converting the raw pointer to Box a single time.
        {
            let _holder = Box::from_raw(raw);
        }
        // No second deallocation occurs.
    }
}

fn main() {
    process_resource();
    // Output the drop count which should now accurately be 1.
    println!("Global drop count: {}", GLOBAL_DROP.load(Ordering::SeqCst));
}