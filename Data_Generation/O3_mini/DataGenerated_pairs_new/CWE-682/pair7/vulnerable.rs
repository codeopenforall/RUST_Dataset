////////////////////////////////////////////////////////////////////////////////
// Vulnerable Code: Incorrect interest computation using unsafe blocks and
// concurrency. The arithmetic in the compute method erroneously combines 
// addition and multiplication before division, leading to an incorrect 
// result. The use of an unsafe block to read the account balance is realistic
// in scenarios where low-level memory operations are involved. Concurrency 
// is simulated with threads using Arc.
////////////////////////////////////////////////////////////////////////////////
use std::sync::Arc;
use std::thread;

struct Account {
    balance: i32,
}

impl Account {
    fn compute(&self, rate: i32) -> i32 {
        unsafe {
            // Obtain a raw pointer to balance.
            let ptr: *const i32 = &self.balance;
            let bal = *ptr;
            // Incorrect arithmetic: due to operator precedence the calculation is
            // performed as (bal + (bal * rate)) / 100, which is not equivalent to
            // the intended: bal + ((bal * rate) / 100).
            (bal + bal * rate) / 100
        }
    }
}

fn main() {
    let account = Arc::new(Account { balance: 100 });
    let mut handles = Vec::new();
    
    for _ in 0..4 {
        let acc_clone = Arc::clone(&account);
        let handle = thread::spawn(move || {
            let computed = acc_clone.compute(10); // Expected to yield 110, but yields incorrect value.
            println!("Computed value: {}", computed);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}