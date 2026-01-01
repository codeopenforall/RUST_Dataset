#[cfg(test)]
mod tests {
    // Import the execute function from the module under test.
    // When running the test on the vulnerable version, an input of 150 will trigger a panic.
    // For the fixed version, the error is returned gracefully without panicking.
    use super::execute;
    
    #[test]
    fn test_boundary() {
        let test_input = 150;
        let result = std::panic::catch_unwind(|| {
            // For the vulnerable code this will panic.
            // For the fixed code, this should return an error and not panic.
            execute(test_input)
        });
        assert!(
            result.is_ok(),
            "Execution panicked with an input equal or above the threshold"
        );
    }
}