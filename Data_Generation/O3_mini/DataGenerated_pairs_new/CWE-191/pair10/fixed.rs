///////////////////////////////
// CWE-191 Corrected Code
///////////////////////////////
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

struct Ledger {
    value: UnsafeCell<u32>,
}

// Still manual Sync because we continue to use UnsafeCell.
unsafe impl Sync for Ledger {}

impl Ledger {
    fn new(initial: u32) -> Self {
        Ledger {
            value: UnsafeCell::new(initial),
        }
    }

    // Securely subtracts amount by using saturating subtraction to avoid underflow.
    fn withdraw(&self, amount: u32) {
        unsafe {
            let current = *self.value.get();
            // Using saturating_sub to ensure that the result does not underflow.
            *self.value.get() = current.saturating_sub(amount);
        }
    }

    fn get(&self) -> u32 {
        unsafe { *self.value.get() }
    }
}

fn main() {
    let ledger = Arc::new(Ledger::new(100));
    let mut threads = Vec::new();

    // Spawn a thread that attempts to withdraw beyond the current balance.
    {
        let ledger_clone = Arc::clone(&ledger);
        threads.push(thread::spawn(move || {
            ledger_clone.withdraw(150);
        }));
    }

    // Another concurrent thread for additional deduction.
    {
        let ledger_clone = Arc::clone(&ledger);
        threads.push(thread::spawn(move || {
            ledger_clone.withdraw(10);
        }));
    }

    for thr in threads {
        thr.join().unwrap();
    }
    // With saturating arithmetic, the balance is clamped at 0.
    println!("Final value: {}", ledger.get());
}