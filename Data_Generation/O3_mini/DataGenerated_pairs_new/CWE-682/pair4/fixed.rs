use std::sync::{Arc, Mutex};
use std::thread;

struct Purchase {
    price: u32,
    discount: u32,
    tax: u32,
}

impl Purchase {
    // This function calculates the total amount by properly subtracting a discount and adding tax.
    // It corrects the tax computation by multiplying first then dividing.
    // For example, for price 99, discount 20, tax 8:
    //   discount = 99 * 20 / 100 = 19,
    //   tax    = 99 * 8 / 100 = 7,
    //   total  = 99 - 19 + 7 = 87.
    fn calculate(&self) -> u32 {
        unsafe {
            // Using a raw pointer for price as before.
            let price_ptr = &self.price as *const u32;
            let base = *price_ptr;
            let discount_val = base * self.discount / 100;
            // Correct tax calculation: multiply first, then divide to avoid premature truncation.
            let tax_val = base * self.tax / 100;
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