#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    // The test oracle verifies that attempting to withdraw more than the account's balance
    // is rejected. In the secure version, this operation returns an error.
    fn test_debit_underflow() {
        let wallet = Arc::new(Wallet::new(10));
        let result = wallet.debit(20);
        assert!(result.is_err(), "Operation should fail due to insufficient funds");
    }
}