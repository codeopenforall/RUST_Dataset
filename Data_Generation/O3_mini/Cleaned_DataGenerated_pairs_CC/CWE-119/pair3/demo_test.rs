////////////////// Test Oracle //////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // This test will call operate with an out-of-bound index.
    // In the fixed code, update should panic because of the bound check.
    // In the vulnerable code, undefined behavior (likely no panic) may occur.
    // The test expects a panic; thus, it will pass for the corrected version and fail for the vulnerable one.
    #[test]
    fn test_out_of_bounds() {
        let instance = Arc::new(Mutex::new(Holder::new(10)));
        let result = panic::catch_unwind(|| {
            // Attempt an out-of-bound update
            operate(Arc::clone(&instance), 10, 99);
        });
        assert!(result.is_err(), "Expected panic on out-of-bound update");
    }
}