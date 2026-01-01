//
// In the corrected version, the ledger's total is stored using u64 so that the multiplication result
// is not truncated. Additionally, the code uses checked arithmetic via checked_mul and checked_add,
// ensuring that any multiplication or addition overflow is detected and handled immediately.
//
#![allow(dead_code)]
use std::sync::{Arc, Mutex};
use std::thread;

struct Ledger {
    total: u64,
}

impl Ledger {
    fn new() -> Self {
        Ledger { total: 1000 }
    }

    // Safely adds funds by performing arithmetic in u64 using checked operations.
    fn add(&mut self, amount: u32) {
        let factor: u64 = 3;
        let calc = (amount as u64)
            .checked_mul(factor)
            .expect("Multiplication overflow");
        self.total = self.total
            .checked_add(calc)
            .expect("Addition overflow");
    }

    fn get_total(&self) -> u64 {
        self.total
    }
}

// Simulates a transaction concurrently using threads.
fn simulate_transaction(amount: u32) -> u64 {
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
    // Using the same high-value input, the checked operations guarantee correct arithmetic.
    let target = 1_500_000_000;
    let final_total = simulate_transaction(target);
    println!("Final total: {}", final_total);
}