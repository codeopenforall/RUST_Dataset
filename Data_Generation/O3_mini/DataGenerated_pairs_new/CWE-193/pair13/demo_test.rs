//////////////////////////////////////////
// Test Oracle Function
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    // This test builds a buffer with 10 elements, fills it, and expects the sum to be exactly 55.
    // The vulnerable version will exhibit undefined behavior (and typically an incorrect sum or a crash),
    // while the corrected version should reliably produce the correct result.
    #[test]
    fn check_buffer_sum() {
        let expected: u32 = 55;
        let mut buf = Buffer::new(10);
        buf.populate();
        let total = buf.concurrent_sum();
        assert_eq!(total, expected, "The computed sum does not match the expected value.");
    }
}