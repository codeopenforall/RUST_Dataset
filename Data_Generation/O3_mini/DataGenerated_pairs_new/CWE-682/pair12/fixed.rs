//////////////////////////////////////////
// Fixed Version for Discount Calculation
// CWE-682: Corrected Calculation Logic
//
// This executable fixes the arithmetic error, correctly computing the total by
// applying the discount as: (price * (100 - discount)) / 100.
// Unsafe block is still used for demonstration purposes.
use std::sync::{Arc, Mutex};
use std::thread;

struct Order {
    price: u32,
    discount: u32, // discount percentage value (0-100)
}

impl Order {
    unsafe fn total(&self) -> u32 {
        let discount_ptr: *const u32 = &self.discount;
        let discount_val = *discount_ptr;
        // Correct computation with proper parentheses:
        // (price * (100 - discount)) / 100
        (self.price * (100 - discount_val)) / 100
    }
}

fn main() {
    let order = Order { price: 200, discount: 15 };
    let shared_order = Arc::new(Mutex::new(order));
    let shared_order_clone = Arc::clone(&shared_order);

    let handle = thread::spawn(move || {
        let order_guard = shared_order_clone.lock().unwrap();
        let total = unsafe { order_guard.total() };
        println!("Total computed (fixed): {}", total);
    });

    handle.join().unwrap();
}