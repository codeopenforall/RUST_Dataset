use std::sync::{Arc, Mutex};
    let product = Arc::new(Mutex::new(1u32));
        handles.push(thread::spawn(move || {
            let mut guard = product_clone.lock().unwrap();
            *guard = guard.checked_mul(i).ok_or("Overflow detected")?;
            Ok::<(), &'static str>(())
        h.join().unwrap()?;
    Ok(*product.lock().unwrap())
    match compute_factorial(13) {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
