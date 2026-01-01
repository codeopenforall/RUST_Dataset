//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::run_logic;

    #[test]
    fn test_concurrent_count() {
        // Expected final count: 10 threads * 1000 increments = 10000.
        let final_val = run_logic();
        assert_eq!(final_val, 10000, "The final count should be 10000.");
    }
}