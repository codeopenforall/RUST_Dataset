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
    let target = 1_500_000_000;
    let final_total = simulate_transaction(target);
    println!("Final total: {}", final_total);
}
