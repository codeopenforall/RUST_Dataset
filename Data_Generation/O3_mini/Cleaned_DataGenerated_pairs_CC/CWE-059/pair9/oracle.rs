#[cfg(test)]
mod tests {
    use super::process_file;
    #[test]
    fn test_symlink_resolution() {
        let malicious_input = "../secret.txt";
        let result = process_file(malicious_input);
        assert!(result.is_err(), "Access was improperly granted for a forbidden path.");
    }
}
