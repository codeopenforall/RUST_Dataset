//////////////////////////////////////////
// Test Oracle Function for CWE-662 Example
// This single test invokes the run() function and checks that it
// completes without panicking. For the vulnerable version, the test
// will catch a panic (due to unhandled poison) and fail. For the fixed
// version, run() should complete normally, and the test passes.
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::run;
    use std::panic;

    #[test]
    fn test_oracle() {
        // The expected behavior is that run() completes without panicking.
        let result = panic::catch_unwind(|| {
            run();
        });
        assert!(result.is_ok(), "The run() function panicked unexpectedly");
    }
}