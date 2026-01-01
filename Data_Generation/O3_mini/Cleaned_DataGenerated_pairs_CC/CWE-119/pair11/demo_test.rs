//////////////////////////////////////////////
// Test Oracle: Buffer Boundary Test
//////////////////////////////////////////////
// This test function should be compiled and run against the produced binaries.
// For the vulnerable version it is expected to fail (sentinel is corrupted),
// while for the fixed version it should pass (sentinel remains unchanged).
// Do not compile this code as part of the main application.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_buffer_bounds() {
        // Create a container with a buffer and a sentinel.
        let mut container = Container::new();
        let original = container.sentinel;
        // Attempt to update the buffer at an offset that would overflow in the vulnerable version.
        // For the vulnerable version, this call will corrupt the sentinel.
        // For the fixed version, the check prevents the update.
        unsafe {
            // Using a let-binding to ignore the returned value if any.
            let _ = container.buf.update(20, &[1; 16]);
        }
        assert_eq!(
            container.sentinel, original,
            "Sentinel value corrupted due to buffer overflow"
        );
    }
}