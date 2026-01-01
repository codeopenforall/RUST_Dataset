#[cfg(test)]
mod tests {
    // Import the run_concurrent function from the outer scope.
    use super::*;

    #[test]
    fn test_concurrent() {
        // These parameters are chosen such that the expected final count is:
        // threads * increments = 4 * 10000 = 40000.
        let threads = 4;
        let increments = 10000;
        let result = run_concurrent(increments, threads);
        assert_eq!(result, threads as u64 * increments, 
            "Test failed: The counter did not reach the expected value. Likely a race condition is present.");
    }
}