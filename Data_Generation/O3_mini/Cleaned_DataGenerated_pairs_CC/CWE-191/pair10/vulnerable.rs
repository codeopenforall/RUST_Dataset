use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
struct Ledger {
    value: UnsafeCell<u32>,
}
unsafe impl Sync for Ledger {}
impl Ledger {
    fn new(initial: u32) -> Self {
        Ledger {
            value: UnsafeCell::new(initial),
        }
    }
    fn withdraw(&self, amount: u32) {
        unsafe {
            let current = *self.value.get();
            let new = current - amount; 
            *self.value.get() = new;
        }
    }
    fn get(&self) -> u32 {
        unsafe { *self.value.get() }
    }
}
fn main() {
    let ledger = Arc::new(Ledger::new(100));
    let mut threads = Vec::new();
    {
        let ledger_clone = Arc::clone(&ledger);
        threads.push(thread::spawn(move || {
            ledger_clone.withdraw(150);
        }));
    }
    {
        let ledger_clone = Arc::clone(&ledger);
        threads.push(thread::spawn(move || {
            ledger_clone.withdraw(10);
        }));
    }
    for thr in threads {
        thr.join().unwrap();
    }
    println!("Final value: {}", ledger.get());
}
