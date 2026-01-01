///////////////////////////////////////////////////////////
// Vulnerable Rust Code: Resource Allocation Without Limits
///////////////////////////////////////////////////////////
use std::thread;
use std::ptr;

struct ResourceManager;

impl ResourceManager {
    fn new() -> Self {
        ResourceManager
    }

    // This method allocates a vector of the given size using an unsafe block
    // to manually set the vector’s length and initialize its memory.
    // There is no checking to limit the allocation, which can lead to resource exhaustion.
    fn allocate_items(&self, count: usize) -> Result<Vec<u8>, &'static str> {
        unsafe {
            // Create a vector with the specified capacity (potentially huge)
            let mut vec = Vec::with_capacity(count);
            // Unsafely set the length to the requested count without bounds check
            vec.set_len(count);
            // Manually initialize each element to a fixed byte value
            for i in 0..count {
                // Using unchecked access here in an unsafe block.
                *vec.get_unchecked_mut(i) = 0xAA;
            }
            Ok(vec)
        }
    }
}

// This function spawns a thread to simulate concurrent processing of the allocation request.
fn process(count: usize) -> Result<Vec<u8>, &'static str> {
    let manager = ResourceManager::new();
    let handle = thread::spawn(move || {
         manager.allocate_items(count)
    });
    // Unwrap the join result; if the thread panicked, propagate the panic.
    handle.join().unwrap()
}

fn main() {
    // Read the allocation size from the command-line arguments (defaulting to 0)
    let args: Vec<String> = std::env::args().collect();
    let count = if args.len() > 1 {
         args[1].parse().unwrap_or(0)
    } else {
         0
    };
    match process(count) {
         Ok(vec) => println!("Allocation completed, size: {}", vec.len()),
         Err(e) => println!("Failed: {}", e),
    }
}