/*
This code demonstrates an unchecked resource allocation flaw. The API accepts a count and a size,
then spawns that many threads. Each thread unsafely allocates a Vec<u8> with the given size,
without any limits. An attacker may supply excessively large numbers leading to resource exhaustion (CWE-863).
*/
use std::thread;

struct ResourceManager;

impl ResourceManager {
    // This function processes requests by spawning count threads, each allocating a buffer of size bytes.
    pub fn process_request(count: usize, size: usize) -> Result<u8, &'static str> {
        // No checks on count and size, allowing unbounded resource allocation.
        let mut handles = Vec::new();
        let mut total = 0u8;
        for _ in 0..count {
            handles.push(thread::spawn(move || {
                unsafe {
                    // Allocate a vector with unchecked capacity and force its length.
                    let mut buffer: Vec<u8> = Vec::with_capacity(size);
                    buffer.set_len(size);
                    // Unsafely fill the vector without bounds checking.
                    for i in 0..size {
                        *buffer.get_unchecked_mut(i) = 42;
                    }
                    // Compute a wrapped sum of the contents.
                    buffer.iter().fold(0u8, |acc, &x| acc.wrapping_add(x))
                }
            }));
        }
        for handle in handles {
            // Join each thread. If any thread panicked, unwrap() will panic.
            total = total.wrapping_add(handle.join().unwrap());
        }
        Ok(total)
    }
}

fn main() {
    // Accept command line arguments: number of threads and allocation size.
    let args: Vec<String> = std::env::args().collect();
    let count: usize = args.get(1).unwrap_or(&"1000".to_string()).parse().unwrap();
    let size: usize = args.get(2).unwrap_or(&"1048576".to_string()).parse().unwrap();
    match ResourceManager::process_request(count, size) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}