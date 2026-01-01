//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::Record;
    // Import the serialize function from the module under test.
    use super::serialize;

    #[test]
    fn test_serialization_buffer_size() {
        // Create a test record with a 50-byte message.
        let rec = Record::new(42, "A".repeat(50));
        let buf = serialize(&rec);
        // The expected buffer length is: 4 bytes for u32 'id' + 50 bytes for message = 54 bytes.
        assert_eq!(
            buf.len(),
            54,
            "Buffer size incorrect. Expected 54, got {}. This test should fail for the vulnerable version and pass for the corrected version.",
            buf.len()
        );
    }
}