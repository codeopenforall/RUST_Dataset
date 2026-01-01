///////////////////////////////////////////////
// Test Oracle for verifying the API behavior
///////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle_test() {
        // For the vulnerable version, run() returns an incorrect sum due to UB
        // when passing an out-of-bound count. For the corrected version, it returns 10.
        let result = run();
        // Expected result derived from summing [1, 2, 3, 4]
        assert_eq!(result, 10, "Test failed: Buffer sum does not match expected value.");
    }
}