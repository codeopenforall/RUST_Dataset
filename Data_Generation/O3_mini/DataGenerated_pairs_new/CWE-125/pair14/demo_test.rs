#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_oracle() {
        // The expected behavior of the safe (fixed) version is to return None when an out-of-bound index is used.
        // In the vulnerable version, unsafe access may return Some(_) or even trigger a panic.
        // This test will pass for the fixed code and fail for the vulnerable code.
        let result = panic::catch_unwind(|| {
            run_op()
        });
        // If the function panicked, the test fails.
        if let Err(_) = result {
            panic!("Test failed: function panicked");
        }
        let value = result.unwrap();
        assert!(value.is_none(), "Test failed: Expected None for out-of-bound access");
    }
}