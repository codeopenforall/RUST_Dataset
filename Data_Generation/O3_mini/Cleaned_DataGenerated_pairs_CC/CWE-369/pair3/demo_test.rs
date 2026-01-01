////////////////////////////////////////////
// Test Oracle
////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn division_zero_test() {
        // The function should safely handle a zero divisor by returning None without panicking.
        let result = std::panic::catch_unwind(|| {
            Arithmetic::compute(50, 0)
        });
        // For the fixed version, the operation should not panic.
        assert!(
            result.is_ok(),
            "The arithmetic operation panicked on division by zero"
        );
        let outcome = result.unwrap();
        // The expected behavior is to return None when a zero divisor is provided.
        assert!(
            outcome.is_none(),
            "Expected None when dividing by zero, but got a value"
        );
    }
}