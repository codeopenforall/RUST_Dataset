#[cfg(test)]
mod tests {
    use super::*;
    
    // The test oracle verifies that relative path usage is correctly rejected.
    // When using a relative path, the secure (fixed) version will return -1,
    // while the vulnerable version erroneously returns 0.
    #[test]
    fn test_untrusted_path_behavior() {
        // Adjust the Loader input to a relative executable name.
        let loader = Loader { cmd: "insecure_app".to_string() };
        let result = loader.run();
        // In a secure implementation, relative paths should not be executed.
        // The oracle asserts that the result is -1.
        assert_eq!(result, -1, "Relative path must be rejected to mitigate untrusted search path risk");
    }
}