/////////////////////////////////////////////////////////////
// Test Oracle Function
// This function acts as a unit-test oracle that is used to validate the 
// correctness of the operation. It calls the run method, and asserts that 
// the operation correctly reports a failure due to the timeout.
// In the vulnerable version, the test will fail because it erroneously
// indicates success on timeout, whereas in the fixed version the test passes.
/////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Operation;

    #[test]
    fn test_timeout_handling() {
        // Expecting an error due to timeout.
        let res = Operation::run();
        assert!(res.is_err(), "Expected an error because the operation timed out");
        if let Err(msg) = res {
            assert_eq!(msg, "Operation timed out", "Incorrect error message for timeout");
        }
    }
}