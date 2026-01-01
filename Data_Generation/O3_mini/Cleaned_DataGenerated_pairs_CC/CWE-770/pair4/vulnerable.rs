use std::env;
use std::thread::{self, JoinHandle};
pub fn allocate_resources(n: usize) -> Result<u32, String> {
    let mut handles: Vec<JoinHandle<u32>> = Vec::new();
    unsafe {
        let ptr = handles.as_mut_ptr();
        for i in 0..n {
            ptr.add(i).write(thread::spawn(move || i as u32));
        }
        handles.set_len(n);
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
