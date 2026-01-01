#[cfg(test)]
mod tests {
    use super::*;
    
    // The test oracle verifies the differences between the two implementations.
    // For the vulnerable version, the unchecked subtraction causes an underflow leading to an incorrect final balance.
    // For the corrected version, the underflow is caught; thus only the valid subtraction (of 5) updates the balance.
    // This test asserts that the final balance is exactly 5.
    #[test]
    fn test_oracle() {
        let final_balance = run_app();
        assert_eq!(final_balance, 5, "Final balance should remain 5 after valid operations");
    }
}