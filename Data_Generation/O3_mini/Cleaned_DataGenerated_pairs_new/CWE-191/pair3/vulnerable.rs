use std::sync::{Arc, Mutex};
use std::thread;
struct Account {
    balance: u32,
}
impl Account {
    fn new(init: u32) -> Self {
        Self { balance: init }
    }
    fn withdraw(&mut self, amount: u32) {
        unsafe {
            subtract_unchecked(&mut self.balance as *mut u32, amount);
        }
    }
    fn get_balance(&self) -> u32 {
        self.balance
    }
}
unsafe fn subtract_unchecked(val: *mut u32, sub: u32) {
    *val = (*val).wrapping_sub(sub);
}
fn main() {
    let acct = Arc::new(Mutex::new(Account::new(0)));
    let acct_clone = Arc::clone(&acct);
    let handle = thread::spawn(move || {
        let mut account = acct_clone.lock().unwrap();
        account.withdraw(1);
    });
    handle.join().unwrap();
    let account = acct.lock().unwrap();
    println!("Balance: {}", account.get_balance());
}
