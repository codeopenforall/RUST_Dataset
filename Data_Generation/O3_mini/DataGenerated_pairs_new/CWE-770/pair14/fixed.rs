use std::env;
use std::thread;

const MAX_THREADS: usize = 10; // Maximum allowed threads to ensure safe pointer access.

fn allocate_resources(count: usize) -> Result<(), String> {
    // Enforce an upper limit to prevent unsafe memory access.
    if count > MAX_THREADS {
        return Err("Too many threads requested, potential out-of-bound access prevented".to_string());
    }
    let mut handles = Vec::with_capacity(count);
    for i in 0..count {
        let handle = thread::spawn(move || {
            unsafe {
                let data = [1u8; 10];
                // With count capped, i is guaranteed to be < MAX_THREADS and within data's bounds.
                let ptr = data.as_ptr().offset(i as isize);
                let _ = *ptr;
            }
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().map_err(|_| "Thread panicked".to_string())?;
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let count: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };
    match allocate_resources(count) {
        Ok(_) => println!("Allocation safely completed with count {}", count),
        Err(e) => println!("Error: {}", e),
    }
}