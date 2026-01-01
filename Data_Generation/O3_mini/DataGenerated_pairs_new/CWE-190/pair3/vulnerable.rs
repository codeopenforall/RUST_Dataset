//
// This Rust code simulates a ledger update where a deposit is multiplied by a constant factor.
// It uses unsafe blocks and concurrency, but the arithmetic operation is vulnerable to integer
// overflow (CWE-190) due to improper conversion from a u64 multiplication result to a u32 value.
//
#![allow(dead_code)]
use std::sync::{Arc, Mutex};
use std::thread;

struct Ledger {
    total: u32,
}

impl Ledger {
    fn new() -> Self {
        Ledger { total: 1000 }
    }

    // Adds funds by multiplying the deposit by 3.
    // The unsafe block performs the multiplication in u64 then forcibly casts
    // the product back to u32 without validating the value's range.
    fn add(&mut self, amount: u32) {
        let factor: u32 = 3;
        unsafe {
            // Multiplication in u64.
            let calc = (amount as u64).wrapping_mul(factor as u64);
            // Vulnerable: casting to u32 leads to truncation when calc > u32::MAX.
            let credit = calc as u32;
            self.total = self.total.wrapping_add(credit);
        }
    }

    fn get_total(&self) -> u32 {
        self.total
    }
}

// Spawns a thread to simulate a transaction concurrently.
fn simulate_transaction(amount: u32) -> u32 {
    let ledger = Arc::new(Mutex::new(Ledger::new()));
    let ledger_clone = Arc::clone(&ledger);
    let handle = thread::spawn(move || {
        let mut account = ledger_clone.lock().unwrap();
        account.add(amount);
    });
    handle.join().unwrap();
    let account = ledger.lock().unwrap();
    account.get_total()
}

fn main() {
    // Using an input value that causes the multiplication to exceed u32 capacity.
    let target = 1_500_000_000; // 1.5 billion
    let final_total = simulate_transaction(target);
    println!("Final total: {}", final_total);
}