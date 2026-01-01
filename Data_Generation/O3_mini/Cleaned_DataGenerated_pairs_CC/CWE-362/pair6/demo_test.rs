#[cfg(test)]
mod tests {
    // Import the 'run' function from the module under test.
    use super::*;

    #[test]
    fn test_final_count() {
        // The correct behavior is to have a final count of 10 threads * 1000 increments = 10000.
        // In the vulnerable code, due to the race condition, this invariant will often be violated.
        let final_count = run();
        assert_eq!(final_count, 10000, "Expected final count to be 10000, but got {}", final_count);
    }
}