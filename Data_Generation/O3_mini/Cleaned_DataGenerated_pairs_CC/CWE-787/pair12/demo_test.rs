////////////////////////////////////////////////////////////////////////
// Test Oracle Function (for unit testing)
////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // In order to test both implementations with the same interface, assume that
    // the vulnerable version is compiled with the unsafe unchecked behavior and the
    // fixed version has the bounds check (by ensuring that Buffer::new initializes the
    // vector length properly). This oracle tests that an out-of-bound write is prevented.
    use super::*;
    use std::panic;

    #[test]
    fn test_out_of_bound_write() {
        // The triggering input: a 5-byte slice written at offset 8 in a 10-byte region.
        // In the vulnerable code, this would extend the buffer length to 13.
        // In the fixed code, a panic should occur to prevent corruption.
        let input = b"ABCDE";

        // Test behavior:
        // For vulnerable impl: the out-of-bound write corrupts the vector so that its length > 10.
        // For fixed impl: the attempt will panic, which we catch and treat as correct behavior.
        // We run the test block inside a catch_unwind to observe if a panic occurred.
        let result = panic::catch_unwind(|| {
            // Create a buffer with capacity/size 10.
            let mut buf = Buffer::new(10);
            // Use unsafe block to call write_data.
            unsafe {
                buf.write_data(8, input);
            }
            // Return the vector length to validate if corruption occurred.
            buf.data.len()
        });

        match result {
            Ok(len) => {
                // In vulnerable version, len would be 13 (8+5).
                // In fixed version, we should never get here for an out-of-bound write.
                assert!(
                    len <= 10,
                    "Test failure: Buffer write extended the length to {} (expected <= 10)", len
                );
            }
            Err(_) => {
                // A panic occurred.
                // For fixed implementation, panicking on an out-of-bound attempt is correct.
                // Signal success for the fixed version.
            }
        }
    }
}