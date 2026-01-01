#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_limit() {
        // This size is intentionally set to exceed the fixed allocation limit.
        let oversized = 10 * 1024 * 1024 + 1; // 10 MB + 1 byte
        let result = process_input(oversized);
        // The expected behavior for the fixed version is to return an error.
        // In the vulnerable version, this condition would not be detected.
        assert!(result.is_err(), "Expected an error for oversized allocation, but got OK");
    }
}