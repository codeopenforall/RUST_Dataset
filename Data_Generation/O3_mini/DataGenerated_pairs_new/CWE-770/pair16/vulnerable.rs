use std::alloc::{alloc, dealloc, Layout};
use std::thread;

// This function performs a raw memory allocation based directly on the input multiplier.
// Lacking any sensible bound check, a malicious caller can supply a high value leading
// to allocation of an excessive amount of memory, potentially exhausting system resources.
fn allocate_resources(count: usize) -> Result<usize, String> {
    // Compute the total number of bytes to allocate.
    // Vulnerability: No check is performed on 'count', allowing unbounded allocation.
    let allocation = count
        .checked_mul(1_000_000)
        .ok_or("Multiplication overflow")?;
    let layout = Layout::from_size_align(allocation, 8).map_err(|_| "Invalid layout")?;
    // Unsafe block performing a raw allocation. In a real-world scenario this may lead to
    // resource exhaustion if 'allocation' is unreasonably high.
    unsafe {
        let ptr = alloc(layout);
        if ptr.is_null() {
            return Err("Allocation failed".into());
        }
        // Immediately free the allocated memory.
        dealloc(ptr, layout);
    }
    Ok(allocation)
}

fn spawn_allocation(count: usize) -> thread::JoinHandle<Result<usize, String>> {
    thread::spawn(move || {
        // Direct invocation without any resource limit enforcement.
        allocate_resources(count)
    })
}

fn main() {
    // Test input that may trigger resource exhaustion by requesting a very high allocation.
    let count = 150;
    let handle = spawn_allocation(count);
    match handle.join() {
        Ok(result) => match result {
            Ok(val) => println!("Successfully allocated {} bytes", val),
            Err(e) => println!("Error during allocation: {}", e),
        },
        Err(_) => println!("Thread panicked"),
    }
}