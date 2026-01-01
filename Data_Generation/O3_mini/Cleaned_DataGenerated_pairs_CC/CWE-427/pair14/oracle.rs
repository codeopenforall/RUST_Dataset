#[cfg(test)]
mod tests {
    use std::env;
    use super::Runner;
    #[test]
    fn test_oracle() {
        env::set_var("MALICIOUS_PATH", "/nonexistent");
        let runner = Runner::new("echo", vec!["Hello".to_string()]);
        let result = unsafe {
            runner.execute()
        };
        match result {
            Ok(output) => assert_eq!(output.trim(), "Hello", "Expected command output 'Hello'"),
            Err(e) => panic!("Test failed: {}", e),
        }
    }
}
