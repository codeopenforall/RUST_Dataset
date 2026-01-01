//////////////////////
// Vulnerability Version
//////////////////////
use std::process;

const LIMIT: usize = 100;

fn process_input(n: usize) -> Result<u64, &'static str> {
    let mut vec = Vec::with_capacity(n);
    // Unsafe: Manually setting the vector length without proper initialization.
    unsafe { vec.set_len(n); }
    for i in 0..n {
        vec[i] = 1;
    }
    // Resource exhaustion / unsafe memory write:
    // When n exceeds the limit, an out‐of‐bounds write is performed without throttling,
    // leading to undefined behavior and potential memory corruption.
    if n > LIMIT {
        unsafe {
            let ptr = vec.as_mut_ptr();
            // The valid indices are 0..n-1. Writing at position n is out-of-bound.
            *ptr.add(n) = 42; // <-- Vulnerability trigger (CWE-770)
        }
    }
    let sum: u64 = vec.iter().sum();
    Ok(sum)
}

fn main() {
    // Triggering input: 150 exceeds the allowed limit.
    let n: usize = 150;
    match process_input(n) {
        Ok(s) => println!("Sum: {}", s),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        },
    }
}