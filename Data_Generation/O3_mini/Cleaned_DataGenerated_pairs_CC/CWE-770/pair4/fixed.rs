use std::env;
use std::thread::{self, JoinHandle};
const MAX_ALLOWED: usize = 100;
pub fn allocate_resources(n: usize) -> Result<u32, String> {
    if n > MAX_ALLOWED {
        return Err(format!("Input exceeds the allowed maximum of {}", MAX_ALLOWED));
    }
    let mut handles: Vec<JoinHandle<u32>> = Vec::with_capacity(n);
    for i in 0..n {
        handles.push(thread::spawn(move || i as u32));
    }
    let mut sum: u32 = 0;
    for handle in handles {
        sum = sum.checked_add(handle.join().map_err(|_| "thread join error".to_string())?)
            .ok_or_else(|| "integer overflow".to_string())?;
    }
    Ok(sum)
}
fn main() {
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
