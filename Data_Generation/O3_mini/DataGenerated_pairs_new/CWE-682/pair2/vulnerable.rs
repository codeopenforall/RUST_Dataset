//////////////////////////
// Vulnerable Code Start
//////////////////////////
use std::sync::Arc;
use std::thread;

struct Product {
    price: u32,
    quantity: u32,
    discount_rate: u32,
}

impl Product {
    fn new(price: u32, quantity: u32, discount_rate: u32) -> Self {
        Self {
            price,
            quantity,
            discount_rate,
        }
    }
}

// The calculation function uses an unsafe block to read from a raw pointer.
// The intended formula is: (price - (price * discount_rate / 100)) * quantity.
// However, due to a mistake in parenthesization the discount is computed as:
// price - ((price * discount_rate / 100) * quantity)
// which yields an incorrect result.
fn compute_total(product: &Product) -> u32 {
    unsafe {
        // Create a raw pointer and immediately dereference it.
        let ptr: *const Product = product;
        let prod_ref: &Product = &*ptr;
        // Vulnerability: Incorrect operator precedence leads to a math error.
        let result = prod_ref.price - ((prod_ref.price * prod_ref.discount_rate / 100) * prod_ref.quantity);
        result
    }
}

// This function spawns multiple threads that concurrently compute the product total.
// The threads all operate on the same Product instance via an Arc.
fn run_app() -> u32 {
    let product = Arc::new(Product::new(100, 2, 15)); // Test input: price=100, quantity=2, discount_rate=15
    let mut handles = vec![];
    for _ in 0..4 {
        let prod_clone = Arc::clone(&product);
        handles.push(thread::spawn(move || {
            compute_total(&*prod_clone)
        }));
    }
    let mut aggregate = 0;
    for handle in handles {
        aggregate += handle.join().unwrap();
    }
    aggregate
}

fn main() {
    let total = run_app();
    println!("Total: {}", total);
}
//////////////////////////
// Vulnerable Code End
//////////////////////////