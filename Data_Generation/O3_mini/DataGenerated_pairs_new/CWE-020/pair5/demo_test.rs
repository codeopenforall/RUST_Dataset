#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_oracle() {
        // The test uses an input that should trigger an out-of-bounds access if unchecked.
        // "95" in hexadecimal equals 149, so 149 + 10 = 159, which is greater than 100.
        let handler = Handler::new();
        let malicious_input = "95";
        let result = panic::catch_unwind(|| handler.execute(malicious_input));

        // In the fixed code, the execute method returns an error ("index out of bounds").
        // In the vulnerable code, the unsafe copy may succeed (or invoke undefined behavior) leading to an Ok result, failing the test.
        match result {
            Ok(Ok(_)) => panic!("Test failed: Expected error on out-of-bounds input, but execution succeeded."),
            Ok(Err(e)) => assert_eq!(e, "index out of bounds", "Expected specific error message on out-of-bounds input."),
            Err(_) => panic!("Test failed: Execution panicked on out-of-bounds input."),
        }
    }
}