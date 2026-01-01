use std::env;
use std::sync::Arc;
use std::thread;

const MAX_ALLOCATION: usize = 10 * 1024 * 1024; // Limit allocations to 10 MB

static mut GLOBAL_ARRAY: *mut [u8; 1024] = std::ptr::null_mut();

/// Safely allocates a vector ensuring the requested size does not exceed a prescribed maximum.
fn allocate_resources(size: usize) -> Result<Vec<u8>, &'static str> {
    if size > MAX_ALLOCATION {
        return Err("Requested allocation size exceeds the allowed limit");
    }
    let mut vec = Vec::with_capacity(size);
    // SAFETY: The length is set only when the allocation size is within safe bounds.
    unsafe {
        vec.set_len(size);
    }
    Ok(vec)
}

/// Processes the input by allocating resources with a cap and spawns a thread.
/// The thread performs an unsafe write to a global pointer in a controlled manner.
fn process_input(input: usize) -> Result<Arc<Vec<u8>>, &'static str> {
    let data = allocate_resources(input)?;
    let arc_data = Arc::new(data);
    let arc_clone = Arc::clone(&arc_data);
    thread::spawn(move || {
        unsafe {
            GLOBAL_ARRAY = Box::into_raw(Box::new([0u8; 1024]));
        }
        let sum: usize = arc_clone.iter().map(|&x| x as usize).sum();
        drop(sum);
    })
    .join()
    .unwrap();
    Ok(arc_data)
}

fn main() {
    // Command line argument as input.
    let args: Vec<String> = env::args().collect();
    let input: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(10)
    } else {
        10
    };
    match process_input(input) {
        Ok(_) => println!("Completed allocation with input: {}", input),
        Err(e) => println!("Error: {}", e),
    }
}