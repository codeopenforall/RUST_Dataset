///////////////////////////////////////////////////////////////
// Vulnerable version for integer underflow (CWE-191)
// This code uses unsafe pointer arithmetic to perform subtraction
// without checking for underflow. Concurrent threads access the
// shared Account instance through a Mutex. If a withdrawal of 1 is
// attempted on an account initialized to 0, the subtraction underflows,
// resulting in a wraparound value.
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

    // The withdrawal method subtracts without any check, using an unsafe helper.
    fn withdraw(&mut self, amount: u32) {
        unsafe {
            // Vulnerability: subtracting without checking for underflow.
            subtract_unchecked(&mut self.balance as *mut u32, amount);
        }
    }

    fn get_balance(&self) -> u32 {
        self.balance
    }
}

// Unsafe helper performing unchecked subtraction via raw pointer manipulation.
// This function does not check whether the subtraction will underflow.
unsafe fn subtract_unchecked(val: *mut u32, sub: u32) {
    // POTENTIAL FLAW: performing wrapping subtraction unsafely;
    // if *val < sub, this will wrap around.
    *val = (*val).wrapping_sub(sub);
}

fn main() {
    // Create shared account with initial balance of 0.
    let acct = Arc::new(Mutex::new(Account::new(0)));
    let acct_clone = Arc::clone(&acct);

    // Spawn a thread that performs a withdrawal.
    let handle = thread::spawn(move || {
        let mut account = acct_clone.lock().unwrap();
        account.withdraw(1);
    });
    handle.join().unwrap();

    let account = acct.lock().unwrap();
    println!("Balance: {}", account.get_balance());
}