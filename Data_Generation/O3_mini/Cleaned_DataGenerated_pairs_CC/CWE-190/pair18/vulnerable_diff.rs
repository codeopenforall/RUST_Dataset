use std::cell::UnsafeCell;
use std::sync::Arc;
    let product = Arc::new(UnsafeCell::new(1u32));
        handles.push(thread::spawn(move || unsafe {
            let val = *product_clone.get();
            *product_clone.get() = val * i;
        h.join().unwrap();
    unsafe { Ok(*product.get()) }
    let result = compute_factorial(13).unwrap();
    println!("Result: {}", result);
