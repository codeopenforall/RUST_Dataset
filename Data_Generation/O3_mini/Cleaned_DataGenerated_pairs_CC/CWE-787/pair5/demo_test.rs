////////////////////////////////////////////////////////////////
// Test Oracle: Ensuring the Correct Behavior Against OOB Access
////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    // This test function asserts the expected behavior for a properly checked update.
    // For the safe version, an out-of-bound update (index 10 on a 10-element buffer)
    // should be ignored, leaving the sum unchanged (i.e. 0), while an in-bound update
    // should correctly modify the buffer.
    #[test]
    fn test_process() {
        let mut buf = DataBuffer::new(10);
        // Call with an out-of-bound index. For the safe version, no update occurs.
        let r1 = buf.process(10, 42);
        // Now, call with a valid index.
        let r2 = buf.process(5, 13);
        // The sum after the first call should be 0 (since no valid element was updated).
        assert_eq!(r1, 0, "Out-of-bound update should not alter buffer sum");
        // After the valid update, the element at index 5 becomes 13.
        // As all other elements are 0, the total sum should be 13.
        assert_eq!(r2, 13, "In-bound update should result in a correct buffer sum");
    }
}