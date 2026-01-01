//////////////// Test Oracle //////////////////
#[cfg(test)]
mod tests {
    use std::panic;
    // The following tests assume that the Executor struct in each code version is available.
    // For the vulnerable build, the injected input will be processed and execute extra commands,
    // whereas in the corrected build, the input validation will cause a panic.

    // To run these tests, compile the vulnerable and corrected versions separately.
    // When running the vulnerable build, this test should fail because it detects injected output.
    // When running the corrected build, it should pass (by panicking on unsafe input).

    #[test]
    fn test_injection() {
        let malicious_input = String::from("nonexistent; echo injected");
        // Catch a panic if one occurs.
        let result = panic::catch_unwind(|| {
            // Depending on the build (vulnerable vs. corrected), this may or may not panic.
            let exec = super::Executor::new(malicious_input);
            exec.run()
        });
        match result {
            // If no panic occurred, then we check that the output does not contain injected text.
            Ok(exec_result) => {
                match exec_result {
                    Ok(output) => {
                        // In the vulnerable version, "echo injected" will cause "injected" to appear.
                        assert!(
                            !output.contains("injected"),
                            "Test failed: command injection vulnerability exploited."
                        );
                    },
                    Err(err) => {
                        // Even if error, ensure it doesn't reflect injected content.
                        assert!(
                            !err.contains("injected"),
                            "Test failed: command injection vulnerability exploited in error message."
                        );
                    }
                }
            },
            // A panic is expected in the fixed version.
            Err(_) => {
                // The fixed version panics due to input validation, so this branch is considered safe.
            }
        }
    }
}