//////////////////////////////////////////////////////////////
// Test Oracle Function for Race Condition (CWE-362) Check  //
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // The oracle function is designed to be used with either the vulnerable or
    // the corrected code. It calls the run_test() function, expecting exactly 1
    // successful “check_then_set” invocation. For the vulnerable version, due to
    // the race condition, the test is expected to fail (i.e. returning a number
    // greater than 1). For the corrected version, it should pass.
    use super::run_test;

    #[test]
    fn test_oracle() {
        let successes = run_test();
        assert_eq!(successes, 1, "Race condition detected: expected exactly one successful access, but got {}.", successes);
    }
}