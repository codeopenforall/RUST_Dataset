/* 
   Complex fixed Rust code where the arithmetic is corrected by proper use of parentheses.
   This code still uses concurrency and unsafe constructs similarly to the vulnerable version,
   but the arithmetic error is resolved.
*/

use std::sync::Arc;
use std::thread;

struct Transaction {
    amount: u32,
    discount: u32, // discount percentage (0-100)
}

impl Transaction {
    // Corrected calculation: Group multiplication before division to correctly compute the discount.
    pub unsafe fn apply(&self) -> u32 {
        self.amount - ((self.amount * self.discount) / 100)
    }
}

fn main() {
    let transaction = Transaction { amount: 100, discount: 20 }; // Expected net amount: 80
    let shared = Arc::new(transaction);
    let mut handles = vec![];

    for _ in 0..5 {
        let clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            unsafe {
                clone.apply()
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.join().unwrap();
        println!("Net amount: {}", result); // Correctly prints 80 in the fixed version.
    }
}