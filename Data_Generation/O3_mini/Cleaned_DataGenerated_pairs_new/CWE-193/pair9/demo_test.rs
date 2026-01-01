#[cfg(test)]
mod tests {
    use super::*;

    // Oracle test that should fail for the vulnerable version and pass for the corrected version.
    #[test]
    fn test_buffer_content() {
        // Create a buffer of expected size 5.
        let mut buf = DataBuffer::new(5);
        buf.fill(10);
        // For the vulnerable version, the extra uninitialized element (at index 5)
        // can lead to an unexpected value or panic. We check only the first 5 elements.
        for i in 0..5 {
            assert_eq!(buf.data[i], 10, "Element at index {} did not match expected value", i);
        }
        // Additionally, ensure the buffer length is exactly 5 in the fixed version.
        assert_eq!(buf.data.len(), 5, "Buffer length is not correct");
    }
}