/*
This code demonstrates a resource allocation function that spawns threads without proper limits.
It uses an unsafe block to write directly into a Vec’s unallocated memory.
This unbounded allocation and unsafe pointer manipulation may lead to memory corruption
or exhaustion if the input “n” is too large.
*/
use std::env;
use std::thread::{self, JoinHandle};

pub fn allocate_resources(n: usize) -> Result<u32, String> {
    // Intentionally create an empty vector (capacity = 0)
    let mut handles: Vec<JoinHandle<u32>> = Vec::new();
    unsafe {
        // Using the raw pointer of the vector without pre-allocating capacity.
        // This will write to unallocated memory if n > 0.
        let ptr = handles.as_mut_ptr();
        for i in 0..n {
            // Spawn a thread that returns its index as u32.
            // The write below doesn't check if the pointer is valid for index i.
            ptr.add(i).write(thread::spawn(move || i as u32));
        }
        // Inform the vector that it now contains n elements.
        handles.set_len(n);
    }
    // Join all spawned threads and sum their outputs.
    let mut sum: u32 = 0;
    for handle in handles {
        sum = sum.checked_add(handle.join().map_err(|_| "thread join error".to_string())?)
            .ok_or_else(|| "integer overflow".to_string())?;
    }
    Ok(sum)
}

fn main() {
    // Read the number of threads to spawn from command line arguments.
    // If no argument is provided, default to 10.
    let args: Vec<String> = env::args().collect();
    let input: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(10)
    } else {
        10
    };
    match allocate_resources(input) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}