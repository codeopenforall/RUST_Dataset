use std::sync::{Arc, Mutex};
use std::thread;

struct Account {
    balance: u32,
}

impl Account {
    // Safely subtracts the given amount after verifying that the deduction
    // does not exceed the current balance. Returns an error if underflow would occur.
    fn adjust(&mut self, amount: u32) -> Result<(), String> {
        if amount > self.balance {
            return Err(String::from("Underflow detected: deduction exceeds balance"));
        }
        self.balance -= amount;
        Ok(())
    }
}

fn main() {
    let account = Arc::new(Mutex::new(Account { balance: 10 }));
    let acc_clone = Arc::clone(&account);
    let handle = thread::spawn(move || {
        let mut acc = acc_clone.lock().unwrap();
        // If the deduction would cause an underflow, an error is produced and logged.
        if let Err(err) = acc.adjust(20) {
            eprintln!("Error: {}", err);
        }
    });
    handle.join().unwrap();
    let acc = account.lock().unwrap();
    println!("Balance: {}", acc.balance);
}