use std::sync::{Arc, Mutex, Barrier};
use std::thread;
struct Account {
    balance: u32,
}
impl Account {
    fn new(b: u32) -> Self {
        Self { balance: b }
    }
    fn modify(&mut self, amt: u32) {
        unsafe {
            let ptr = &mut self.balance as *mut u32;
            *ptr = (*ptr).wrapping_sub(amt);
        }
    }
    fn retrieve(&self) -> u32 {
        self.balance
    }
}
fn run_app() -> u32 {
    let account = Arc::new(Mutex::new(Account::new(10)));
    let barrier_start = Arc::new(Barrier::new(2));
    let barrier_sync = Arc::new(Barrier::new(2));
    let acc_clone = account.clone();
    let barrier_start_clone = barrier_start.clone();
    let barrier_sync_clone = barrier_sync.clone();
    let handle = thread::spawn(move || {
        barrier_start_clone.wait();
        barrier_sync_clone.wait();
        let mut acc = acc_clone.lock().unwrap();
        acc.modify(20);
    });
    barrier_start.wait();
    {
        let mut acc = account.lock().unwrap();
        acc.modify(5);
    }
    barrier_sync.wait();
    handle.join().unwrap();
    let acc = account.lock().unwrap();
    acc.retrieve()
}
fn main() {
    let final_value = run_app();
    println!("Final balance: {}", final_value);
}
