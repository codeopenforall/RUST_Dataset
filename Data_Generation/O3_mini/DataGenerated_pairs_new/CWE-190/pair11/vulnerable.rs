/////////////////////////////////////////////////////////////
// A real‐world simulation that mishandles integer arithmetic
// by performing unchecked (unsafe) addition in a concurrent context.
// CWE-190: Integer Overflow vulnerabilities. In this design, a shared
// ledger struct is updated concurrently using an unsafe block without
// proper overflow checking. This can lead to a wrapped-around balance.
/////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Ledger {
    balance: i32,
}

impl Ledger {
    fn new() -> Self {
        Ledger { balance: 0 }
    }

    // Performs an addition without checking for overflow.
    // Using an unsafe block to directly write to the balance pointer.
    fn add(&mut self, amount: i32) -> Result<(), &'static str> {
        unsafe {
            let ptr = &mut self.balance as *mut i32;
            // Vulnerability: unchecked arithmetic that can wrap around
            *ptr = *ptr + amount;
        }
        Ok(())
    }

    fn get_balance(&self) -> i32 {
        self.balance
    }
}

fn simulate_transactions() -> Result<i32, &'static str> {
    let ledger = Arc::new(Mutex::new(Ledger::new()));
    let mut handles = vec![];
    let iterations = 10;
    let threads = 10;
    for _ in 0..threads {
        let ledger = Arc::clone(&ledger);
        handles.push(thread::spawn(move || {
            for _ in 0..iterations {
                let mut account = ledger.lock().unwrap();
                // This large addition will trigger an arithmetic overflow
                // when summed across threads.
                account.add(300_000_000).unwrap();
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let final_balance = ledger.lock().unwrap().get_balance();
    Ok(final_balance)
}

fn main() {
    match simulate_transactions() {
        Ok(val) => println!("Final balance: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}