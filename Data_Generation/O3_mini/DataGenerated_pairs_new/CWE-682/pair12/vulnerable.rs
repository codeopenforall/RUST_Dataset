//////////////////////////////////////////
// Vulnerable Version for Discount Calculation
// CWE-682: Incorrect Calculation (logic/math error)
//
// This executable mimics a real-world bug where an unsafe arithmetic expression
// incorrectly applies a discount due to operator precedence mistakes.
use std::sync::{Arc, Mutex};
use std::thread;

struct Order {
    price: u32,
    discount: u32, // discount percentage value (0-100)
}

impl Order {
    // Unsafe block is used to simulate pointer access as sometimes seen in FFI or low-level concurrency.
    unsafe fn total(&self) -> u32 {
        let discount_ptr: *const u32 = &self.discount;
        let discount_val = *discount_ptr;
        // Intended calculation: (price * (100 - discount)) / 100
        // BUG: Missing parentheses causes expression to be evaluated as:
        // (price * 100) - (discount the pointer value) / 100
        self.price * 100 - discount_val / 100
    }
}

fn main() {
    let order = Order { price: 200, discount: 15 };
    let shared_order = Arc::new(Mutex::new(order));
    let shared_order_clone = Arc::clone(&shared_order);

    let handle = thread::spawn(move || {
        let order_guard = shared_order_clone.lock().unwrap();
        let total = unsafe { order_guard.total() };
        println!("Total computed (vulnerable): {}", total);
    });

    handle.join().unwrap();
}