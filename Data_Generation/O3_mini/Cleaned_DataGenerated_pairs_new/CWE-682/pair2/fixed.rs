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
fn compute_total(product: &Product) -> u32 {
    unsafe {
        let ptr: *const Product = product;
        let prod_ref: &Product = &*ptr;
        let result = (prod_ref.price - (prod_ref.price * prod_ref.discount_rate / 100)) * prod_ref.quantity;
        result
    }
}
fn run_app() -> u32 {
    let product = Arc::new(Product::new(100, 2, 15));
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
