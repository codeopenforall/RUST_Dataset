use std::sync::{Arc, Mutex};
use std::thread;
struct Account {
    balance: u32,
}
impl Account {
    fn adjust(&mut self, amount: u32) {
        unsafe {
            let ptr: *mut u32 = &mut self.balance;
            *ptr = *ptr - amount;
        }
    }
}
fn main() {
    let account = Arc::new(Mutex::new(Account { balance: 10 }));
    let acc_clone = Arc::clone(&account);
    let handle = thread::spawn(move || {
        let mut acc = acc_clone.lock().unwrap();
        acc.adjust(20);
    });
    handle.join().unwrap();
    let acc = account.lock().unwrap();
    println!("Balance: {}", acc.balance);
}
