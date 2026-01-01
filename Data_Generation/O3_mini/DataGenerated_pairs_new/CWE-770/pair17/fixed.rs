//////////////////////
// Corrected Version
//////////////////////
use std::process;

const LIMIT: usize = 100;

fn process_input(n: usize) -> Result<u64, &'static str> {
    // Throttle the allocation by enforcing the input must be below the allowed maximum.
    if n > LIMIT {
        return Err("Input exceeds allowed limit");
    }
    let mut vec = Vec::with_capacity(n);
    // Initialize the vector properly.
    unsafe { vec.set_len(n); }
    for i in 0..n {
        vec[i] = 1;
    }
    // No unsafe out-of-bound operations are performed.
    let sum: u64 = vec.iter().sum();
    Ok(sum)
}

fn main() {
    // A safe input within limits.
    let n: usize = 50;
    match process_input(n) {
        Ok(s) => println!("Sum: {}", s),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        },
    }
}