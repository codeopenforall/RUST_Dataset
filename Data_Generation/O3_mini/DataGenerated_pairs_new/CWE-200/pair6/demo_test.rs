#[cfg(test)]
mod tests {
    use super::get_debug;

    #[test]
    fn test_sensitive_output() {
        // For secure code, the debug output should not contain the actual secret.
        let output = get_debug();
        // The secret "super-secret-1234" should not appear in the output.
        assert!(
            !output.contains("super-secret-1234"),
            "Sensitive information was exposed in debug output!"
        );
    }
}