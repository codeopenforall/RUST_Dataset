//////////////////////
// Test Oracle
//////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    // The expected result is the sum of 100 (max_conn) and 443 (port), i.e., 543.
    // In the vulnerable code, max_conn is uninitialized which results in undefined behavior,
    // and the test is expected to fail. In the corrected version, the test should pass.
    #[test]
    fn test_compute() {
        let res = compute();
        assert_eq!(res, 543, "The computed value did not match the expected result.");
    }
}