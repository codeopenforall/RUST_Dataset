//////////////////////////////////////////////////////////////
// Test Oracle
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::execute_app;

    #[test]
    fn race_condition_oracle() {
        // This test expects the final counter to be exactly 10000.
        // For the vulnerable code, improper synchronization will likely result in a value less than 10000,
        // while the corrected version reliably returns 10000.
        let final_value = execute_app();
        assert_eq!(final_value, 10 * 1000, "Final counter mismatch: expected 10000, got {}", final_value);
    }
}