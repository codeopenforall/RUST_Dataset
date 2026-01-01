/* 
   Complex wallet system demonstrating CWE-191 (Integer Underflow)
   by improperly subtracting an amount greater than the available funds.
*/
use std::sync::{Arc, Mutex};
use std::thread;
use std::error::Error;

// Unsafe helper performing unchecked subtraction.
// Note: This function mimics an unsafe unchecked subtraction without proper validation.
unsafe fn unchecked_sub(lhs: u32, rhs: u32) -> u32 {
    // Using wrapping_sub to emulate unchecked arithmetic wraparound.
    lhs.wrapping_sub(rhs)
}

pub struct Wallet {
    balance: Mutex<u32>,
}

impl Wallet {
    pub fn new(amount: u32) -> Wallet {
        Wallet {
            balance: Mutex::new(amount),
        }
    }

    // Debit method that subtracts from the account without verifying 
    // that the funds suffice, leading to an integer underflow.
    pub fn debit(&self, amount: u32) -> Result<u32, &'static str> {
        let mut bal = self.balance.lock().unwrap();
        unsafe {
            *bal = unchecked_sub(*bal, amount);
        }
        Ok(*bal)
    }
    
    pub fn credit(&self, amount: u32) -> u32 {
        let mut bal = self.balance.lock().unwrap();
        *bal += amount;
        *bal
    }
    
    pub fn get_balance(&self) -> u32 {
        *self.balance.lock().unwrap()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let wallet = Arc::new(Wallet::new(10));
    let wallet_clone = Arc::clone(&wallet);
    
    // Spawn a thread that attempts to withdraw 20 from an account holding 10.
    let handle = thread::spawn(move || {
        // This operation will underflow the balance.
        wallet_clone.debit(20).unwrap();
    });
    
    handle.join().unwrap();
    println!("Wallet balance: {}", wallet.get_balance());
    Ok(())
}