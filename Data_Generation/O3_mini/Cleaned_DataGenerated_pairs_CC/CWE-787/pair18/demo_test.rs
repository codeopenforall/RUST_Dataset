/*
   The following test oracle function verifies the integrity of the operation.
   It calls the process function and asserts that the buffer's aggregate sum remains 0.
   In the vulnerable version, an out-of-bounds write will modify memory and cause the sum
   to be 42, failing this test. In the corrected version, the update is rejected and the sum
   remains 0, causing the test to pass.
*/
#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_buffer_integrity() {
        let result = process();
        // The invariant is that no unintended update occurs; the sum must be 0.
        assert_eq!(result, 0, "Invariant violation: unexpected buffer modification detected");
    }
}