#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn contract_test() {
        let account = Arc::new(Mutex::new(Account { balance: 10 }));
        let acc_clone = Arc::clone(&account);
        let handle = thread::spawn(move || {
            let mut acc = acc_clone.lock().unwrap();
            // In the vulnerable version, this will underflow the balance,
            // whereas the safe version will refuse to deduct.
            // For the vulnerable variant, the final balance will differ from 10.
            // For the corrected variant, the balance remains 10.
            let _ = acc.adjust(20);
        });
        handle.join().unwrap();
        let acc = account.lock().unwrap();
        assert_eq!(acc.balance, 10, "Contract test failed: balance must remain 10");
    }
}