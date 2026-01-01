///////////////////////////////////////////////
// Corrected Version - Using Reference Counting
///////////////////////////////////////////////
use std::sync::Arc;
use std::thread;

pub struct Resource {
    data: Arc<i32>,
}

impl Resource {
    pub fn new(val: i32) -> Self {
        // Use Arc for automatic, thread-safe reference counting.
        Resource { data: Arc::new(val) }
    }
}

impl Clone for Resource {
    fn clone(&self) -> Self {
        // Cloning the Arc increases the reference count safely.
        Resource { data: Arc::clone(&self.data) }
    }
}

fn main() {
    // Create the resource. Cloning it increases the count.
    let resource = Resource::new(100);
    let handle = thread::spawn({
        let clone_resource = resource.clone();
        move || {
            // Thread holds its reference safely.
            let _ = clone_resource;
        }
    });
    handle.join().unwrap();
    // When all instances drop, the Arc automatically deallocates the memory once.
}