#[cfg(test)]
mod tests {
    use super::process_file;

    #[test]
    fn test_symlink_resolution() {
        // Test input simulating a malicious symbolic link resolution
        // that would traverse directories.
        let malicious_input = "../secret.txt";
        let result = process_file(malicious_input);
        // For a secure implementation, access should be denied,
        // hence an error must be returned.
        assert!(result.is_err(), "Access was improperly granted for a forbidden path.");
    }
}