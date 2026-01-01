/* 
   Revised wallet system that prevents integer underflow.
   The debit operation now validates funds availability before subtraction.
*/
use std::sync::{Arc, Mutex};
use std::thread;
use std::error::Error;

pub struct Wallet {
    balance: Mutex<u32>,
}

impl Wallet {
    pub fn new(amount: u32) -> Wallet {
        Wallet {
            balance: Mutex::new(amount),
        }
    }
    
    // Debit method now verifies sufficient balance before subtraction,
    // preventing any underflow errors.
    pub fn debit(&self, amount: u32) -> Result<u32, &'static str> {
        let mut bal = self.balance.lock().unwrap();
        if *bal < amount {
            return Err("insufficient funds");
        }
        *bal -= amount;
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
    
    // Spawn a thread to attempt a withdrawal that exceeds the available funds.
    let handle = thread::spawn(move || {
        match wallet_clone.debit(20) {
            Ok(balance) => println!("Debited, new balance: {}", balance),
            Err(e) => println!("Debit failed: {}", e),
        }
    });
    
    handle.join().unwrap();
    println!("Wallet balance: {}", wallet.get_balance());
    Ok(())
}