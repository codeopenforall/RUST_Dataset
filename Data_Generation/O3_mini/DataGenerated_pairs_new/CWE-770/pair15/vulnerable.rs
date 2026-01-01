//////////////////////////////////////////////
// Vulnerable Code Sample
//////////////////////////////////////////////

use std::alloc::{alloc, dealloc, Layout};

struct MemoryManager;

impl MemoryManager {
    // Unsafely allocate a buffer of the given size without checking for bounds.
    unsafe fn reserve(&self, size: usize) -> *mut u8 {
        // Construct a layout with alignment 8. Will panic if layout is invalid.
        let layout = Layout::from_size_align(size, 8).unwrap();
        let ptr = alloc(layout);
        if ptr.is_null() {
            panic!("Allocation failed");
        }
        // Initialize all allocated bytes with a constant value.
        for i in 0..size {
            *ptr.add(i) = 0xAA;
        }
        ptr
    }

    // Process a request by allocating `size` bytes.
    // There is no limit check on the size, enabling an attacker to request huge allocations.
    fn process(&self, size: usize) -> Result<(), String> {
        unsafe {
            let ptr = self.reserve(size);
            // Simulate usage of the allocated block.
            // (In a real application, further operations would be performed.)
            let layout = Layout::from_size_align(size, 8).unwrap();
            dealloc(ptr, layout);
        }
        Ok(())
    }
}

fn main() {
    // Read size from the first command-line argument; default to 2000.
    let args: Vec<String> = std::env::args().collect();
    let req_size = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(0)
    } else {
        2000
    };
    let manager = MemoryManager{};
    match manager.process(req_size) {
        Ok(()) => println!("Operation completed successfully."),
        Err(e) => println!("Error encountered: {}", e),
    }
}