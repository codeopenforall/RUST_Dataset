#[cfg(test)]
mod tests {
    use std::panic;

    // The test oracle ensures that the run_app function does NOT panic.
    // In the presence of the double free vulnerability, the vulnerable version will panic,
    // causing the test to fail. The corrected version, however, will run to completion.
    //
    // Note: The test is identical for both versions and will report a failure if a panic is caught.
    #[test]
    fn test_run_app_integrity() {
        let result = panic::catch_unwind(|| {
            // Depending on which version is compiled (vulnerable or corrected),
            // this call will either panic (vulnerable) or succeed (corrected).
            crate::run_app();
        });
        assert!(result.is_ok(), "Application panicked due to double free vulnerability");
    }
}