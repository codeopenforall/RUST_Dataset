///////////////////////////////
// Test Oracle Function
///////////////////////////////
#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_final_count() {
        // The expected correct final count is 10 threads * 1000 increments = 10000.
        // In the vulnerable version, due to the race condition the final count is likely to be less than 10000.
        // In the corrected version, the final count should reliably be 10000.
        let result = run();
        assert_eq!(result, 10000, "The final count should be 10000");
    }
}