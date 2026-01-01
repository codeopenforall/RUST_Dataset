#[cfg(test)]
mod tests {
    use super::*;
    // This test oracle function is used externally and must be run
    // separately from the vulnerable and fixed binaries.
    //
    // For a correct (fixed) implementation, the final counter value
    // should be exactly 8000. For the vulnerable implementation, due to
    // unsynchronized access, the final counter value is likely to be less.
    #[test]
    fn test_final_counter() {
        let result = run_app();
        assert_eq!(result, 8000, "Final counter value should be 8000, found {}", result);
    }
}