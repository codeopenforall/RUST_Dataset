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
        self.balance = self.balance.checked_add(amount).ok_or("integer overflow detected")?;
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
                if let Err(e) = account.add(300_000_000) {
                    panic!("{}", e);
                }
            }
        }));
    }
    for h in handles {
        if let Err(_) = h.join() {
            return Err("integer overflow detected");
        }
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
