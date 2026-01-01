///////////////////////////////
// Test Oracle Functionality
///////////////////////////////
//
// This test oracle is designed to be run against the compiled library/binary.
// It verifies that the Loader instance uses an absolute (secure) path.
// For the insecure (vulnerable) version the test will fail, whereas it passes for the corrected version.
#[cfg(test)]
mod tests {
    use super::Loader;

    #[test]
    fn test_command_path_is_absolute() {
        let loader = Loader::new();
        assert!(
            loader.is_secure(),
            "Test failed: The command path is not absolute, indicating an untrusted search path vulnerability."
        );
    }
}