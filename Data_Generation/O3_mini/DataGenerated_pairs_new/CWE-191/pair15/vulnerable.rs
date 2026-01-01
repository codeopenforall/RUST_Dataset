use std::sync::{Arc, Mutex};
use std::thread;

struct Account {
    balance: u32,
}

impl Account {
    // Performs a subtraction using an unsafe block without checking for underflow.
    // CWE-191: Integer Underflow occurs when subtracting a larger value than the balance,
    // which causes wrapping of the unsigned integer.
    fn adjust(&mut self, amount: u32) {
        unsafe {
            let ptr: *mut u32 = &mut self.balance;
            // Vulnerability: subtracting without checking may underflow the balance.
            *ptr = *ptr - amount;
        }
    }
}

fn main() {
    let account = Arc::new(Mutex::new(Account { balance: 10 }));
    let acc_clone = Arc::clone(&account);
    let handle = thread::spawn(move || {
        let mut acc = acc_clone.lock().unwrap();
        // Deduct an amount greater than the balance to trigger underflow
        acc.adjust(20);
    });
    handle.join().unwrap();
    let acc = account.lock().unwrap();
    println!("Balance: {}", acc.balance);
}