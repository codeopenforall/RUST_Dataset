///////////////////////////////
// CWE-191 Demonstration Code
///////////////////////////////
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

struct Ledger {
    value: UnsafeCell<u32>,
}

// Manually asserting Sync because the inner UnsafeCell is unsynchronized.
unsafe impl Sync for Ledger {}

impl Ledger {
    fn new(initial: u32) -> Self {
        Ledger {
            value: UnsafeCell::new(initial),
        }
    }

    // Subtracts amount from the current value without checking for underflow.
    // This method uses an unsafe block to directly access the inner value.
    fn withdraw(&self, amount: u32) {
        unsafe {
            // Read current value unsafely.
            let current = *self.value.get();
            // Vulnerable arithmetic: if amount > current, underflow occurs.
            let new = current - amount; // CWE-191 vulnerability: Integer underflow.
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

    // Spawn a thread that withdraws more than the available amount to force underflow.
    {
        let ledger_clone = Arc::clone(&ledger);
        threads.push(thread::spawn(move || {
            ledger_clone.withdraw(150);
        }));
    }

    // Another thread that performs an additional deduction.
    {
        let ledger_clone = Arc::clone(&ledger);
        threads.push(thread::spawn(move || {
            ledger_clone.withdraw(10);
        }));
    }

    for thr in threads {
        thr.join().unwrap();
    }
    // In a debug build, the subtraction may panic if built with overflow checks.
    // In release mode, the underflow will wrap around.
    println!("Final value: {}", ledger.get());
}