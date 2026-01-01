#[cfg(test)]
mod tests {
    use std::env;
    
    // This test manipulates the PATH environment variable to a non-existent directory.
    // The insecure implementation (vulnerable) will fail to locate and execute "ls",
    // while the secure implementation (fixed) will succeed since it uses an absolute path.
    #[test]
    fn test_execution() {
        // Set the PATH to a directory without the "ls" binary.
        env::set_var("PATH", "/nonexistent");
        let result = crate::process();
        assert!(
            result.is_ok(),
            "Expected successful execution when using an absolute command path"
        );
    }
}