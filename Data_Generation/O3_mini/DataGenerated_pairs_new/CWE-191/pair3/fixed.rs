///////////////////////////////////////////////////////////////
// Corrected version for integer underflow prevention (CWE-191)
// This code avoids underflow by checking the balance before subtracting.
// The safe implementation returns a Result, and no unsafe pointer arithmetic is used.
///////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Account {
    balance: u32,
}

impl Account {
    fn new(init: u32) -> Self {
        Self { balance: init }
    }

    // The withdrawal method checks for underflow prior to performing subtraction.
    fn withdraw(&mut self, amount: u32) -> Result<(), &'static str> {
        if self.balance < amount {
            return Err("Insufficient funds: withdrawal would underflow");
        }
        self.balance -= amount;
        Ok(())
    }

    fn get_balance(&self) -> u32 {
        self.balance
    }
}

fn main() {
    // Create shared account with initial balance of 0.
    let acct = Arc::new(Mutex::new(Account::new(0)));
    let acct_clone = Arc::clone(&acct);

    // Spawn a thread that attempts a withdrawal.
    let handle = thread::spawn(move || {
        let mut account = acct_clone.lock().unwrap();
        // The withdrawal returns an error if it would cause underflow.
        let _ = account.withdraw(1).map_err(|e| eprintln!("Error: {}", e));
    });
    handle.join().unwrap();

    let account = acct.lock().unwrap();
    println!("Balance: {}", account.get_balance());
}