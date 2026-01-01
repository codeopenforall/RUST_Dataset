#[cfg(test)]
mod tests {
    // The test oracle validates that the final outcome is as expected.
    // For the corrected version, after 5 concurrent attempts to subtract 20 from an initial value of 50,
    // only the first two deductions succeed (50 -> 30 -> 10). Subsequent attempts do not change the state due to underflow prevention.
    // Thus, the final value must be exactly 10.
    use super::run_process;

    #[test]
    fn test_underflow() {
        // Execute the process and capture the final output.
        let result = run_process();
        // Assert that the final value is 10.
        assert_eq!(result, 10, "Test failed: Detected integer underflow vulnerability.");
    }
}