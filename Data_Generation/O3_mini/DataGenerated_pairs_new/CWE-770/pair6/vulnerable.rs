/////////////////////////////////////////////////////////////////
// Vulnerability: Unbounded Resource Allocation (CWE-770)
// This code accepts a user‐provided count and then allocates memory
// and spawns that many threads without enforcing any limit. The use
// of unsafe blocks to manually allocate memory and the unchecked spawn
// of threads creates a risk of uncontrolled resource utilization.
/////////////////////////////////////////////////////////////////

use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::thread;

fn process(count: usize) -> Result<usize, &'static str> {
    // Compute allocation size: multiply count by 10.
    // No cap is applied, so a very large input might cause resource exhaustion.
    let alloc_size = count.checked_mul(10).ok_or("Multiplier overflow")?;
    
    unsafe {
        let layout = Layout::from_size_align(alloc_size, 8).map_err(|_| "Layout error")?;
        let raw_ptr = alloc(layout);
        if raw_ptr.is_null() {
            return Err("Memory allocation failed");
        }
        // Create a Vec from the raw pointer without any bounds checks during its lifetime.
        let vec = Vec::from_raw_parts(raw_ptr, alloc_size, alloc_size);
        // Perform a dummy computation on the vector elements.
        let _dummy: u8 = vec.iter().fold(0, |acc, &val| acc.wrapping_add(val));
        // Issue a compiler fence to prevent reordering.
        std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
        // Manually free the allocated memory.
        dealloc(raw_ptr, layout);
        // Prevent double free via Vec’s destructor.
        std::mem::forget(vec);
    }

    // Spawn "count" threads, each performing an unsafe volatile read.
    let mut handles = Vec::with_capacity(count);
    for i in 0..count {
        let handle = thread::spawn(move || {
            unsafe {
                // Unsafe read from the local variable.
                let val = ptr::read_volatile(&i);
                val
            }
        });
        handles.push(handle);
    }
    // Join all threads.
    for handle in handles {
        handle.join().map_err(|_| "Thread join error")?;
    }
    Ok(count)
}

fn main() {
    // Parse count from command line if provided, defaulting to 50.
    let args: Vec<String> = std::env::args().collect();
    let count = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(50)
    } else {
        50
    };

    match process(count) {
        Ok(c) => println!("Successfully processed {} resources.", c),
        Err(e) => eprintln!("Error encountered: {}", e),
    }
}