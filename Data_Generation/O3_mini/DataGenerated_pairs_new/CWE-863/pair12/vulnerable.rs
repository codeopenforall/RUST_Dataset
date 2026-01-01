use std::env;
use std::sync::Arc;
use std::thread;

static mut GLOBAL_ARRAY: *mut [u8; 1024] = std::ptr::null_mut();

/// Allocates a vector with user-provided capacity without any upper limit.
/// This can lead to excessive memory allocation.
fn allocate_resources(size: usize) -> Vec<u8> {
    // Construct a vector with capacity equal to user input.
    let mut vec = Vec::with_capacity(size);
    // UNSAFE: Directly set the length of the vector bypassing initialization checks.
    // This is the critical point where an unbounded size is accepted.
    unsafe {
        vec.set_len(size);
    }
    vec
}

/// Processes the provided input by allocating resources and spawning a thread.
/// The thread performs an unsafe write to a global pointer.
fn process_input(input: usize) -> Result<Arc<Vec<u8>>, &'static str> {
    // Vulnerability: Using untrusted input to allocate memory without capping.
    let data = allocate_resources(input);
    let arc_data = Arc::new(data);
    let arc_clone = Arc::clone(&arc_data);
    thread::spawn(move || {
        unsafe {
            // UNSAFE: Overwrites a global pointer with a new boxed array.
            GLOBAL_ARRAY = Box::into_raw(Box::new([0u8; 1024]));
        }
        // Naively compute a sum over the allocated memory.
        let sum: usize = arc_clone.iter().map(|&x| x as usize).sum();
        drop(sum);
    })
    .join()
    .unwrap();
    Ok(arc_data)
}

fn main() {
    // Input taken from command line arguments.
    let args: Vec<String> = env::args().collect();
    // If an argument is provided, parse it as the allocation size.
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