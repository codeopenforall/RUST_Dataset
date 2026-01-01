/*
Test Oracle Function (to be compiled and run separately as part of the unit test suite)
This function verifies that the parser gracefully handles deeply nested input.
It spawns a thread to execute the parse function on malicious input.
For the vulnerable version, the thread is expected to panic (or not return an error in a controlled manner),
whereas for the corrected version, the parser returns an error indicating that the recursion depth limit was exceeded.
*/

#[cfg(test)]
mod oracle {
    use std::thread;
    use super::Parser;

    #[test]
    fn test_recursion_depth_limit() {
        // Create an input with extremely deep nesting
        let input = "(".repeat(600) + &")".repeat(600);
        let parser = Parser::new();

        // Spawn a thread to isolate potential panics
        let handle = thread::spawn(move || {
            parser.parse(&input)
        });

        // Join the thread and inspect the result.
        match handle.join() {
            // For the corrected version, the parser should return an Err with the expected message.
            Ok(result) => {
                assert!(result.is_err(), "Expected an error due to recursion depth limit");
                let err = result.unwrap_err();
                assert!(err.contains("Recursion depth limit exceeded"), "Unexpected error message: {}", err);
            },
            // For the vulnerable version, the thread would likely panic, causing the test to fail.
            Err(_) => panic!("Thread panicked: uncontrolled recursion vulnerability detected"),
        }
    }
}