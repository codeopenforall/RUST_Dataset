//////////////////////////////////////////////////////////////
// Test Oracle - This test should fail for the vulnerable version and pass for the secure version.
//////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Loader;

    // The test simulates a scenario where an insecure (relative) path is provided.
    // In the secure version, invoke() will reject relative paths, yielding an error.
    // In the vulnerable version, invoke() will simulate a hijacked binary and return "hijacked".
    #[test]
    fn external_loader_test() {
        // Use a clearly insecure relative path.
        let loader = Loader { path: "insecure".to_string() };
        let result = loader.invoke();

        // For the secure (fixed) version we expect an error.
        // For the vulnerable version, a valid (but malicious) output ("hijacked") is returned.
        // Thus, the test asserts that an error must be returned.
        assert!(result.is_err(), "Test failed: Insecure relative path should not be executed");
    }
}