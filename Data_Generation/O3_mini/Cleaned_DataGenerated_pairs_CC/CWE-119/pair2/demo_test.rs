//////////////////////////////////////////////
// Test Oracle: This function serves as an invariant check that distinguishes
// the vulnerable implementation from the corrected one.
// It creates a buffer of logical size 10 (with an extra sentinel byte at index 10).
// Before any operation, the sentinel (index 10) is expected to be 0.
// After invoking the fill method concurrently, the corrected version should leave
// the sentinel unchanged (0), while the vulnerable version would erroneously overwrite it (0xAA).
// The test asserts that the sentinel remains 0.
#[cfg(test)]
mod tests {
    use super::Buffer;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_sentinel_invariant() {
        // Create a buffer with logical size 10 (actual allocation: 11 bytes).
        let buffer = Arc::new(Buffer::new(10));
        // Check that sentinel (index 10) is initially 0.
        assert_eq!(buffer.get(10), 0, "Sentinel should be initially 0");

        let buffer_clone = Arc::clone(&buffer);
        let handle = thread::spawn(move || {
            unsafe { buffer_clone.fill(0xAA); }
        });
        unsafe { buffer.fill(0xAA); }
        handle.join().unwrap();

        // In the fixed version the sentinel remains unchanged.
        // In the vulnerable version, an off-by-one error corrupts it to 0xAA.
        assert_eq!(buffer.get(10), 0, "Sentinel was corrupted by out-of-bound write");
    }
}