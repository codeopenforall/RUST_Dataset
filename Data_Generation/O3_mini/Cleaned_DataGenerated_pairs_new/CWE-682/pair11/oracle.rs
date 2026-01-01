#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_transaction_calculation() {
        let trans = Transaction { amount: 100, discount: 20 };
        let result = unsafe { trans.apply() };
        assert_eq!(result, 80, "Calculation error: expected 80, got {}", result);
    }
}
