#[cfg(test)]
mod tests {
    use std::panic;
    // Import the run function from the crate under test.
    // When testing the vulnerable build, this test should fail as run() will panic.
    // For the fixed build, run() will execute without panicking.
    use super::run;

    #[test]
    fn test_oracle() {
        // Catch any panic from run(). In a correct (fixed) implementation, no panic occurs.
        let result = panic::catch_unwind(|| {
            run();
        });
        // The test asserts that no panic occurred; it will fail on the vulnerable version.
        assert!(
            result.is_ok(),
            "Expected normal completion without panic, but a panic occurred."
        );
    }
}