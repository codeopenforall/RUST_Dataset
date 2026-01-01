/* 
This corrected Rust program fixes the double-free vulnerability by ensuring that after the resource is freed,
the internal pointer is set to null. This prevents a second accidental deallocation.
*/
use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_COUNT: AtomicUsize = AtomicUsize::new(0);

struct Container {
    raw: *mut i32,
}

impl Container {
    fn new(val: i32) -> Self {
        let boxed = Box::new(val);
        Self { raw: Box::into_raw(boxed) }
    }
    
    // The unsafe function now nullifies the pointer after deallocation.
    unsafe fn free_resource(&mut self) {
        if !self.raw.is_null() {
            drop(Box::from_raw(self.raw));
            GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst);
            // Fixed: Reset the pointer after freeing to avoid double free.
            self.raw = std::ptr::null_mut();
        }
    }
}

// The process function attempts to free the resource twice.
// In the fixed version, the second attempt is a no-op.
fn process() -> Result<(), &'static str> {
    let mut obj = Container::new(50);
    unsafe {
        obj.free_resource(); // First deallocation.
        obj.free_resource(); // Second call does nothing.
    }
    // With proper handling the resource is freed only once.
    if GLOBAL_COUNT.load(Ordering::SeqCst) == 1 {
        Ok(())
    } else {
        Err("Double free occurred")
    }
}

fn main() {
    let result = process();
    println!("Deallocation count: {}", GLOBAL_COUNT.load(Ordering::SeqCst));
    match result {
        Ok(_) => println!("Operation completed successfully."),
        Err(e) => println!("Error: {}", e),
    }
}