#[cfg(test)]
mod tests {
    // Import from the module where Buffer is defined.
    use super::*;

    #[test]
    fn test_out_of_bounds() {
        // Create a buffer of size 10.
        let mut buffer = Buffer::new(10);
        // Attempt to write to an out-of-bound index.
        // For the corrected code, this should return an error.
        let result = buffer.assign(10, 42);
        assert!(result.is_err(), "Buffer assignment with out-of-bounds index should fail.");
    }
}