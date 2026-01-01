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
    fn add(&mut self, amount: u32) {
        let factor: u32 = 3;
        unsafe {
            let calc = (amount as u64).wrapping_mul(factor as u64);
            let credit = calc as u32;
            self.total = self.total.wrapping_add(credit);
        }
    }
    fn get_total(&self) -> u32 {
        self.total
    }
}
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
    let target = 1_500_000_000; 
    let final_total = simulate_transaction(target);
    println!("Final total: {}", final_total);
}
