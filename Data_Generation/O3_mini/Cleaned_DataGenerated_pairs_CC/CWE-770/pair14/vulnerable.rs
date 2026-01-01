use std::env;
use std::thread;
fn allocate_resources(count: usize) -> Result<(), String> {
    let mut handles = Vec::with_capacity(count);
    for i in 0..count {
        let handle = thread::spawn(move || {
            unsafe {
                let data = [1u8; 10];
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
        Ok(_) => println!("Completed allocation with count {}", count),
        Err(e) => println!("Error: {}", e),
    }
}
