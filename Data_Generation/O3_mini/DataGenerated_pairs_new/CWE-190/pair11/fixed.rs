/////////////////////////////////////////////////////////////
// A corrected version of the simulation that avoids unchecked
// arithmetic by using checked addition. On detecting an overflow,
// the operation returns an error, ensuring that the ledger never
// holds an unexpected wrapped value.
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

    // Uses checked arithmetic to prevent overflow.
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
                // Attempting the large addition; if an overflow is detected,
                // the add() method returns an error immediately.
                if let Err(e) = account.add(300_000_000) {
                    // Propagate the error by panicking in the thread.
                    panic!("{}", e);
                }
            }
        }));
    }
    for h in handles {
        // If a thread panicked due to overflow detection, we catch it here.
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