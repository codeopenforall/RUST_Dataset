use std::sync::{Arc, Mutex};
use std::thread;

struct Purchase {
    price: u32,
    discount: u32,
    tax: u32,
}

impl Purchase {
    // This function calculates the total amount by subtracting a discount and adding tax.
    // It intentionally computes the tax incorrectly by performing integer division on the price first.
    // For example, for price 99, discount 20, tax 8:
    //   discount = 99 * 20 / 100 = 19,
    //   tax    = 99 / 100 * 8 = 0 * 8 = 0 (incorrect, expected 7),
    //   total  = 99 - 19 + 0 = 80.
    fn calculate(&self) -> u32 {
        unsafe {
            // Optimize by reading the price via a raw pointer.
            let price_ptr = &self.price as *const u32;
            let base = *price_ptr;
            let discount_val = base * self.discount / 100;
            // Vulnerability: Wrong ordering of operations causes premature truncation.
            let tax_val = base / 100 * self.tax; 
            base - discount_val + tax_val
        }
    }
}

fn main() {
    let orders = vec![
        Purchase { price: 99, discount: 20, tax: 8 }
    ];
    let shared_total = Arc::new(Mutex::new(0u32));
    let mut handles = vec![];

    for order in orders {
        let total_clone = Arc::clone(&shared_total);
        let o = order;
        let handle = thread::spawn(move || {
            let value = o.calculate();
            let mut total = total_clone.lock().unwrap();
            *total += value;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    let final_total = *shared_total.lock().unwrap();
    println!("Final Total: {}", final_total);
}