////////////////////////////////////////////////////////////////////////////////////////////////////
// Test Oracle: This unit test verifies the behavior of the public process_input API.
// It uses catch_unwind to safely capture panics. For the vulnerable version, the assertion will
// trigger and cause a panic (test failure). For the fixed version, process_input returns an Err,
// indicating that the input was safely rejected, and no panic occurs.
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_process_input() {
        let result = panic::catch_unwind(|| process_input(150));
        match result {
            Ok(res) => {
                // In the fixed version, process_input does not panic and returns Err.
                assert!(res.is_err(), "Expected an error result for excessive input in safe version");
            },
            Err(_) => {
                panic!("process_input panicked, indicating a vulnerability");
            }
        }
    }
}