#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // This test oracle is intended to evaluate the processing of the "admin" token.
    // For the correct implementation, processing "admin" must yield 1337.
    // In the flawed version, it would incorrectly return 0.
    #[test]
    fn test_authorization() {
        let processor = Processor;
        let result = processor.process("admin");
        assert_eq!(result, 1337, "Processing with an 'admin' token must yield 1337");
    }
}