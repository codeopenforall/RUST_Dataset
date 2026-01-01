///////////////////////////////////////////////
// Vulnerable Version - Double Free Example
///////////////////////////////////////////////
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

pub struct Resource {
    data: *mut i32,
    flag: *mut AtomicBool,
}

impl Resource {
    pub fn new(val: i32) -> Self {
        // Allocate the integer and an atomic flag to track deallocation.
        let data = Box::into_raw(Box::new(val));
        let flag = Box::into_raw(Box::new(AtomicBool::new(false)));
        Resource { data, flag }
    }
}

impl Clone for Resource {
    // A shallow clone: copies the raw pointers.
    fn clone(&self) -> Self {
        Resource {
            data: self.data,
            flag: self.flag,
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        unsafe {
            // Attempt to atomically mark the resource as freed.
            // The first drop will flip the flag to true.
            if (*self.flag).compare_and_swap(false, true, Ordering::SeqCst) == false {
                // Free both allocations.
                let _ = Box::from_raw(self.data);
                let _ = Box::from_raw(self.flag);
            } else {
                // A second drop (via a clone) triggers a double free.
                panic!("Double free detected");
            }
        }
    }
}

fn main() {
    // Create one instance and then create a shallow copy.
    let resource = Resource::new(100);
    let handle = thread::spawn({
        // Clone the resource, so both instances refer to the same allocation.
        let clone_resource = resource.clone();
        move || {
            // Thread simply holds its clone.
            let _ = clone_resource;
        }
    });
    handle.join().unwrap();
    // When main ends, resource gets dropped and will try to free memory
    // that is already freed by the thread's clone drop.
}