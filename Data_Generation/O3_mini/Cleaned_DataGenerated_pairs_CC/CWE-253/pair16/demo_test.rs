//////////////////////////////////////////////
// Test oracle for verifying the fix against CWE-253 vulnerability.
//
// This unit test calls the run method with a known safe input (e.g. 2, which is even).
// In the fixed version, input 2 should yield Ok(4). In the vulnerable version,
// the same input incorrectly results in an error. The test asserts that the processed
// value equates to doubling the input.
//////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Handler;
    use std::sync::Arc;

    #[test]
    fn test_oracle() {
        let handler = Arc::new(Handler::new());
        let res = unsafe { handler.run(2) };
        // Expected behavior: even input (2) produces success (Ok(4))
        assert!(res.is_ok(), "Expected Ok for safe input 2");
        assert_eq!(res.unwrap(), 4, "Processing 2 should yield 4");
    }
}