use std::sync::{Arc, Mutex};
use std::thread;
struct Ledger {
    balance: i32,
}
impl Ledger {
    fn new() -> Self {
        Ledger { balance: 0 }
    }
    fn add(&mut self, amount: i32) -> Result<(), &'static str> {
        unsafe {
            let ptr = &mut self.balance as *mut i32;
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
