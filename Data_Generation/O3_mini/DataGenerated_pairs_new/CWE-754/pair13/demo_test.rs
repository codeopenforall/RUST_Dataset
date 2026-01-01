#[cfg(test)]
mod tests {
    use super::Application;

    // This test calls the execute method expecting an error because the operation should time out.
    #[test]
    fn test_timeout_handling() {
        let app = Application;
        match app.execute() {
            Ok(val) => panic!("Expected error due to timeout, but got successful result: {}", val),
            Err(err) => assert_eq!(err, "Operation timed out", "Expected a timeout error"),
        }
    }
}