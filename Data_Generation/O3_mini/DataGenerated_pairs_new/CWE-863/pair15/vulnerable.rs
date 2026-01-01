/*******************************************************************************
This code demonstrates a resource management flaw where unchecked user‐supplied
values can lead to unbounded thread spawning and excessive memory allocations.
An attacker can supply an arbitrarily large numeric input that causes the program
to spawn too many threads and attempt allocating huge buffers without limits,
potentially leading to a denial‐of‐service (CWE-863 style) situation.
*******************************************************************************/
use std::env;
use std::thread;
use std::vec::Vec;

struct Allocator;

impl Allocator {
    // This function accepts a user-provided value and spawns that many threads.
    // Each thread allocates a vector with size computed as (user_input * MULTIPLIER)
    // without any caps or domain checks, and uses an unsafe block to fill the vector.
    // If the input is huge, resource exhaustion may occur.
    pub fn process(&self, input: usize) -> Result<(), String> {
        // MULTIPLIER representing allocation size per thread.
        const MULTIPLIER: usize = 1024;
        let mut handles = Vec::with_capacity(input);

        // Spawn threads without enforcing limits:
        for _ in 0..input {
            // Calculate allocation size based on the user input.
            // Note: No check is done for multiplication overflow or resource exhaustion.
            let allocate_size = input * MULTIPLIER;
            let handle = thread::spawn(move || {
                // Unsafe block: force the allocation and write operations without bound checking.
                unsafe {
                    // Create a vector with the desired capacity.
                    let mut buffer = Vec::with_capacity(allocate_size);
                    // Force the vector's length without initialization.
                    buffer.set_len(allocate_size);
                    // Fill the vector with a constant value without bounds checking.
                    for i in 0..allocate_size {
                        // Using unchecked mutable access.
                        *buffer.get_unchecked_mut(i) = 42;
                    }
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete.
        for handle in handles {
            let _ = handle.join();
        }

        Ok(())
    }
}

fn main() {
    // Expect one argument: a positive integer.
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <positive_integer>", args[0]);
        std::process::exit(1);
    }
    let input = args[1].parse::<usize>().unwrap_or(0);
    if input == 0 {
        eprintln!("The number must be greater than zero.");
        std::process::exit(1);
    }

    let manager = Allocator;
    // In the vulnerable implementation, there is no throttling mechanism.
    if let Err(e) = manager.process(input) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}