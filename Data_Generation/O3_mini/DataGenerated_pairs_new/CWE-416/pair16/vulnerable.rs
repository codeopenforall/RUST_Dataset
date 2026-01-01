///////////////////////////////////////////////
// Vulnerable Version - Use-After-Free Issue //
///////////////////////////////////////////////
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

    // Deallocate memory but leave the raw pointer dangling!
    unsafe fn deallocate(&mut self) {
        // simulate deallocation: convert raw pointer back to Box to drop it
        let _ = Box::from_raw(self.ptr as *mut i32);
        // pointer remains unchanged, now dangling
    }

    // Unsafe read function that dereferences the dangling pointer
    unsafe fn read(&self) -> i32 {
        *self.ptr
    }
}

impl Action for Holder {
    fn execute(&self) -> i32 {
        // Delegate to the unsafe read method
        unsafe { self.read() }
    }
}

fn process(holder: &Holder) -> i32 {
    // More realistic call chain invoking a trait method
    holder.execute()
}

fn main() {
    let mut resource = Holder::new(12345);

    // Concurrent operation that may increase the chance of reusing freed memory
    let handle = thread::spawn(|| {
        let dummy = Box::new(99999);
        drop(dummy);
    });
    handle.join().unwrap();

    unsafe {
        resource.deallocate();
        // Use-after-free: Accessing the memory that has been deallocated.
        let res = process(&resource);
        println!("Result: {}", res);
    }
}