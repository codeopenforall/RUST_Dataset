/*
   Test oracle function that verifies the correctness of the transaction calculation.
   It creates a Transaction with amount=100 and discount=20; the expected net amount is 80.
   When run against the vulnerable version, the test fails (returns 100); with the fixed version, it passes.
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_calculation() {
        let trans = Transaction { amount: 100, discount: 20 };
        // Expected: 100 - ((100 * 20) / 100) = 80.
        let result = unsafe { trans.apply() };
        assert_eq!(result, 80, "Calculation error: expected 80, got {}", result);
    }
}