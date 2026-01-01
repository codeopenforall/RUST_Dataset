/* 
This Rust program simulates a double-free vulnerability by manually managing a heap‐allocated integer.
It uses an unsafe function to free the allocated memory, but fails to nullify the internal pointer,
thereby allowing a second deallocation on the same pointer. In a real-world context, this may lead to
undefined behavior and memory corruption.
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
    
    // The unsafe function relinquishes ownership by converting the pointer back into a Box to drop it.
    // Vulnerability: It does not nullify the pointer after deallocation.
    unsafe fn free_resource(&mut self) {
        if !self.raw.is_null() {
            // Convert raw pointer to Box and free memory.
            drop(Box::from_raw(self.raw));
            GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst);
            // Vulnerability: Missing pointer reset leads to double free if called again.
        }
    }
}

// The process function performs two deallocations on the same resource.
// The second call leads to double free.
fn process() -> Result<(), &'static str> {
    let mut obj = Container::new(50);
    unsafe {
        obj.free_resource(); // First deallocation.
        obj.free_resource(); // Second deallocation -> double free vulnerability.
    }
    // If memory was freed only once, GLOBAL_COUNT would be 1.
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