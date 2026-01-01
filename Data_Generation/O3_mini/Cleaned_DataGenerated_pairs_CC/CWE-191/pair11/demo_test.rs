#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // The test oracle expects that using an input that would cause underflow (i.e., less than 5)
    // should result in an error. In the safe implementation, the error is returned.
    // In the vulnerable implementation, the unchecked arithmetic may lead to an unexpected result or a panic.
    #[test]
    fn test_no_underflow() {
        let data = Arc::new(Data::new());
        let engine = Engine::new(data);
        // Using an input that is too small; fixed version should return an error.
        match engine.execute(3) {
            Ok(val) => {
                panic!("Test failed: Expected error for input causing underflow, but got value {}", val);
            },
            Err(msg) => {
                assert_eq!(msg, "Input value too small, causes underflow", "Unexpected error message");
            },
        }
    }
}