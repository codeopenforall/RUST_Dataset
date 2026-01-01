use std::env;
use std::thread;

fn allocate_resources(count: usize) -> Result<(), String> {
    // Create a vector to hold thread handles.
    let mut handles = Vec::with_capacity(count);
    // Spawn one thread per requested resource without capping the count.
    for i in 0..count {
        let handle = thread::spawn(move || {
            // Unsafe block performing unchecked pointer arithmetic.
            unsafe {
                let data = [1u8; 10];
                // Vulnerability: no bounds check on the offset; when i ≥ 10 this will read outside the array.
                let ptr = data.as_ptr().offset(i as isize);
                // Dereference the pointer, which may lead to undefined behavior or a crash.
                let _ = *ptr;
            }
        });
        handles.push(handle);
    }
    // Wait for all threads to complete.
    for h in handles {
        h.join().map_err(|_| "Thread panicked".to_string())?;
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Read the count from command-line arguments.
    let count: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };
    // Execute the resource allocation.
    match allocate_resources(count) {
        Ok(_) => println!("Completed allocation with count {}", count),
        Err(e) => println!("Error: {}", e),
    }
}