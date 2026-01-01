use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
const MAX_ALLOWED: usize = 1000;
fn main() {
    let args: Vec<String> = env::args().collect();
    let n: usize = args.get(1)
        .expect("Missing input parameter")
        .parse()
        .expect("Invalid number");
    match execute(n) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
fn execute(n: usize) -> Result<u64, &'static str> {
    if n > MAX_ALLOWED {
        return Err("Input too large, maximum allowed is 1000");
    }
    let total = Arc::new(AtomicU64::new(0));
    let mut handles = Vec::with_capacity(n);
    for i in 0..n {
        let total_clone = Arc::clone(&total);
        let h = thread::spawn(move || {
            total_clone.fetch_add(i as u64, Ordering::SeqCst);
        });
        handles.push(h);
    }
    for h in handles {
        h.join().map_err(|_| "Thread panicked")?;
    }
    Ok(total.load(Ordering::SeqCst))
}
