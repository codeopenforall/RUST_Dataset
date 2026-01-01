////////////////////////////////////////////////////////////////
// This code implements a resource allocation routine that 
// spawns a new thread for each requested allocation. It 
// unsafely allocates a 1 KB memory block in each thread using 
// low-level allocation APIs and pointer arithmetic. 
// There is no upper bound on the number of threads created,
// so a user (or malicious caller) may specify an extremely 
// high number thereby exhausting system resources (CWE-770).
////////////////////////////////////////////////////////////////
use std::{thread, env, alloc::{alloc, dealloc, Layout}};

pub fn process(count: usize) -> Result<(), String> {
    // NOTE: No limitation or throttling is applied to count.
    let mut handles = Vec::new();
    for _ in 0..count {
        let handle = thread::spawn(move || {
            unsafe {
                // Allocate a 1024-byte memory block.
                let layout = Layout::from_size_align(1024, 8).unwrap();
                let ptr = alloc(layout);
                if ptr.is_null() {
                    panic!("Memory allocation failed");
                }
                // Write to every byte of the allocated memory.
                for j in 0..1024 {
                    ptr.add(j).write(0);
                }
                dealloc(ptr, layout);
            }
        });
        handles.push(handle);
    }
    // Wait for all threads to complete.
    for handle in handles {
        handle.join().map_err(|_| "Thread panicked".to_string())?;
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let count = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(0)
    } else {
        5
    };
    // Vulnerability: unbounded resource allocation without any cap.
    if let Err(e) = process(count) {
        eprintln!("Error: {}", e);
    }
}