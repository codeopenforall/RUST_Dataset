#[cfg(test)]
mod tests {
    // For the vulnerable implementation, this test is expected to fail because compute() returns a corrupted value.
    // For the corrected implementation, compute() returns the correct value 42, and the test passes.
    use super::*;
    
    #[test]
    fn test_compute_value() {
        // For the vulnerable code, compute() is unsafe and may trigger undefined behavior.
        // The expected invariant is that compute() should return 42.
        let result = unsafe { compute() };
        assert_eq!(result, 42, "Test failed: expected 42 but got {}", result);
    }
}