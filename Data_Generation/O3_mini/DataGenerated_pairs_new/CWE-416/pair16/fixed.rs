////////////////////////////////////////////
// Fixed Version - Preventing Dangling Use //
////////////////////////////////////////////
#![allow(dead_code)]
use std::thread;

trait Action {
    fn execute(&self) -> i32;
}

struct Holder {
    ptr: *const i32,
}

impl Holder {
    fn new(value: i32) -> Self {
        let b = Box::new(value);
        let ptr = Box::into_raw(b);
        Holder { ptr }
    }

    // Safer deallocation: after freeing, nullify the pointer.
    unsafe fn deallocate(&mut self) {
        if !self.ptr.is_null() {
            let _ = Box::from_raw(self.ptr as *mut i32);
            self.ptr = std::ptr::null();
        }
    }

    // Read function now checks if the pointer is valid.
    fn read(&self) -> i32 {
        if self.ptr.is_null() {
            // Return a sentinel value indicating an invalid access.
            -1
        } else {
            // Safe because we verify the pointer is valid.
            unsafe { *self.ptr }
        }
    }
}

impl Action for Holder {
    fn execute(&self) -> i32 {
        self.read()
    }
}

fn process(holder: &Holder) -> i32 {
    holder.execute()
}

fn main() {
    let mut resource = Holder::new(12345);

    // Perform a concurrent operation; this mimics real-world usage.
    let handle = thread::spawn(|| {
        let dummy = Box::new(99999);
        drop(dummy);
    });
    handle.join().unwrap();

    unsafe {
        resource.deallocate();
    }
    // The safe read function now returns -1 to flag that the memory was already freed.
    let res = process(&resource);
    println!("Result: {}", res);
}