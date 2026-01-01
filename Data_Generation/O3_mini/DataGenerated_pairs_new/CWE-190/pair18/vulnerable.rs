/* 
   This Rust program computes the factorial of an input number concurrently.
   It stores the running product in a shared UnsafeCell<u32> wrapped in an Arc.
   Each thread performs an unchecked multiplication (via raw pointer dereference)
   that may overflow a u32 arithmetic range. For example, when the input is 13,
   the multiplication overflows, yielding an incorrect result.
*/
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

fn compute_factorial(n: u32) -> Result<u32, &'static str> {
    let product = Arc::new(UnsafeCell::new(1u32));
    let mut handles = Vec::new();
    for i in 1..=n {
        let product_clone = Arc::clone(&product);
        handles.push(thread::spawn(move || unsafe {
            let val = *product_clone.get();
            // Vulnerable multiplication: unchecked multiplication may overflow.
            *product_clone.get() = val * i;
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    unsafe { Ok(*product.get()) }
}

fn main() {
    // Using 13 to trigger u32 overflow (13! = 6227020800, overflows u32).
    let result = compute_factorial(13).unwrap();
    println!("Result: {}", result);
}