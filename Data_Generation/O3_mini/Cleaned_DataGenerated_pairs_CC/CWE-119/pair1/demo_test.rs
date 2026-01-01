#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_bounds() {
        let mut obj = Container::new(10);

        // In the fixed version, this out-of-bound operation should return an error.
        let res = obj.modify(0, 12, 42);
        assert!(res.is_err(), "Expected an error when attempting an out-of-bound write");

        // In both versions, a proper in-bound write should succeed.
        let res_valid = obj.modify(0, 10, 99);
        assert!(res_valid.is_ok(), "In-bound modification should succeed");

        // Verify that each byte in the buffer is correctly updated.
        for &byte in &obj.inner {
            assert_eq!(byte, 99, "Each element should have been updated to 99");
        }
    }
}