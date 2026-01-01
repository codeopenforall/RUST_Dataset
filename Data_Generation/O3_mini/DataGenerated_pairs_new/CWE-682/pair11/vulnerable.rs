/* 
   Complex vulnerable Rust code demonstrating incorrect arithmetic due to operator precedence errors.
   This example involves concurrent threads and unsafe pointer-like operations that mimic real-world issues.
*/

use std::sync::Arc;
use std::thread;

struct Transaction {
    amount: u32,
    discount: u32, // discount percentage (0-100)
}

impl Transaction {
    // Unsafe calculation with a precedence bug:
    // The intended formula is: net = amount - ((amount * discount) / 100)
    // Bug: Due to wrong grouping, it calculates as: amount - (amount * (discount / 100))
    // For discount < 100, (discount / 100) equals 0 so discount is effectively ignored.
    pub unsafe fn apply(&self) -> u32 {
        self.amount - self.amount * (self.discount / 100)
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
        println!("Net amount: {}", result); // In the vulnerable version, this prints 100 instead of 80.
    }
}