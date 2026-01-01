use std::sync::{Arc, Mutex};
use std::thread;
struct Account {
    balance: u32,
}
impl Account {
    fn new(init: u32) -> Self {
        Self { balance: init }
    }
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
    let acct = Arc::new(Mutex::new(Account::new(0)));
    let acct_clone = Arc::clone(&acct);
    let handle = thread::spawn(move || {
        let mut account = acct_clone.lock().unwrap();
        let _ = account.withdraw(1).map_err(|e| eprintln!("Error: {}", e));
    });
    handle.join().unwrap();
    let account = acct.lock().unwrap();
    println!("Balance: {}", account.get_balance());
}
