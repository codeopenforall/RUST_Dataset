#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_test() {
        // When adding 1 to i32::MAX, a correct implementation must return an error,
        // while the vulnerable implementation erroneously returns a success value.
        let result = perform_calc(i32::MAX, 1);
        // The oracle expects an error to be returned.
        assert!(result.is_err(), "Expected error, but got {:?}", result);
    }
}