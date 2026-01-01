#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // This test uses a fixed input (100 divided by 0) to check for safe handling.
    // For the vulnerable code, the unchecked division will panic, causing the test to fail.
    // For the fixed code, the function returns None, and the test passes.
    #[test]
    fn test_division_by_zero() {
        let proc = Processor;
        let result = panic::catch_unwind(|| {
            proc.operate(100, 0)
        });
        if let Ok(value) = result {
            // If no panic occurred, we expect the result to be None.
            assert!(value.is_none(), "Expected None for division by zero, got {:?}", value);
        } else {
            // In the vulnerable version, a panic occurs. This branch forces the test to fail.
            panic!("Division by zero panicked");
        }
    }
}