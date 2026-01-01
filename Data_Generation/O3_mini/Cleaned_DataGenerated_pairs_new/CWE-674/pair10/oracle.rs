#[cfg(test)]
mod oracle {
    use std::thread;
    use super::Parser;
    #[test]
    fn test_recursion_depth_limit() {
        let input = "(".repeat(600) + &")".repeat(600);
        let parser = Parser::new();
        let handle = thread::spawn(move || {
            parser.parse(&input)
        });
        match handle.join() {
            Ok(result) => {
                assert!(result.is_err(), "Expected an error due to recursion depth limit");
                let err = result.unwrap_err();
                assert!(err.contains("Recursion depth limit exceeded"), "Unexpected error message: {}", err);
            },
            Err(_) => panic!("Thread panicked: uncontrolled recursion vulnerability detected"),
        }
    }
}
