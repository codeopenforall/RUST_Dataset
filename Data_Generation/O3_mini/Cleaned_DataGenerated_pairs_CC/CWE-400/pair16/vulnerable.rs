use std::thread;
use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
use std::env;
fn process(limit: u32) -> Result<u64, &'static str> {
    let total = Arc::new(AtomicU64::new(0));
    let mut handles = vec![];
    for i in 0..limit {
        let total = total.clone();
        unsafe {
            let handle = thread::spawn(move || {
                let mut local = i as u64;
                let ptr = &mut local as *mut u64;
                *ptr = *ptr + 1;
                total.fetch_add(*ptr, Ordering::Relaxed);
            });
            handles.push(handle);
        }
    }
    for h in handles {
        h.join().map_err(|_| "Thread join error")?;
    }
    Ok(total.load(Ordering::Relaxed))
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let limit: u32 = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        10  
    };
    match process(limit) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
