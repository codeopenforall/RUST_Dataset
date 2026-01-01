////////////////////////////////////////////////////////////////////////////////
// Corrected Code: Proper interest computation using unsafe block and thread
// concurrency. Here, the arithmetic has been corrected to perform the multiplication
// and division before the addition, ensuring the correct order of operations.
////////////////////////////////////////////////////////////////////////////////
use std::sync::Arc;
use std::thread;

struct Account {
    balance: i32,
}

impl Account {
    fn compute(&self, rate: i32) -> i32 {
        unsafe {
            // Safely reading the balance via a raw pointer.
            let ptr: *const i32 = &self.balance;
            let bal = *ptr;
            // Correct arithmetic: first compute (bal * rate) / 100 then add the original balance.
            bal + (bal * rate) / 100
        }
    }
}

fn main() {
    let account = Arc::new(Account { balance: 100 });
    let mut handles = Vec::new();
    
    for _ in 0..4 {
        let acc_clone = Arc::clone(&account);
        let handle = thread::spawn(move || {
            let computed = acc_clone.compute(10); // Expected to yield the correct value of 110.
            println!("Computed value: {}", computed);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}