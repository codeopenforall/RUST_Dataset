//
// The test oracle function verifies that the simulation produces the expected total:
//  Final value = 1000 (initial total) + (1_500_000_000 * 3) = 4,500,001,000.
// For the vulnerable version the computed result will be incorrect due to overflow,
// whereas the fixed version produces the expected total.
//
fn test_oracle() {
    // Expected final total.
    let expected: u64 = 4_500_001_000;

    // Vulnerable simulation block:
    let res_vul = {
        use std::sync::{Arc, Mutex};
        use std::thread;
        struct Ledger {
            total: u32,
        }
        impl Ledger {
            fn new() -> Self {
                Ledger { total: 1000 }
            }
            fn add(&mut self, amount: u32) {
                let factor: u32 = 3;
                unsafe {
                    let calc = (amount as u64).wrapping_mul(factor as u64);
                    let credit = calc as u32;
                    self.total = self.total.wrapping_add(credit);
                }
            }
            fn get_total(&self) -> u32 {
                self.total
            }
        }
        fn simulate_transaction(amount: u32) -> u32 {
            let ledger = Arc::new(Mutex::new(Ledger::new()));
            let ledger_clone = Arc::clone(&ledger);
            let handle = thread::spawn(move || {
                let mut account = ledger_clone.lock().unwrap();
                account.add(amount);
            });
            handle.join().unwrap();
            let account = ledger.lock().unwrap();
            account.get_total()
        }
        simulate_transaction(1_500_000_000) as u64
    };

    // Fixed simulation block:
    let res_fix = {
        use std::sync::{Arc, Mutex};
        use std::thread;
        struct Ledger {
            total: u64,
        }
        impl Ledger {
            fn new() -> Self {
                Ledger { total: 1000 }
            }
            fn add(&mut self, amount: u32) {
                let factor: u64 = 3;
                let calc = (amount as u64)
                    .checked_mul(factor)
                    .expect("Multiplication overflow");
                self.total = self.total
                    .checked_add(calc)
                    .expect("Addition overflow");
            }
            fn get_total(&self) -> u64 {
                self.total
            }
        }
        fn simulate_transaction(amount: u32) -> u64 {
            let ledger = Arc::new(Mutex::new(Ledger::new()));
            let ledger_clone = Arc::clone(&ledger);
            let handle = thread::spawn(move || {
                let mut account = ledger_clone.lock().unwrap();
                account.add(amount);
            });
            handle.join().unwrap();
            let account = ledger.lock().unwrap();
            account.get_total()
        }
        simulate_transaction(1_500_000_000)
    };

    // Assertions: The vulnerable code should not meet the expected value while the fixed code must.
    assert!(res_vul != expected, "Vulnerable code produced the expected value unexpectedly.");
    assert_eq!(res_fix, expected, "Fixed code did not produce the expected total.");
}

#[cfg(test)]
mod tests {
    use super::test_oracle;
    #[test]
    fn oracle_test() { test_oracle(); }
}