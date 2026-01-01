use std::sync::{Arc, Mutex};
use std::thread;
fn compute_factorial(n: u32) -> Result<u32, &'static str> {
    let product = Arc::new(Mutex::new(1u32));
    let mut handles = Vec::new();
    for i in 1..=n {
        let product_clone = Arc::clone(&product);
        handles.push(thread::spawn(move || {
            let mut guard = product_clone.lock().unwrap();
            *guard = guard.checked_mul(i).ok_or("Overflow detected")?;
            Ok::<(), &'static str>(())
        }));
    }
    for h in handles {
        h.join().unwrap()?;
    }
    Ok(*product.lock().unwrap())
}
fn main() {
    match compute_factorial(13) {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}
