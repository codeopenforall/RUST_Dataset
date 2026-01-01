#[cfg(test)]
mod tests {
    // Import compute from the outer scope.
    use super::compute;

    #[test]
    fn test_computation() {
        // The expected result is the sum of indices 0 through 9.
        // 0 + 1 + 2 + ... + 9 = 45.
        let result = compute();
        // This assertion should fail with the vulnerable version due to potential memory corruption,
        // and pass with the corrected implementation.
        assert_eq!(result, 45, "The computed sum does not match the expected value");
    }
}