// Test Oracle Function (separate file/module)
#[cfg(test)]
mod tests {
    use super::run_app;

    #[test]
    fn test_total_computation() {
        // For the provided input (price=100, quantity=2, discount_rate=15) and 4 threads,
        // the correct aggregate total should be 680.
        let total = run_app();
        assert_eq!(total, 680, "The total computed does not match the expected value (680).");
    }
}