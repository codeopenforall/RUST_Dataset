#[cfg(test)]
mod tests {
    use super::Engine;
    #[test]
    fn test_engine_error_handling() {
        let engine = Engine;
        // The run method is expected to return -1 when trigger is true if errors are handled gracefully.
        // In the vulnerable implementation this call would propagate a panic.
        let result = std::panic::catch_unwind(|| engine.run(true));
        match result {
            Ok(val) => {
                // For the corrected version, the error is caught and -1 is returned.
                assert_eq!(val, -1, "Engine run did not handle panic as expected");
            },
            Err(_) => {
                panic!("Test failed: function panicked instead of handling the error gracefully");
            }
        }
    }
}